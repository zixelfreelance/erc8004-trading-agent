use ethers::prelude::*;
use std::sync::Arc;

pub type EthClient = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub fn build_client(
    rpc_url: &str,
    private_key: &str,
    chain_id: u64,
) -> anyhow::Result<EthClient> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let clean = private_key.strip_prefix("0x").unwrap_or(private_key);
    let wallet: LocalWallet = clean.parse::<LocalWallet>()?.with_chain_id(chain_id);
    Ok(Arc::new(SignerMiddleware::new(provider, wallet)))
}
