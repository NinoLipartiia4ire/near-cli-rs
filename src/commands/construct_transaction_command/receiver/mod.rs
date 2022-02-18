#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = crate::common::SignerContext)]
pub struct Receiver {
    ///What is the account ID of the receiver?
    pub receiver_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    pub action: super::transaction_actions::NextAction,
}

impl Receiver {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            receiver_id: self.receiver_account_id.clone().into(),
            ..prepopulated_unsigned_transaction
        };
        self.action
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
