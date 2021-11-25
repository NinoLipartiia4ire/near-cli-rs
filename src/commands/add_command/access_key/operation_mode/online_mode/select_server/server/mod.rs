use std::str::FromStr;

use dialoguer::Input;

/// предустановленный RPC-сервер
#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliServer {
    #[clap(subcommand)]
    pub send_from: Option<super::super::super::super::sender::CliSendFrom>,
}

#[derive(Debug, Clone)]
// #[interactive_clap(context = super::SelectServerContext)]
pub struct Server {
    pub send_from: super::super::super::super::sender::SendFrom,
}

pub struct InteractiveClapContextScopeForServer {}

impl crate::common::ToInteractiveClapContextScope for Server {
    type InteractiveClapContextScope = InteractiveClapContextScopeForServer;
}

#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliCustomServer {
    #[clap(long)]
    pub url: Option<crate::common::AvailableRpcServerUrl>,
    #[clap(subcommand)]
    send_from: Option<super::super::super::super::sender::CliSendFrom>,
}

#[derive(Debug, Clone)]
// #[interactive_clap(input_context = SelectServerContext, output_context = CustomServerContext)]
pub struct CustomServer {
    pub url: crate::common::AvailableRpcServerUrl,
    pub send_from: super::super::super::super::sender::SendFrom,
}

pub struct InteractiveClapContextScopeForCustomServer {
    pub url: crate::common::AvailableRpcServerUrl,
}

impl crate::common::ToInteractiveClapContextScope for CustomServer {
    type InteractiveClapContextScope = InteractiveClapContextScopeForCustomServer;
}

struct CustomServerContext {
    pub url: crate::common::AvailableRpcServerUrl,
}

impl CustomServerContext {
    fn from_previous_context(
        previous_context: super::SelectServerContext,
        scope: &<CustomServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            url: scope.url.clone(),
        }
    }
}

impl From<CustomServerContext> for super::super::super::NetworkContext {
    fn from(item: CustomServerContext) -> Self {
        Self {
            connection_config: Some(crate::common::ConnectionConfig::from_custom_url(&item.url)),
        }
    }
}

impl CliCustomServer {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        let mut args = self
            .send_from
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default();
        if let Some(url) = &self.url {
            args.push_front(url.to_string());
            args.push_front("--url".to_string());
        }
        args
    }
}

impl CustomServer {
    pub fn from(
        optional_clap_variant: Option<CliCustomServer>,
        context: super::SelectServerContext,
    ) -> color_eyre::eyre::Result<Self> {
        let url: crate::common::AvailableRpcServerUrl = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.url)
        {
            Some(url) => url,
            None => Input::new()
                .with_prompt("What is the RPC endpoint?")
                .interact_text()
                .unwrap(),
        };
        let new_context_scope = InteractiveClapContextScopeForCustomServer {
            // todo <Self as
            url,
        };
        let new_context/*: CustomServerContext */ = CustomServerContext::from_previous_context(context, &new_context_scope);
        let send_from = super::super::super::super::sender::SendFrom::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
            new_context.into(),
        )?;
        Ok(Self {
            url: new_context_scope.url,
            send_from,
        })
    }

    pub fn input_url() -> crate::common::AvailableRpcServerUrl {
        Input::new()
            .with_prompt("What is the RPC endpoint?")
            .interact_text()
            .unwrap()
    }
}

impl From<CustomServer> for CliCustomServer {
    fn from(custom_server: CustomServer) -> Self {
        Self {
            url: Some(custom_server.url),
            send_from: Some(custom_server.send_from.into()),
        }
    }
}

impl CliServer {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        self.send_from
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default()
    }
}

impl From<Server> for CliServer {
    fn from(server: Server) -> Self {
        Self {
            send_from: Some(server.send_from.into()),
        }
    }
}

impl Server {
    pub fn from(
        optional_clap_variant: Option<CliServer>,
        context: super::SelectServerContext,
    ) -> color_eyre::eyre::Result<Self> {
        let send_from = super::super::super::super::sender::SendFrom::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
            context.into(),
        )?;
        Ok(Self { send_from })
    }
}

impl Server {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        self.send_from
            .process(prepopulated_unsigned_transaction, Some(connection_config))
            .await
    }
}

impl CustomServer {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let connection_config = Some(crate::common::ConnectionConfig::from_custom_url(&self.url));
        self.send_from
            .process(prepopulated_unsigned_transaction, connection_config)
            .await
    }
}

// #[derive(Debug, Clone, clap::Clap)]
// pub enum CliSendFrom {
//     /// Specify a sender
//     Account(super::super::super::super::sender::CliSender),
// }

// #[derive(Debug, Clone)]
// pub enum SendFrom {
//     Account(super::super::super::super::sender::Sender),
// }

// impl CliSendFrom {
//     pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
//         match self {
//             Self::Account(subcommand) => {
//                 let mut args = subcommand.to_cli_args();
//                 args.push_front("account".to_owned());
//                 args
//             }
//         }
//     }
// }

// impl From<SendFrom> for CliSendFrom {
//     fn from(send_from: SendFrom) -> Self {
//         match send_from {
//             SendFrom::Account(sender) => Self::Account(sender.into()),
//         }
//     }
// }

// impl SendFrom {
//     pub fn from(
//         item: CliSendFrom,
//         context: super::SelectServerContext,
//         // connection_config: Option<crate::common::ConnectionConfig>,
//     ) -> color_eyre::eyre::Result<Self> {
//         match item {
//             CliSendFrom::Account(cli_sender) => Ok(Self::Account(
//                 super::super::super::super::sender::Sender::from(cli_sender, context)?,
//             )),
//         }
//     }
// }

// impl SendFrom {
//     pub fn choose_send_from(
//         context: super::SelectServerContext,
//         // connection_config: Option<crate::common::ConnectionConfig>,
//     ) -> color_eyre::eyre::Result<Self> {
//         Self::from(CliSendFrom::Account(Default::default()), context)
//     }

//     pub async fn process(
//         self,
//         prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
//         network_connection_config: Option<crate::common::ConnectionConfig>,
//     ) -> crate::CliResult {
//         match self {
//             SendFrom::Account(sender) => {
//                 sender
//                     .process(prepopulated_unsigned_transaction, network_connection_config)
//                     .await
//             }
//         }
//     }
// }
