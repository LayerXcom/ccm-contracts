# anonify-contracts

## contracts

Building an Anonify contract.

```bash
solc -o contract-build --bin --abi --optimize --overwrite contracts/AnonifyWithTreeKem.sol contracts/AnonifyWithEnclaveKey.sol contracts/Factory.sol
```

## deployer

A command-line utilities for deploying anonify contracts in solidity.

### Usage

```bash
export CONFIRMATIONS=1
export ETH_URL=https://u1hf4ydgf1:W79M06uApx55c_6xdJ-VfH9PxDthklww5tq4Psj3kFk@u1b7qg6u7m-u1wjgac2in-rpc.us1-azure.kaleido.io
```

```bash
# Deploy a factory contract
cargo run factory

# Deploy a `AnonifyWithTreeKem` or `AnonifyWithEnclaveKey` contract depending on `ANONIFY_ABI_PATH` and `ANONIFY_BIN_PATH` environment variables directly
cargo run anonify_direct

# Deploy a `AnonifyWithTreeKem` contract by the factory contract
cargo run anonify_tk <FACTORY CONTRACT ADDRESS>

# Deploy a `AnonifyWithEnclaveKey` contract by the factory contract
cargo run anonify_ek <FACTORY CONTRACT ADDRESS>
```
