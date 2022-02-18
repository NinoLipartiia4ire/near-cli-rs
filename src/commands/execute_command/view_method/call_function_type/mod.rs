#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::operation_mode::online_mode::select_server::ExecuteViewMethodCommandNetworkContext)]
pub struct CallFunctionView {
    ///Enter a method name
    method_name: String,
    ///Enter args for function
    function_args: String,
    #[interactive_clap(subcommand)]
    selected_block_id: super::block_id::BlockId,
}

impl CallFunctionView {
    pub async fn process(
        self,
        network_connection_config: crate::common::ConnectionConfig,
        contract_account_id: near_primitives::types::AccountId,
    ) -> crate::CliResult {
        self.selected_block_id
            .process(
                contract_account_id,
                network_connection_config,
                self.method_name,
                self.function_args.into_bytes(),
            )
            .await
    }
}
