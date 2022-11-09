window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic will not be allowed to consume more than `AvailableBlockRatio - 1%`."],["BABE_GENESIS_EPOCH_CONFIG","The BABE epoch configuration at genesis."],["MAXIMUM_BLOCK_WEIGHT","The storage proof size is not limited so far."],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by  Operational  extrinsics."],["VERSION","Runtime version (Westend)."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["EPMCall","Contains one variant per dispatchable that can be called by an extrinsic."],["OriginCaller",""],["ProxyType","The type used to represent the kinds of proxying allowed."],["RuntimeCall",""],["RuntimeEvent",""],["StakerStatus","Indicates the initial status of the staker."],["SystemCall","Contains one variant per dispatchable that can be called by an extrinsic."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["native_version","Native version."]],"macro":[["impl_elections_weights","Implements the weight types for the elections module and a specific runtime. This macro should not be called directly; use [`impl_runtime_weights`] instead."]],"mod":[["api",""],["private",""],["xcm_config","XCM configurations for Westend."]],"struct":[["AnnouncementDepositBase",""],["AnnouncementDepositFactor",""],["BagThresholds",""],["BasicDeposit",""],["BetterUnsignedThreshold",""],["BlockExecutionWeight","Time to execute an empty block. Calculated by multiplying the Average with `1` and adding `0`."],["BlockWeights","Block weights base values and limits."],["BondingDuration",""],["ConfigDepositBase",""],["CrowdloanId",""],["DataDepositPerByte",""],["DepositBase",""],["DepositFactor",""],["EndingPeriod",""],["EpochDuration",""],["ExistentialDeposit",""],["ExpectedBlockTime",""],["ExtrinsicBaseWeight","Time to execute a NO-OP extrinsic, for example `System::remark`. Calculated by multiplying the Average with `1` and adding `0`."],["FieldDeposit",""],["FirstMessageFactorPercent",""],["FriendDepositFactor",""],["GenesisConfig",""],["ImOnlineUnsignedPriority",""],["IndexDeposit",""],["LeasePeriod",""],["MaxActiveValidators",""],["MaxAdditionalFields",""],["MaxAuthorities",""],["MaxElectableTargets","… and all of the validators as electable targets. Whilst this is the case, we cannot and shall not increase the size of the validator intentions."],["MaxElectingVoters","We take the top 22500 nominators as electing voters.."],["MaxFriends",""],["MaxKeys",""],["MaxLocks",""],["MaxMemoLength",""],["MaxNominations",""],["MaxNominatorRewardedPerValidator",""],["MaxPeerDataEncodingSize",""],["MaxPeerInHeartbeats",""],["MaxPending",""],["MaxPermanentSlots",""],["MaxPointsToBalance",""],["MaxProxies",""],["MaxRegistrars",""],["MaxReserves",""],["MaxScheduledPerBlock",""],["MaxSignatories",""],["MaxSubAccounts",""],["MaxTemporarySlotPerLeasePeriod",""],["MaxTemporarySlots",""],["MaximumSchedulerWeight",""],["MinContribution",""],["MinVestedTransfer",""],["MinimumPeriod",""],["NoPreimagePostponement",""],["NposCompactSolution16",""],["NposSolutionPriority",""],["OffchainRepeat",""],["OffchainSolutionLengthLimit","A limit for off-chain phragmen unsigned solution length."],["OffchainSolutionWeightLimit","A limit for off-chain phragmen unsigned solution submission."],["OffendingValidatorsThreshold",""],["Offset",""],["OnChainSeqPhragmen",""],["OperationalFeeMultiplier","This value increases the priority of `Operational` transactions by adding a “virtual tip” that’s equal to the `OperationalFeeMultiplier * final_fee`."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["ParaDeposit",""],["ParasUnsignedPriority",""],["ParityDbWeight","`ParityDB` can be enabled with a feature flag, but is still experimental. These weights are available for brave runtime engineers who may want to try this out as default."],["Period",""],["PermanentSlotLeasePeriodLength",""],["PoolsPalletId",""],["PreimageBaseDeposit",""],["PreimageByteDeposit",""],["PreimageMaxSize",""],["ProxyDepositBase",""],["ProxyDepositFactor",""],["RecoveryDeposit",""],["RemoveKeysLimit",""],["ReportLongevity",""],["RewardCurve",""],["RocksDbWeight","By default, Substrate uses `RocksDB`, so this will be the weight used throughout the runtime."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SS58Prefix",""],["SampleLength",""],["SessionKeys",""],["SessionsPerEra",""],["SignedDepositBase",""],["SignedDepositByte",""],["SignedMaxRefunds",""],["SignedMaxSubmissions",""],["SignedPhase",""],["SignedRewardBase",""],["SlashDeferDuration",""],["StakingMigrationV11OldPallet",""],["SubAccountDeposit",""],["SubmissionDeposit",""],["TemporarySlotLeasePeriodLength",""],["TransactionByteFee",""],["UncleGenerations",""],["UnsignedPhase",""],["UnvestedFundsAllowedWithdrawReasons",""],["Version",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["AssignedSlots",""],["Auctions",""],["AuthorityDiscovery",""],["AuthorityDiscoveryConfig",""],["Authorship",""],["Babe",""],["BabeConfig",""],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","`BlockId` type as expected by this runtime."],["Configuration",""],["ConfigurationConfig",""],["Crowdloan",""],["Dmp",""],["ElectionProviderMultiPhase",""],["Executive","Executive: handles dispatch to the various modules."],["FastUnstake",""],["Grandpa",""],["GrandpaConfig",""],["Header","Block header type as expected by this runtime."],["Historical",""],["Hrmp",""],["HrmpConfig",""],["Identity",""],["ImOnline",""],["ImOnlineConfig",""],["Indices",""],["IndicesConfig",""],["Initializer",""],["Multisig",""],["NominationPools",""],["NominationPoolsConfig",""],["Offences",""],["ParaInclusion",""],["ParaInherent",""],["ParaScheduler",""],["ParaSessionInfo",""],["ParachainsOrigin",""],["Paras",""],["ParasConfig",""],["ParasDisputes",""],["ParasShared",""],["ParasSlashing",""],["ParasSudoWrapper",""],["Preimage",""],["Proxy",""],["Recovery",""],["Registrar",""],["RegistrarConfig",""],["Scheduler",""],["Session",""],["SessionConfig",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The `SignedExtension` to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["Slots",""],["Staking",""],["StakingConfig",""],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["Timestamp",""],["TransactionPayment",""],["Ump",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["Vesting",""],["VestingConfig",""],["VoterList",""],["XcmPallet",""],["XcmPalletConfig",""]]};