use async_trait::async_trait;
use helios::client::{Client, ClientBuilder, FileDB};
use std::primitive::u64;

use crate::config::Config;

use super::EthereumLightClient;

/// Helios implementation of `EthereumLightClient`.
pub struct HeliosLightClient {
    /// The wrapped Helios client.
    pub helios_light_client: Client<FileDB>,
}

/// Implementation of `EthereumLightClient` for Helios.
#[async_trait]
impl EthereumLightClient for HeliosLightClient {
    async fn start(&mut self) -> eyre::Result<()> {
        // Start the Helios light client.
        self.helios_light_client.start().await
    }

    async fn call(
        &self,
        opts: &helios::types::CallOpts,
        block: helios::types::BlockTag,
    ) -> eyre::Result<Vec<u8>> {
        // Wrap the Helios call.
        self.helios_light_client.call(opts, block).await
    }

    async fn get_balance(
        &self,
        address: &ethers::types::Address,
        block: helios::types::BlockTag,
    ) -> eyre::Result<primitive_types::U256> {
        self.helios_light_client.get_balance(address, block).await
    }

    async fn get_nonce(
        &self,
        address: &ethers::types::Address,
        block: helios::types::BlockTag,
    ) -> eyre::Result<u64> {
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_number(&self) -> eyre::Result<u64> {
        self.helios_light_client.get_block_number().await
    }

    async fn chain_id(&self) -> u64 {
        self.helios_light_client.chain_id().await
    }
}

/// HeliosLightClient non-trait functions.
impl HeliosLightClient {
    /// Create a new HeliosLightClient.
    pub fn new(config: Config) -> eyre::Result<Self> {
        // Build the Helios wrapped light client.
        let helios_light_client = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .checkpoint("c93123ff83f8bd1fdbe3a0dbd8cfa3b491a3eda66ecd49fa21c4fd82985ed73b")
            .build()?;
        Ok(Self {
            helios_light_client,
        })
    }
}
