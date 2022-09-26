// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Direct distribution of statements, even those concerning candidates which
//! are not yet backed.
//!
//! Members of a validation group assigned to a para at a given relay-parent
//! always distribute statements directly to each other.
//!
//! The main way we limit the amount of candidates that have to be handled by
//! the system is to limit the amount of `Seconded` messages that we allow
//! each validator to issue at each relay-parent. Since the amount of relay-parents
//! that we have to deal with at any time is itself bounded, this lets us bound
//! the memory and work that we have here. Bounding `Seconded` statements is enough
//! because they imply a bounded amount of `Valid` statements about the same candidate
//! which may follow.
//!
//! The motivation for this piece of code is that the statements that each validator
//! sees may differ. i.e. even though a validator is allowed to issue X `Seconded`
//! statements at a relay-parent, they may in fact issue X*2 and issue one set to
//! one partition of the backing group and one set to another. Of course, in practice
//! these types of partitions will not exist, but in the worst case each validator in the
//! group would see an entirely different set of X `Seconded` statements from some validator
//! and each validator is in its own partition. After that partition resolves, we'd have to
//! deal with up to `limit*group_size` `Seconded` statements from that validator. And then
//! if every validator in the group does the same thing, we're dealing with something like
//! `limit*group_size^2` `Seconded` statements in total.
//!
//! Given that both our group sizes and our limits per relay-parent are small, this is
//! quite manageable, and the utility here lets us deal with it in only a few kilobytes
//! of memory.
//!
//! It's also worth noting that any case where a validator issues more than the legal limit
//! of `Seconded` statements at a relay parent is trivially slashable on-chain, which means
//! the 'worst case' adversary that this code defends against is effectively lighting money
//! on fire. Nevertheless, we handle the case here to ensure that the behavior of the
//! system is well-defined even if an adversary is willing to be slashed.
//!
//! More concretely, this module exposes a "DirectInGroup" utility which allows us to determine
//! whether to accept or reject messages from other validators in the same group as we
//! are in, based on _the most charitable possible interpretation of our protocol rules_,
//! and to keep track of what we have sent to other validators in the group and what we may
//! continue to send them.
// TODO [now]: decide if we want to also distribute statements to validators
// that are assigned as-of an active leaf i.e. the next group.

use std::ops::Range;

use polkadot_primitives::vstaging::{CandidateHash, CompactStatement, ValidatorIndex};

use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq)]
struct ValidStatementManifest {
	remote: ValidatorIndex,
	originator: ValidatorIndex,
	candidate_hash: CandidateHash,
}

// A piece of knowledge about a candidate
#[derive(Hash, Clone, PartialEq, Eq)]
enum Knowledge {
	// General knowledge.
	General(CandidateHash),
	// Specific knowledge of a given statement (with its originator)
	Specific(CompactStatement, ValidatorIndex),
}

// Knowledge paired with its source.
#[derive(Hash, Clone, PartialEq, Eq)]
enum TaggedKnowledge {
	// Knowledge we have received from the validator on the p2p layer.
	IncomingP2P(Knowledge),
	// Knowledge we have sent to the validator on the p2p layer.
	OutgoingP2P(Knowledge),
	// Knowledge of candidates the validator has seconded.
	Seconded(CandidateHash),
}

/// Utility for keeping track of limits on direct statements within a group.
///
/// See module docs for more details.
pub struct DirectInGroup {
	validators: Vec<ValidatorIndex>,
	seconding_limit: usize,

	knowledge: HashMap<ValidatorIndex, HashSet<TaggedKnowledge>>,
}

impl DirectInGroup {
	/// Instantiate a new `DirectInGroup` tracker. Fails if `group_validators` is empty
	pub fn new(
		group_validators: Vec<ValidatorIndex>,
		seconding_limit: usize,
	) -> Option<Self> {
		if group_validators.is_empty() {
			return None
		}
		Some(DirectInGroup {
			validators: group_validators,
			seconding_limit,
			knowledge: HashMap::new(),
		})
	}

