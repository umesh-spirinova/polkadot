FROM rust:1.65

COPY . /polkadot
WORKDIR /polkadot

# Install WebAssembly tools
RUN rustup target add wasm32-unknown-unknown
RUN rustup override set nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#Install dependenties
RUN apt-get update
RUN apt install cmake -y
RUN apt install -y protobuf-compiler
RUN apt-get -y install clang

#Run build
RUN cargo build --release

#Start the app
CMD ./target/release/polkadot --alice --validator  --base-path /tmp/relay/alice --chain rococo-custom-3-raw.json --public-addr=/ip4/54.242.89.129/tcp/2435 --listen-addr=/ip4/0.0.0.0/tcp/30333 --ws-port 9944 --rpc-methods=Unsafe --rpc-cors all

#Expose ports
EXPOSE 2435
EXPOSE 30333
EXPOSE 9944
