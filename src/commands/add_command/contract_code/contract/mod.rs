mod initialize_mode;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = crate::common::SignerContext)]
pub struct ContractFile {
    ///What is a file location of the contract?
    pub file_path: crate::types::path_buf::PathBuf,
    #[interactive_clap(subcommand)]
    next_action: self::initialize_mode::NextAction,
}

impl ContractFile {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let code = std::fs::read(&self.file_path.0.clone())
            .map_err(|err| color_eyre::Report::msg(format!("Failed to open file: {:?}", err)))?;
        let action = near_primitives::transaction::Action::DeployContract(
            near_primitives::transaction::DeployContractAction { code },
        );
        let mut actions = prepopulated_unsigned_transaction.actions.clone();
        actions.push(action);
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions,
            ..prepopulated_unsigned_transaction
        };
        self.next_action
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
