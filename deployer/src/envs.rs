use lazy_static::lazy_static;
use std::{env, path::PathBuf, ffi::OsStr};

lazy_static! {
    pub(crate) static ref PJ_ROOT_DIR: PathBuf = {
        let pj_name = env::var("PJ_NAME").unwrap_or("anonify-contracts".to_string());
        let mut current_dir = env::current_dir().unwrap();
        loop {
            if current_dir.file_name() == Some(OsStr::new(pj_name.as_str())) {
                break;
            }
            if !current_dir.pop() {
                break;
            }
        }

        current_dir
    };

    pub(crate) static ref ANONIFY_ABI_PATH: PathBuf = {
        let abi_path_from_root = env::var("ANONIFY_ABI_PATH")
            .unwrap_or_else(|_| "contract-build/AnonifyWithEnclaveKey.abi".to_string());
        let mut abi_path = PJ_ROOT_DIR.clone();
        abi_path.push(abi_path_from_root);
        abi_path
    };
    pub static ref ANONIFY_BIN_PATH: PathBuf = {
        let bin_path_from_root = env::var("ANONIFY_BIN_PATH")
            .unwrap_or_else(|_| "contract-build/AnonifyWithEnclaveKey.bin".to_string());
        let mut bin_path = PJ_ROOT_DIR.clone();
        bin_path.push(bin_path_from_root);
        bin_path
    };
    pub static ref FACTORY_ABI_PATH: PathBuf = {
        let abi_path_from_root = env::var("FACTORY_ABI_PATH")
            .unwrap_or_else(|_| "contract-build/DeployAnonify.abi".to_string());
        let mut abi_path = PJ_ROOT_DIR.clone();
        abi_path.push(abi_path_from_root);
        abi_path
    };
    pub static ref FACTORY_BIN_PATH: PathBuf = {
        let bin_path_from_root = env::var("FACTORY_BIN_PATH")
            .unwrap_or_else(|_| "contract-build/DeployAnonify.bin".to_string());
        let mut bin_path = PJ_ROOT_DIR.clone();
        bin_path.push(bin_path_from_root);
        bin_path
    };
}
