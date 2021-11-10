use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;

#[derive(Debug, Clone, InteractiveClap)]
pub struct Sender {
    pub sender_account_id: crate::types::account_id::AccountId,
    #[interactive_clap(named_arg)]
    pub send_to: super::receiver::Receiver,
}

impl Sender {
    pub fn from(
        // item: CliSender,
        optional_clap_variant: Option<CliSender>,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        // let optional_clap_variant = Some(item);
        let connection_config = context.connection_config.clone();
        // let sender_account_id: crate::types::account_id::AccountId = match item.sender_account_id {
        //     Some(cli_sender_account_id) => match &connection_config {
        //         Some(network_connection_config) => match crate::common::check_account_id(
        //             network_connection_config.clone(),
        //             cli_sender_account_id.clone().into(),
        //         )? {
        //             Some(_) => cli_sender_account_id,
        //             None => {
        //                 println!("Account <{}> doesn't exist", cli_sender_account_id);
        //                 Sender::input_sender_account_id(connection_config.clone())?
        //             }
        //         },
        //         None => cli_sender_account_id,
        //     },
        //     None => Sender::input_sender_account_id(connection_config.clone())?,
        // };
        let sender_account_id = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.sender_account_id)
        {
            Some(signer_account_id) => signer_account_id,
            None => Self::input_sender_account_id(connection_config.clone())?,
        };
        let context = crate::common::Context {
            sender_account_id: Some(sender_account_id.clone().into()),
            ..context
        };
        //-------------------- to do!
        // let send_to: super::receiver::Receiver = match item.send_to {
        //     Some(cli_send_to) => {
        //         let cli_receiver: super::receiver::CliReceiver = super::receiver::CliReceiver {
        //             receiver_account_id: None,
        //             transfer: None
        //         };
        //         super::receiver::Receiver::from(cli_receiver, context)?
        //     },
        //     None => {
        //         let receiver_account_id = super::receiver::Receiver::input_receiver_account_id(connection_config)?;
        //         let transfer: super::transfer_near_tokens_type::Transfer = super::transfer_near_tokens_type::Transfer::choose_variant(context)?;
        //         super::receiver::Receiver {
        //             receiver_account_id,
        //             transfer
        //         }
        //     },
        // };
        let send_to = super::receiver::Receiver::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_to {
                Some(ClapNamedArgReceiverForSender::SendTo(cli_receiver)) => Some(cli_receiver),
                None => None,
            }),
            context,
        )?;
        Ok(Self {
            sender_account_id,
            send_to,
        })
    }
}

impl Sender {
    fn input_sender_account_id(
        connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<crate::types::account_id::AccountId> {
        loop {
            let account_id: crate::types::account_id::AccountId = Input::new()
                .with_prompt("What is the account ID of the sender?")
                .interact_text()
                .unwrap();
            if let Some(connection_config) = &connection_config {
                if let Some(_) = crate::common::check_account_id(
                    connection_config.clone(),
                    account_id.clone().into(),
                )? {
                    break Ok(account_id);
                } else {
                    println!("Account <{}> doesn't exist", account_id.to_string());
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
            signer_id: self.sender_account_id.0.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.send_to
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
