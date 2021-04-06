# inherit the baidu sdk image
FROM rust:1.51 as builder
LABEL maintainer="osuke.sudo@layerx.co.jp"

ENV PJ_NAME=anonify
ENV PJ_ROOT=/root/$PJ_NAME

SHELL ["/bin/bash", "-c"]

RUN set -x && \
    apt-get update && \
    apt-get upgrade -y --no-install-recommends && \
    apt-get install -y --no-install-recommends software-properties-common nodejs && \
    rm -rf /var/lib/apt/lists/* && \
    curl -o /usr/bin/solc -fL https://github.com/ethereum/solidity/releases/download/v0.7.4/solc-static-linux && \
    chmod u+x /usr/bin/solc && \
    rm -rf /root/.cargo/registry && rm -rf /root/.cargo/git

COPY . $PJ_ROOT
WORKDIR $PJ_ROOT

RUN solc -o contract-build --bin --abi --optimize --overwrite \
        contracts/AnonifyWithTreeKem.sol \
        contracts/AnonifyWithEnclaveKey.sol \
        contracts/Factory.sol && \
    cd $PJ_ROOT/deployer && \
    RUST_BACKTRACE=1 RUST_LOG=debug cargo build --release

# ===== SECOND STAGE ======
FROM rust:1.51
LABEL maintainer="osuke.sudo@layerx.co.jp"

ENV PJ_NAME=anonify
ENV PJ_ROOT=/root/$PJ_NAME

WORKDIR $PJ_ROOT

RUN cd $PJ_ROOT
COPY --from=builder $PJ_ROOT/target/release/eth-deployer ./target/release/
COPY --from=builder $PJ_ROOT/contract-build/AnonifyWithEnclaveKey.abi ./contract-build/
COPY --from=builder $PJ_ROOT/contract-build/AnonifyWithEnclaveKey.bin ./contract-build/
COPY --from=builder $PJ_ROOT/contract-build/AnonifyWithTreeKem.abi ./contract-build/
COPY --from=builder $PJ_ROOT/contract-build/AnonifyWithTreeKem.bin ./contract-build/
COPY --from=builder $PJ_ROOT/contract-build/DeployAnonify.abi ./contract-build/
COPY --from=builder $PJ_ROOT/contract-build/DeployAnonify.bin ./contract-build/

ENTRYPOINT ["./target/release/eth-deployer"]
CMD ["$1", "$2"]
