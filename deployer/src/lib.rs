use anyhow::{
    anyhow,
    Result,
};
use std::{
    env,
    fs,
    marker::Send,
    path::Path,
    time,
};
use web3::{
    Web3,
    api::Namespace,
    confirm,
    contract::{
        Contract,
        Options,
    },
    signing::{
        Key,
        SecretKeyRef,
    },
    transports::Http,
    types::{
        Address,
        TransactionReceipt,
        TransactionParameters,
        Bytes,
    }};
use secp256k1::key::SecretKey;
use hex;

#[derive(Debug)]
pub struct Signer {
    pub secret_key: SecretKey,
    pub address: Address,
}

impl Signer {
    pub fn new(key: &str) -> Result<Self> {
        let secret_key: SecretKey = key.parse().unwrap();
        let secret_key_ref = SecretKeyRef::new(&secret_key);
        let address = secret_key_ref.address();
        
        Ok(Signer {
            secret_key,
            address,
        })
    }
}

/// Components needed to deploy a contract
#[derive(Debug)]
pub struct EthDeployer {
    web3: Web3<Http>,
    eth_url: String,
    unlock_duration: u16,   // 要らないかも？
}

impl EthDeployer {
    pub fn new(node_url: &str) -> Result<Self> {
        let transport = Http::new(node_url)?;
        let web3 = Web3::new(transport);
        let unlock_duration = env::var("UNLOCK_DURATION")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u16>()
            .expect("Failed to parse UNLOCK_DURATION");

        Ok(EthDeployer {
            web3,
            eth_url: node_url.to_string(),
            unlock_duration,
        })
    }

    pub async fn deploy<P>(
        &self,
        bin_path: P,
        confirmations: usize,
        gas: u64,
        chain_id: u64,
        deployer: Signer,
    ) -> Result<Address>
    where
        P: AsRef<Path> + Send + Sync + Copy,
    {
        let bin = fs::read_to_string(bin_path)?;

        let tx = TransactionParameters {
            gas: gas.into(),
            chain_id: Some(chain_id),
            data: Bytes::from(hex::decode(bin).unwrap()),
            ..Default::default()
        };
        let signed = self.web3.accounts()
            .sign_transaction(tx, &deployer.secret_key)
            .await?;

        let receipt = confirm::send_raw_transaction_with_confirmation(
            self.web3.eth().transport().clone(),
            signed.raw_transaction,
            time::Duration::from_secs(1),
            confirmations,
        )
        .await?;
        let contract_address: Address = receipt.contract_address.unwrap();

        Ok(contract_address)
    }

    pub async fn deploy_anonify_by_factory<P>(
        &self,
        contract_type: &str,
        abi_path: P,
        signer: Signer,
        gas: u64,
        factory_address: Address,
        confirmations: usize,
    ) -> Result<TransactionReceipt>
    where
        P: AsRef<Path> + Send + Sync + Copy,
    {
        let abi = ethabi::Contract::load(&fs::read(abi_path)?[..])
            .map_err(|e| anyhow!("Failed to load contract abi.: {:?}", e))?;

        Contract::new(self.web3.eth(), factory_address, abi)
            .signed_call_with_confirmations(
                contract_type,
                (),
                Options::with(|opt| opt.gas = Some(gas.into())),
                confirmations,
                &signer.secret_key,
            )
            .await
            .map_err(Into::into)
    }

    // TODO: jkcomment 使わないんだけど、消しちゃう？
    pub async fn get_account(&self, index: usize, password: Option<&str>) -> Result<Address> {
        let accounts = self.web3.eth().accounts().await?;
        if accounts.len() <= index {
            return Err(anyhow!(
                "index {} is out of accounts length: {}",
                index,
                accounts.len()
            ))
            .map_err(Into::into);
        }
        let account = accounts[index];
        if let Some(pw) = password {
            if !self
                .web3
                .personal()
                .unlock_account(account, pw, Some(self.unlock_duration))
                .await?
            {
                return Err(anyhow!("account unlock error"));
            }
        }

        Ok(account)
    }
}