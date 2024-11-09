#[cfg(not(target_arch = "wasm32"))]
pub mod contract;

#[cfg(not(target_arch = "wasm32"))]
pub mod pinata;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
#[cfg(not(target_arch = "wasm32"))]
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};
#[cfg(not(target_arch = "wasm32"))]
use anyhow::{Result, anyhow};

#[cfg(not(target_arch = "wasm32"))]
pub fn load_keypair_from_path(keypair_path: &str) -> Result<Keypair> {
    read_keypair_file(&keypair_path)
        .map_err(|e| anyhow!("Failed to load keypair from {}: {}", keypair_path, e))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_env_keypair(env_var: &str) -> Result<Keypair> {
    let keypair_path = std::env::var(env_var)
        .map_err(|_| anyhow!("Environment variable {} not set", env_var))?;

    read_keypair_file(&keypair_path)
        .map_err(|e| anyhow!("Failed to load keypair from {}: {}", keypair_path, e))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_json_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}