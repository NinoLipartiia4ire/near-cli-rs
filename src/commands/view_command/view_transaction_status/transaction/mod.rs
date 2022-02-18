#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::operation_mode::online_mode::select_server::ViewTransactionCommandNetworkContext)]
pub struct TransactionType {
    ///Enter the hash of the transaction you need to view
    pub transaction_hash: crate::types::crypto_hash::CryptoHash,
    #[interactive_clap(named_arg)]
    signer: super::signer::Sender,
}

impl TransactionType {
    pub async fn process(
        self,
        network_connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        self.signer
            .process(network_connection_config, self.transaction_hash)
            .await
    }
}
