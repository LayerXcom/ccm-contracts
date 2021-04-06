use anyhow::{anyhow, Result};
use std::{env, fs, marker::Send, path::Path};
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, TransactionReceipt},
    Web3,
};

/// Components needed to deploy a contract
#[derive(Debug)]
pub struct EthDeployer {
    web3: Web3<Http>,
    eth_url: String,
    unlock_duration: u16,
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
        abi_path: P,
        bin_path: P,
        confirmations: usize,
        gas: u64,
        deployer: Address,
    ) -> Result<Address>
    where
        P: AsRef<Path> + Send + Sync + Copy,
    {
        let abi = fs::read(abi_path)?;
        let bin = fs::read_to_string(bin_path)?;

        let contract = Contract::deploy(self.web3.eth(), abi.as_slice())?
            .options(Options::with(|opt| opt.gas = Some(gas.into())))
            .confirmations(confirmations)
            .execute(bin.as_str(), (), deployer)
            .await?;

        Ok(contract.address())
    }

    pub async fn deploy_anonify_by_factory<P>(
        &self,
        contract_type: &str,
        abi_path: P,
        signer: Address,
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
            .call_with_confirmations(
                contract_type,
                (),
                signer,
                Options::with(|opt| opt.gas = Some(gas.into())),
                confirmations,
            )
            .await
            .map_err(Into::into)
    }

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
