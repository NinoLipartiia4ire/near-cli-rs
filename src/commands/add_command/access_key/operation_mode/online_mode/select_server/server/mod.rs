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
    #[clap(skip)]
    pub connection_config: Option<crate::common::ConnectionConfig>,
    #[clap(subcommand)]
    pub send_from: Option<super::super::super::super::sender::CliSendFrom>,
}

#[derive(Debug, Clone)]
// #[interactive_clap(input_context = (), output_context = super::NetworkContext)]
pub struct Server {
    pub connection_config: crate::common::ConnectionConfig,
    pub send_from: super::super::super::super::sender::SendFrom,
}

pub struct InteractiveClapContextScopeForServer {
    connection_config: Option<crate::common::ConnectionConfig>,
}

impl crate::common::ToInteractiveClapContextScope for Server {
    type InteractiveClapContextScope = InteractiveClapContextScopeForServer;
}

struct ServerContext {
    connection_config: crate::common::ConnectionConfig,
}

impl ServerContext {
    fn from_previous_context(
        previous_context: (),
        scope: <Server as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            connection_config: scope.connection_config.unwrap(),
        }
    }
}

impl From<ServerContext> for super::super::super::NetworkContext {
    fn from(item: ServerContext) -> Self {
        Self {
            connection_config: Some(item.connection_config),
        }
    }
}

/// данные для custom server
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
// #[interactive_clap(input_context = (), output_context = super::NetworkContext)]
pub struct CustomServer {
    pub url: crate::common::AvailableRpcServerUrl,
    pub send_from: super::super::super::super::sender::SendFrom,
}

pub struct InteractiveClapContextScopeForCustomServer {
    connection_config: Option<crate::common::ConnectionConfig>,
}

impl crate::common::ToInteractiveClapContextScope for CustomServer {
    type InteractiveClapContextScope = InteractiveClapContextScopeForCustomServer;
}

struct CustomServerContext {
    connection_config: crate::common::ConnectionConfig,
}

impl CustomServerContext {
    fn from_previous_context(
        previous_context: (),
        scope: <CustomServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            connection_config: scope.connection_config.unwrap(),
        }
    }
}

impl From<CustomServerContext> for super::super::super::NetworkContext {
    fn from(item: CustomServerContext) -> Self {
        Self {
            connection_config: Some(item.connection_config),
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
        context: &super::super::super::NetworkContext,
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
        let connection_config = crate::common::ConnectionConfig::from_custom_url(&url);
        let new_context_scope = InteractiveClapContextScopeForServer {// todo <Self as
            connection_config: Some(connection_config),
        };
        let new_context: super::super::super::NetworkContext/*: NetworkContext */ = ServerContext::from_previous_context((), new_context_scope).into();
        let send_from = super::super::super::super::sender::SendFrom::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
            &new_context,
        )?;
        Ok(Self { url, send_from })
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
            connection_config: Some(server.connection_config.into()),
            send_from: Some(server.send_from.into()),
        }
    }
}

impl Server {
    pub fn from(
        optional_clap_variant: Option<CliServer>,
        context: &super::super::super::NetworkContext,
    ) -> color_eyre::eyre::Result<Self> {
        let connection_config = context.connection_config.clone().expect("connection_config does not exist");
        // let connection_config = match optional_clap_variant
        //     .clone()
        //     .and_then(|clap_variant| clap_variant.connection_config)
        // {
        //     Some(connection_config) => connection_config,
        //     None => Self::input_connection_config(context)?,
        // };
        let new_context_scope = InteractiveClapContextScopeForServer {// todo <Self as
            connection_config: Some(connection_config.clone()),
        };
        // let new_context: super::super::super::NetworkContext/*: NetworkContext */ = ServerContext::from_previous_context((), new_context_scope).into();
        let send_from = super::super::super::super::sender::SendFrom::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
            // &new_context,
            context
        )?;
        Ok(Self {
            connection_config,
            send_from,
        })
    }
    // pub fn from(
    //     item: CliServer,
    //     context: super::SelectServerContext,
    // ) -> color_eyre::eyre::Result<Self> {
    //     let connection_config = context.connection_config;
    //     let send_from = match item.send_from {
    //         Some(cli_send_from) => super::super::super::super::sender::SendFrom::from(cli_send_from, context.connection_config)?,
    //         None => super::super::super::super::sender::SendFrom::choose_send_from(context.connection_config)?
    //     };
    //     Ok(Self {
    //         connection_config,
    //         send_from
    //     })
    // }

    fn input_connection_config(
        context: (),
    ) -> color_eyre::eyre::Result<crate::common::ConnectionConfig> {
        println!("/////////  input connection config /////////// ");
        let selected_server = super::SelectServer::choose_server(context)?;
        println!(
            "/////////  input connection config /////////// {:?}",
            &selected_server
        );
        Ok(match selected_server {
            super::SelectServer::Testnet(server) => crate::common::ConnectionConfig::Testnet,
            super::SelectServer::Mainnet(server) => crate::common::ConnectionConfig::Mainnet,
            super::SelectServer::Betanet(server) => crate::common::ConnectionConfig::Betanet,
            super::SelectServer::Custom(custom_server) => {
                let custom_url = super::server::CustomServer::input_url();
                crate::common::ConnectionConfig::from_custom_url(&custom_url)
            }
        })
    }
}

impl CliServer {
    // pub fn into_server(
    //     self,
    //     connection_config: crate::common::ConnectionConfig,
    //     context: (),
    //     optional_clap_variant: Option<Server>
    // ) -> color_eyre::eyre::Result<Server> {
    //     // let context = crate::common::Context {
    //     //     connection_config: Some(connection_config.clone()),
    //     //     ..context
    //     // };
    //     // let send_from = match self.send_from {
    //     //     Some(cli_send_from) => SendFrom::from(cli_send_from, context)?,
    //     //     None => SendFrom::choose_send_from(context)?,
    //     // };
    //     Ok(Server {
    //         connection_config: Some(connection_config),
    //         send_from,
    //     })
    // }
}

impl CliCustomServer {
    // pub fn into_server(self, context: crate::common::Context) -> color_eyre::eyre::Result<CustomServer> {
    //     let url: crate::common::AvailableRpcServerUrl = match self.url {
    //         Some(url) => url,
    //         None => Input::new()
    //             .with_prompt("What is the RPC endpoint?")
    //             .interact_text()
    //             .unwrap(),
    //     };
    //     let connection_config = Some(crate::common::ConnectionConfig::Custom {
    //         url: url.inner.clone(),
    //     });
    //     let context = crate::common::Context {
    //         connection_config: connection_config.clone(),
    //         ..context
    //     };
    //     let send_from = match self.send_from {
    //         Some(cli_send_from) => super::super::super::super::sender::SendFrom::from(cli_send_from, context)?,
    //         None => super::super::super::super::sender::SendFrom::choose_send_from(context)?,
    //     };
    //     Ok(CustomServer {
    //         url,
    //         send_from,
    //     })
    // }
}

impl Server {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.send_from
            .process(
                prepopulated_unsigned_transaction,
                Some(self.connection_config),
            )
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
