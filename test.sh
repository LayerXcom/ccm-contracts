#!/bin/bash

set -e

solc -o contract-build --bin --abi --optimize --overwrite \
    contracts/AnonifyWithTreeKem.sol \
    contracts/AnonifyWithEnclaveKey.sol \
    contracts/Factory.sol

export CONFIRMATIONS=0
export ETH_URL=http://0.0.0.0:8554

PJ_ROOT_DIR=$(cd $(dirname $0) && pwd)
cd "${PJ_ROOT_DIR}/deployer"
FACTORY_CONTRACT_ADDRESS=$(cargo run factory)
cargo run anonify_direct
cargo run anonify_tk "$FACTORY_CONTRACT_ADDRESS"
cargo run anonify_ek "$FACTORY_CONTRACT_ADDRESS"