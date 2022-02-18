#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = crate::common::SignerContext)]
pub struct ContractFile {
    ///Where to download the contract file?
    pub file_path: crate::types::path_buf::PathBuf,
    #[interactive_clap(subcommand)]
    pub selected_block_id: super::super::super::block_id::BlockId,
}

impl ContractFile {
    pub async fn process(
        self,
        contract_id: near_primitives::types::AccountId,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        self.selected_block_id
            .process(
                contract_id,
                network_connection_config,
                Some(self.file_path.into()),
            )
            .await
    }
}
