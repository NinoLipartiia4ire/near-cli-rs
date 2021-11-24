use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, InteractiveClap)]
pub struct Server {
    #[interactive_clap(skip)]
    pub connection_config: crate::common::ConnectionConfig,
    #[interactive_clap(named_arg)]
    ///Specify a sender
    pub send_from: super::super::super::super::sender::Sender,
}

impl ToCli for crate::common::ConnectionConfig {
    type CliVariant = crate::common::ConnectionConfig;
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


#[derive(Debug, Clone, InteractiveClap)]
pub struct CustomServer {
    #[interactive_clap(long)]
    pub url: crate::common::AvailableRpcServerUrl,
    #[interactive_clap(named_arg)]
    pub send_from: super::super::super::super::sender::Sender,
}

impl ToCli for crate::common::AvailableRpcServerUrl {
    type CliVariant = crate::common::AvailableRpcServerUrl;
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


// impl CliServer {
//     pub fn into_server(
//         self,
//         connection_config: crate::common::ConnectionConfig,
//         context: crate::common::Context,
//     ) -> color_eyre::eyre::Result<Server> {
//         let optional_clap_variant = Some(self);
//         let context = crate::common::Context {
//             connection_config: Some(connection_config.clone()),
//             ..context
//         };
//         // let send_from = match self.send_from {
//         //     Some(cli_send_from) => SendFrom::from(cli_send_from, context)?,
//         //     None => SendFrom::choose_variant(context)?,
//         // };
//         let send_from = super::super::super::super::sender::Sender::from(
//             optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
//                 Some(ClapNamedArgSenderForServer::SendFrom(cli_sender)) => Some(cli_sender),
//                 None => None,
//             }),
//             context,
//         )?;
//         Ok(Server {
//             connection_config: Some(connection_config),
//             send_from,
//         })
//     }
// }

// impl CliCustomServer {
//     pub fn into_custom_server(
//         self,
//         context: crate::common::Context,
//     ) -> color_eyre::eyre::Result<CustomServer> {
//         let optional_clap_variant = Some(self.clone());
//         let url: crate::common::AvailableRpcServerUrl = match self.url {
//             Some(url) => url,
//             None => Input::new()
//                 .with_prompt("What is the RPC endpoint?")
//                 .interact_text()
//                 .unwrap(),
//         };
//         let connection_config = Some(crate::common::ConnectionConfig::from_custom_url(&url));
//         let context = crate::common::Context {
//             connection_config: connection_config.clone(),
//             ..context
//         };
//         // let send_from = match self.send_from {
//         //     Some(cli_send_from) => SendFrom::from(cli_send_from, context)?,
//         //     None => SendFrom::choose_variant(context)?,
//         // };
//         let send_from = super::super::super::super::sender::Sender::from(
//             optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
//                 Some(ClapNamedArgSenderForCustomServer::SendFrom(cli_sender)) => Some(cli_sender),
//                 None => None,
//             }),
//             context,
//         )?;
//         Ok(CustomServer { url, send_from })
//     }
// }

impl Server {
    pub fn from(
        optional_clap_variant: Option<CliServer>,
        // context: crate::common::Context,
        context: &super::super::super::NetworkContext,
    ) -> color_eyre::eyre::Result<Self> {
        let connection_config = context.connection_config.clone().expect("connection_config does not exist");
        // let connection_config = crate::common::ConnectionConfig::Testnet; // !!!!!!!!!!!
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
        // let send_from = super::super::super::super::sender::SendFrom::from(
        //     optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
        //     // &new_context,
        //     context
        // )?;
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            // context,
            crate::common::Context{connection_config: Some(connection_config.clone()), sender_account_id: None},
        )?;
        Ok(Self {
            connection_config,
            send_from,
        })
    }

    fn input_connection_config(
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<crate::common::ConnectionConfig> {
        println!("/////////  input connection config /////////// ");
        let selected_server = super::SelectServer::choose_variant(context)?;
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

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.send_from
            .process(prepopulated_unsigned_transaction, Some(self.connection_config))
            .await
    }
}

impl CustomServer {
    pub fn from(
        optional_clap_variant: Option<CliCustomServer>,
        context: crate::common::Context,
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
        // let send_from = super::super::super::super::sender::SendFrom::from(
        //     optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
        //     &new_context,
        // )?;
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForCustomServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            context,
        )?;
        Ok(Self { url, send_from })
    }

    pub fn input_url() -> crate::common::AvailableRpcServerUrl {
        Input::new()
            .with_prompt("What is the RPC endpoint?")
            .interact_text()
            .unwrap()
    }

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