	/// Query whether we can receive some statement from the given validator.
	///
	/// This does no deduplication of `Valid` statements.
	pub fn can_receive(
		&self,
		sender: ValidatorIndex,
		originator: ValidatorIndex,
		statement: CompactStatement,
	) -> Result<Accept, RejectIncoming> {
		if !self.is_in_group(sender) || !self.is_in_group(originator) {
			return Err(RejectIncoming::NotInGroup)
		}

		if self.they_sent(sender, Knowledge::Specific(statement.clone(), originator)) {
			return Err(RejectIncoming::Duplicate)
		}

		match statement {
			CompactStatement::Seconded(candidate_hash) => {
				// check whether the sender has not sent too many seconded statements for the originator.
				// we know by the duplicate check above that this iterator doesn't include the
				// statement itself.
				let other_seconded_for_orig_from_remote = self
					.knowledge
					.get(&sender)
					.into_iter()
					.flat_map(|v_knowledge| v_knowledge.iter())
					.filter(|k| match k {
						TaggedKnowledge::IncomingP2P(Knowledge::Specific(
							CompactStatement::Seconded(_),
							orig,
						)) if orig == &originator => true,
						_ => false,
					})
					.count();

				if other_seconded_for_orig_from_remote == self.seconding_limit {
					return Err(RejectIncoming::ExcessiveSeconded)
				}

				// at this point, it doesn't seem like the remote has done anything wrong.
				if self.seconded_already_or_within_limit(originator, candidate_hash) {
					Ok(Accept::Ok)
				} else {
					Ok(Accept::WithPrejudice)
				}
			},
			CompactStatement::Valid(candidate_hash) => {
				if !self.knows_candidate(sender, candidate_hash) {
					return Err(RejectIncoming::CandidateUnknown)
				}

				Ok(Accept::Ok)
			},
		}
	}

	/// Note that we accepted an incoming statement. This updates internal structures.
	/// Should only be called after a successful `can_receive` call.
	pub fn note_received(
		&mut self,
		sender: ValidatorIndex,
		originator: ValidatorIndex,
		statement: CompactStatement,
	) {
		{
			let mut sender_knowledge = self.knowledge.entry(sender).or_default();
			sender_knowledge.insert(TaggedKnowledge::IncomingP2P(
				Knowledge::Specific(statement.clone(), originator)
			));

			if let CompactStatement::Seconded(candidate_hash) = statement.clone() {
				sender_knowledge.insert(TaggedKnowledge::IncomingP2P(
					Knowledge::General(candidate_hash)
				));
			}
		}

		if let CompactStatement::Seconded(candidate_hash) = statement {
			let mut originator_knowledge = self.knowledge.entry(originator).or_default();
			originator_knowledge.insert(TaggedKnowledge::Seconded(candidate_hash));
		}
	}

	/// Query whether we can send a statement to a given validator.
	pub fn can_send(
		&self,
		target: ValidatorIndex,
		originator: ValidatorIndex,
		statement: CompactStatement,
	) -> Result<(), RejectOutgoing> {
		if !self.is_in_group(target) || !self.is_in_group(originator) {
			return Err(RejectOutgoing::NotInGroup)
		}

		if self.we_sent(target, Knowledge::Specific(statement.clone(), originator)) {
			return Err(RejectOutgoing::Known)
		}

		if self.they_sent(target, Knowledge::Specific(statement.clone(), originator)) {
			return Err(RejectOutgoing::Known)
		}

		match statement {
			CompactStatement::Seconded(candidate_hash) => {
				// we send the same `Seconded` statements to all our peers, and only the first `k` from
				// each originator.
				if !self.seconded_already_or_within_limit(originator, candidate_hash) {
					return Err(RejectOutgoing::ExcessiveSeconded)
				}

				Ok(())
			},
			CompactStatement::Valid(candidate_hash) => {
				if !self.knows_candidate(target, candidate_hash) {
					return Err(RejectOutgoing::CandidateUnknown)
				}

				Ok(())
			},
		}
	}

