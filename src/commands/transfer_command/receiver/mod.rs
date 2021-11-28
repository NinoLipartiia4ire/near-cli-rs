use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = super::sender::SenderContext)]
pub struct Receiver {
    pub receiver_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(subcommand)]
    pub transfer: super::transfer_near_tokens_type::Transfer,
}

impl ToCli for crate::types::account_id::AccountId {
    type CliVariant = crate::types::account_id::AccountId;
}

impl Receiver {
    pub fn from(
        optional_clap_variant: Option<CliReceiver>,
        context: crate::common::SenderContext,
    ) -> color_eyre::eyre::Result<Self> {
        let receiver_account_id = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.receiver_account_id)
        {
            Some(receiver_account_id) => receiver_account_id,
            None => Self::input_receiver_account_id(context.connection_config.clone())?,
        };
        let transfer = super::transfer_near_tokens_type::Transfer::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.transfer),
            context,
        )?;

        Ok(Self {
            receiver_account_id,
            transfer,
        })
    }
}

impl Receiver {
    pub fn input_receiver_account_id(
        connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("What is the account ID of the receiver?")
                .interact_text()
                .unwrap();
            if let Some(connection_config) = &connection_config {
                if let Some(_) = crate::common::check_account_id(
                    connection_config.clone(),
                    account_id.clone().into(),
                )? {
                    break Ok(account_id);
                } else {
                    if !crate::common::is_64_len_hex(&account_id) {
                        println!("Account <{}> doesn't exist", account_id.to_string());
                    } else {
                        break Ok(account_id);
                    }
                }
            } else {
                break Ok(account_id);
            }
        }
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            receiver_id: self.receiver_account_id.clone().into(),
            ..prepopulated_unsigned_transaction
        };
        self.transfer
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
