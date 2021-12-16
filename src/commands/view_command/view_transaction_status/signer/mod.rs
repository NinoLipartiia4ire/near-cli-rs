use dialoguer::Input;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::operation_mode::online_mode::select_server::ViewTransactionCommandNetworkContext)]
#[interactive_clap(fn_from_cli = default)]
pub struct Sender {
    pub sender_account_id: crate::types::account_id::AccountId,
}

impl Sender {
    pub fn from(
        optional_clap_variant: Option<CliSender>,
        context: super::operation_mode::online_mode::select_server::ViewTransactionCommandNetworkContext,
    ) -> color_eyre::eyre::Result<Self> {
        let connection_config = context.connection_config.clone();
        let sender_account_id = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.sender_account_id)
        {
            Some(sender_account_id) => match crate::common::check_account_id(
                connection_config.clone(),
                sender_account_id.clone().into(),
            )? {
                Some(_) => sender_account_id,
                None => {
                    println!("Contract <{}> doesn't exist", sender_account_id);
                    Self::input_sender_account_id(&context)?
                }
            },
            None => Self::input_sender_account_id(&context)?,
        };
        Ok(Self { sender_account_id })
    }
}

impl Sender {
    pub fn input_sender_account_id(
        context: &super::operation_mode::online_mode::select_server::ViewTransactionCommandNetworkContext,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        let connection_config = context.connection_config.clone();
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("Specify the account that signed the transaction")
                .interact_text()
                .unwrap();
            if let Some(_) = crate::common::check_account_id(
                connection_config.clone(),
                account_id.clone().into(),
            )? {
                break Ok(account_id);
            } else {
                println!("Account <{}> doesn't exist", account_id.to_string());
            };
        }
    }

    fn rpc_client(&self, selected_server_url: &str) -> near_jsonrpc_client::JsonRpcClient {
        near_jsonrpc_client::new_client(&selected_server_url)
    }

    pub async fn process(
        self,
        network_connection_config: crate::common::ConnectionConfig,
        transaction_hash: String,
    ) -> crate::CliResult {
        let account_id = self.sender_account_id.clone();
        let query_view_transaction_status = self
            .rpc_client(network_connection_config.archival_rpc_url().as_str())
            .tx(transaction_hash, account_id.into())
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to fetch query for view transaction: {:?}",
                    err
                ))
            })?;
        println!("Transactiion status: {:#?}", query_view_transaction_status);
        Ok(())
    }
}