	/// Note that we sent an outgoing statement to a peer in the group.
	/// This must be preceded by a successful `can_send` call.
	pub fn note_sent(
		&mut self,
		target: ValidatorIndex,
		originator: ValidatorIndex,
		statement: CompactStatement,
	) {
		{
			let mut target_knowledge = self.knowledge.entry(target).or_default();
			target_knowledge.insert(TaggedKnowledge::OutgoingP2P(
				Knowledge::Specific(statement.clone(), originator)
			));

			if let CompactStatement::Seconded(candidate_hash) = statement.clone() {
				target_knowledge.insert(TaggedKnowledge::OutgoingP2P(
					Knowledge::General(candidate_hash)
				));
			}
		}

		if let CompactStatement::Seconded(candidate_hash) = statement {
			let mut originator_knowledge = self.knowledge.entry(originator).or_default();
			originator_knowledge.insert(TaggedKnowledge::Seconded(candidate_hash));
		}
	}

	// returns true if it's legal to accept a new `Seconded` message from this validator.
	// This is either
	//   1. because we've already accepted it.
	//   2. because there's space for more seconding.
	fn seconded_already_or_within_limit(
		&self,
		validator: ValidatorIndex,
		candidate_hash: CandidateHash,
	) -> bool {
		let seconded_other_candidates = self
			.knowledge
			.get(&validator)
			.into_iter()
			.flat_map(|v_knowledge| v_knowledge.iter())
			.filter(|k| match k {
				TaggedKnowledge::Seconded(c) if c != &candidate_hash => true,
				_ => false,
			})
			.count();

		// This fulfills both properties by under-counting when the validator is at the limit
		// but _has_ seconded the candidate already.
		seconded_other_candidates < self.seconding_limit
	}

	fn they_sent(&self, validator: ValidatorIndex, knowledge: Knowledge) -> bool {
		self.knowledge
			.get(&validator)
			.map_or(false, |k| k.contains(&TaggedKnowledge::IncomingP2P(knowledge)))
	}

	fn we_sent(&self, validator: ValidatorIndex, knowledge: Knowledge) -> bool {
		self.knowledge
			.get(&validator)
			.map_or(false, |k| k.contains(&TaggedKnowledge::OutgoingP2P(knowledge)))
	}

	fn knows_candidate(&self, validator: ValidatorIndex, candidate_hash: CandidateHash) -> bool {
		self.we_sent_seconded(validator, candidate_hash) ||
			self.they_sent_seconded(validator, candidate_hash)
	}

	fn we_sent_seconded(&self, validator: ValidatorIndex, candidate_hash: CandidateHash) -> bool {
		self.we_sent(validator, Knowledge::General(candidate_hash))
	}

	fn they_sent_seconded(&self, validator: ValidatorIndex, candidate_hash: CandidateHash) -> bool {
		self.they_sent(validator, Knowledge::General(candidate_hash))
	}

	fn is_in_group(&self, validator: ValidatorIndex) -> bool {
		self.validators.contains(&validator)
	}
}

/// Incoming statement was accepted.
#[derive(Debug, PartialEq)]
pub enum Accept {
	/// Neither the peer nor the originator have apparently exceeded limits.
	/// Candidate or statement may already be known.
	Ok,
	/// Accept the message; the peer hasn't exceeded limits but the originator has.
	WithPrejudice,
}

/// Incoming statement was rejected.
#[derive(Debug, PartialEq)]
pub enum RejectIncoming {
	/// Peer sent excessive `Seconded` statements.
	ExcessiveSeconded,
	/// Sender or originator is not in the group.
	NotInGroup,
	/// Candidate is unknown to us. Only applies to `Valid` statements.
	CandidateUnknown,
	/// Statement is duplicate.
	Duplicate,
}

/// Outgoing statement was rejected.
#[derive(Debug, PartialEq)]
pub enum RejectOutgoing {
	/// Candidate was unknown. ONly applies to `Valid` statements.
	CandidateUnknown,
	/// We attempted to send excessive `Seconded` statements.
	/// indicates a bug on the local node's code.
	ExcessiveSeconded,
	/// The statement was already known to the peer.
	Known,
	/// Target or originator not in the group.
	NotInGroup,
}
