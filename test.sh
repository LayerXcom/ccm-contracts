#!/bin/bash

set -e

export PATH=~/.cargo/bin:$PATH

solc -o contract-build --bin --abi --optimize --overwrite \
    contracts/AnonifyWithTreeKem.sol \
    contracts/AnonifyWithEnclaveKey.sol \
    contracts/Factory.sol

# export ETH_URL=http://172.16.11.2:8545
export PJ_NAME=anonify-contracts
# Besu用
export CONFIRMATIONS=1
export ETH_URL=http://besur64ps.eastus.cloudapp.azure.com/jsonrpc
export SIGNER_PRI_KEY=eeb069697a386bb8d5a985af0a4a227be39e9eee8d8d73deadb74672b9605ad3
export CHAIN_ID=2018

# ganache用
# export CONFIRMATIONS=0
# export ETH_URL=http://localhost:8545
# export CHAIN_ID=1337
# export SIGNER_PRI_KEY=eeb069697a386bb8d5a985af0a4a227be39e9eee8d8d73deadb74672b9605ad3

PJ_ROOT_DIR=$(cd $(dirname $0) && pwd)
cd "${PJ_ROOT_DIR}/deployer"
FACTORY_CONTRACT_ADDRESS=$(cargo run factory)
cargo run anonify_direct
cargo run anonify_tk "$FACTORY_CONTRACT_ADDRESS"
cargo run anonify_ek "$FACTORY_CONTRACT_ADDRESS"
