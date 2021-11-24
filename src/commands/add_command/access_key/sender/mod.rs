use dialoguer::Input;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, clap::Clap)]
pub enum CliSendFrom {
    /// Specify a sender
    Account(CliSender),
}

#[derive(Debug, Clone, EnumDiscriminants)]
// #[interactive_clap(input_context = super::operation_mode::NetworkContext, output_context = super::operation_mode::NetworkContext)]
pub enum SendFrom {
    Account(Sender),
}

impl CliSendFrom {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Account(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("account".to_owned());
                args
            }
        }
    }
}

impl From<SendFrom> for CliSendFrom {
    fn from(send_from: SendFrom) -> Self {
        match send_from {
            SendFrom::Account(sender) => Self::Account(sender.into()),
        }
    }
}

impl SendFrom {
    pub fn from(
        optional_clap_variant: Option<CliSendFrom>,
        context: &super::operation_mode::NetworkContext,
    ) -> color_eyre::eyre::Result<Self> {
        match optional_clap_variant.and_then(|clap_variant| match clap_variant {
            CliSendFrom::Account(cli_sender) => Some(Self::Account(
                Sender::from(Some(cli_sender), context).unwrap(),
            )),
        }) {
            Some(x) => Ok(x),
            None => Self::choose_send_from(context),
        }
    }
}

impl SendFrom {
    pub fn choose_send_from(
        context: &super::operation_mode::NetworkContext,
        // connection_config: Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<Self> {
        Self::from(Some(CliSendFrom::Account(Default::default())), context)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        network_connection_config: Option<crate::common::ConnectionConfig>,
    ) -> crate::CliResult {
        match self {
            SendFrom::Account(sender) => {
                sender
                    .process(prepopulated_unsigned_transaction, network_connection_config)
                    .await
            }
        }
    }
}

/// данные об отправителе транзакции
#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliSender {
    pub sender_account_id: Option<near_primitives::types::AccountId>,
    #[clap(subcommand)]
    public_key_mode: Option<super::public_key_mode::CliPublicKeyMode>,
}

#[derive(Debug, Clone)]
//#[interactive_clap(input_context = super::operation_mode::NetworkContext, output_context = SenderContext)]
pub struct Sender {
    pub sender_account_id: near_primitives::types::AccountId,
    pub public_key_mode: super::public_key_mode::PublicKeyMode,
}

pub struct InteractiveClapContextScopeForSender {
    sender_account_id: near_primitives::types::AccountId,
}

impl crate::common::ToInteractiveClapContextScope for Sender {
    type InteractiveClapContextScope = InteractiveClapContextScopeForSender;
}

#[derive(Clone)]
pub struct SenderContext {
    pub connection_config: Option<crate::common::ConnectionConfig>,
    pub sender_account_id: near_primitives::types::AccountId,
}




impl CliSender {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .public_key_mode
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        if let Some(sender_account_id) = &self.sender_account_id {
            args.push_front(sender_account_id.to_string());
        }
        args
    }
}

impl From<Sender> for CliSender {
    fn from(sender: Sender) -> Self {
        Self {
            sender_account_id: Some(sender.sender_account_id),
            public_key_mode: Some(sender.public_key_mode.into()),
        }
    }
}

// impl Sender {
//     pub fn from(
//         item: CliSender,
//         context: crate::common::Context,
//         // connection_config: Option<crate::common::ConnectionConfig>,
//     ) -> color_eyre::eyre::Result<Self> {
//         let connection_config = context.connection_config.clone();
//         let sender_account_id: near_primitives::types::AccountId = match item.sender_account_id {
//             Some(cli_sender_account_id) => match &connection_config {
//                 Some(network_connection_config) => match crate::common::check_account_id(
//                     network_connection_config.clone(),
//                     cli_sender_account_id.clone(),
//                 )? {
//                     Some(_) => cli_sender_account_id,
//                     None => {
//                         println!("Account <{}> doesn't exist", cli_sender_account_id);
//                         Sender::input_sender_account_id(connection_config.clone())?
//                     }
//                 },
//                 None => cli_sender_account_id,
//             },
//             None => Sender::input_sender_account_id(connection_config.clone())?,
//         };
//         let context = crate::common::Context {
//             sender_account_id: Some(sender_account_id.clone()),
//             connection_config,
//         };
//         let public_key_mode = match item.public_key_mode {
//             Some(cli_public_key_mode) => super::public_key_mode::PublicKeyMode::from(
//                 cli_public_key_mode,
//                 context
//                 // connection_config,
//                 // sender_account_id.clone(),
//             )?,
//             None => super::public_key_mode::PublicKeyMode::choose_public_key_mode(
//                 context
//                 // connection_config,
//                 // sender_account_id.clone(),
//             )?,
//         };
//         Ok(Self {
//             sender_account_id,
//             public_key_mode,
//         })
//     }
// }
impl SenderContext {
    fn from_previous_context(
        previous_context: &super::operation_mode::NetworkContext,
        scope: &<Sender as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            connection_config: previous_context.connection_config.clone(),
            sender_account_id: scope.sender_account_id.clone(),
        }
    }
}

impl Sender {
    fn from(
        optional_clap_variant: Option<CliSender>,
        context: &super::operation_mode::NetworkContext,
    ) -> color_eyre::eyre::Result<Self> {
        let sender_account_id = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.sender_account_id)
        {
            Some(sender_account_id) => sender_account_id,
            None => Self::input_sender_account_id(&context)?,
        };
        // let new_context_scope = <Self as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope::from_sender_account_id(sender_account_id);
        type Alias = <Sender as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
        let new_context_scope = Alias {
            sender_account_id
        };
        let new_context /*: SignerContext */ = SenderContext::from_previous_context(context, &new_context_scope);
        let public_key_mode = super::public_key_mode::PublicKeyMode::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.public_key_mode),
            &new_context,
        )?;
        Ok(Self {
            sender_account_id: new_context_scope.sender_account_id,
            public_key_mode,
        })
    }
}

impl Sender {
    fn input_sender_account_id(
        context: &super::operation_mode::NetworkContext, // connection_config: &Option<crate::common::ConnectionConfig>,
    ) -> color_eyre::eyre::Result<near_primitives::types::AccountId> {
        let connection_config = context.connection_config.clone();
        loop {
            let account_id: near_primitives::types::AccountId = Input::new()
                .with_prompt("What account ID do you need to add a key?")
                .interact_text()
                .unwrap();
            if let Some(connection_config) = &connection_config {
                if let Some(_) =
                    crate::common::check_account_id(connection_config.clone(), account_id.clone())?
                {
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
            signer_id: self.sender_account_id.clone(),
            receiver_id: self.sender_account_id.clone(),
            ..prepopulated_unsigned_transaction
        };
        self.public_key_mode
            .process(unsigned_transaction, network_connection_config)
            .await
    }
}
