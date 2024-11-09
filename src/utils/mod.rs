pub mod contract;

pub mod pinata;

use std::fs::File;
use std::io::Read;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};
use anyhow::{Result, anyhow};

pub fn load_keypair() -> Result<Keypair> {
    let keypair_path = std::env::var("SOLANA_KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", std::env::var("HOME").unwrap()));

    read_keypair_file(&keypair_path)
        .map_err(|e| anyhow!("Failed to load keypair from {}: {}", keypair_path, e))
}

pub fn load_env_keypair(env_var: &str) -> Result<Keypair> {
    let keypair_path = std::env::var(env_var)
        .map_err(|_| anyhow!("Environment variable {} not set", env_var))?;

    read_keypair_file(&keypair_path)
        .map_err(|e| anyhow!("Failed to load keypair from {}: {}", keypair_path, e))
}

pub fn read_json_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}