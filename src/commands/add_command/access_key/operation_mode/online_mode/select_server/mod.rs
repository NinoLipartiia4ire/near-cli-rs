use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod server;

#[derive(Debug, Clone, clap::Clap)]
pub enum CliSelectServer {
    /// предоставление данных для сервера https://rpc.testnet.near.org
    Testnet(self::server::CliServer),
    /// предоставление данных для сервера https://rpc.mainnet.near.org
    Mainnet(self::server::CliServer),
    /// предоставление данных для сервера https://rpc.betanet.near.org
    Betanet(self::server::CliServer),
    /// предоставление данных для сервера, указанного вручную
    Custom(self::server::CliCustomServer),
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
// #[interactive_clap(input_context = (), output_context = SelectServerContext)]
pub enum SelectServer {
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(self::server::Server),
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(self::server::Server),
    #[strum_discriminants(strum(message = "Betanet"))]
    Betanet(self::server::Server),
    #[strum_discriminants(strum(message = "Custom"))]
    Custom(self::server::CustomServer),
}

impl CliSelectServer {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Testnet(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("testnet".to_owned());
                args
            }
            Self::Mainnet(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("mainnet".to_owned());
                args
            }
            Self::Betanet(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("betanet".to_owned());
                args
            }
            Self::Custom(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("custom".to_owned());
                args
            }
        }
    }
}

impl From<SelectServer> for CliSelectServer {
    fn from(select_server: SelectServer) -> Self {
        match select_server {
            SelectServer::Testnet(server) => Self::Testnet(server.into()),
            SelectServer::Mainnet(server) => Self::Mainnet(server.into()),
            SelectServer::Betanet(server) => Self::Betanet(server.into()),
            SelectServer::Custom(server) => Self::Custom(server.into()),
        }
    }
}

pub type InteractiveClapContextScopeForSelectServer = SelectServerDiscriminants;

impl crate::common::ToInteractiveClapContextScope for SelectServer {
    type InteractiveClapContextScope = InteractiveClapContextScopeForSelectServer;
}

pub struct SelectServerContext {
    selected_server: SelectServerDiscriminants,
}

impl SelectServerContext {
    fn from_previous_context(
        previous_context: (),
        scope: <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            selected_server: scope,
        }
    }
}

impl From<SelectServerContext> for super::super::NetworkContext {
    fn from(item: SelectServerContext) -> Self {
        let connection_config = match item.selected_server {
            SelectServerDiscriminants::Testnet => crate::common::ConnectionConfig::Testnet,
            SelectServerDiscriminants::Mainnet => crate::common::ConnectionConfig::Mainnet,
            SelectServerDiscriminants::Betanet => crate::common::ConnectionConfig::Betanet,
            SelectServerDiscriminants::Custom => {
                unreachable!("Network context should not be constructed from Custom variant")
            }
        };
        Self {
            connection_config: Some(connection_config),
        }
    }
}

// impl SelectServer {
//     pub fn from(
//         item: CliSelectServer,
//         context: crate::common::Context,
//     ) -> color_eyre::eyre::Result<Self> {
//         match item {
//             CliSelectServer::Testnet(cli_server) => Ok(Self::Testnet(
//                 cli_server.into_server(crate::common::ConnectionConfig::Testnet, context)?,
//             )),
//             CliSelectServer::Mainnet(cli_server) => Ok(Self::Mainnet(
//                 cli_server.into_server(crate::common::ConnectionConfig::Mainnet, context)?,
//             )),
//             CliSelectServer::Betanet(cli_server) => Ok(Self::Betanet(
//                 cli_server.into_server(crate::common::ConnectionConfig::Betanet, context)?,
//             )),
//             CliSelectServer::Custom(cli_custom_server) => {
//                 Ok(Self::Custom(cli_custom_server.into_server(context)?))
//             }
//         }
//     }
// }

impl SelectServer {
    pub fn from(
        optional_clap_variant: Option<CliSelectServer>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        match optional_clap_variant.and_then(|clap_variant| match clap_variant {
            CliSelectServer::Testnet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias::Testnet;
                let new_context = SelectServerContext::from_previous_context((), new_context_scope);
                Some(Self::Testnet(
                    self::server::Server::from(Some(cli_server), new_context.into()).ok()?,
                ))
            }
            CliSelectServer::Mainnet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias::Mainnet;
                let new_context = SelectServerContext::from_previous_context((), new_context_scope);
                Some(Self::Mainnet(
                    self::server::Server::from(Some(cli_server), new_context.into()).ok()?,
                ))
            }
            CliSelectServer::Betanet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias::Betanet;
                let new_context = SelectServerContext::from_previous_context((), new_context_scope);
                Some(Self::Betanet(
                    self::server::Server::from(Some(cli_server), new_context.into()).ok()?,
                ))
            }
            CliSelectServer::Custom(cli_custom_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias::Custom;
                let new_context = SelectServerContext::from_previous_context((), new_context_scope);
                Some(Self::Custom(
                    self::server::CustomServer::from(Some(cli_custom_server), new_context.into()).ok()?,
                ))
            }
        }) {
            Some(x) => {
                println!("++++++++++++ select server: {:?}", &x);
                Ok(x)
            }
            None => {
                println!("------------ select server: ");
                SelectServer::choose_server(context)
            }
        }
    }

    pub fn choose_server(context: ()) -> color_eyre::eyre::Result<Self> {
        println!();
        let variants = SelectServerDiscriminants::iter().collect::<Vec<_>>();
        let servers = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let selected_server = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NEAR protocol RPC server:")
            .items(&servers)
            .default(0)
            .interact()
            .unwrap();
        let cli_select_server = match variants[selected_server] {
            SelectServerDiscriminants::Testnet => CliSelectServer::Testnet(Default::default()),
            SelectServerDiscriminants::Mainnet => CliSelectServer::Mainnet(Default::default()),
            SelectServerDiscriminants::Betanet => CliSelectServer::Betanet(Default::default()),
            SelectServerDiscriminants::Custom => CliSelectServer::Custom(Default::default()),
        };
        // let connection_config = match variants[selected_server] {
        //     SelectServerDiscriminants::Testnet => crate::common::ConnectionConfig::Testnet,
        //     SelectServerDiscriminants::Mainnet => crate::common::ConnectionConfig::Mainnet,
        //     SelectServerDiscriminants::Betanet => crate::common::ConnectionConfig::Betanet,
        //     SelectServerDiscriminants::Custom => {
        //         let custom_url = self::server::CustomServer::input_url();
        //         crate::common::ConnectionConfig::from_custom_url(&custom_url)
        //     },
        // };
        // let new_context_scope = Self::InteractiveClapContextScope {
        //     connection_config,
        // };
        // let new_context = SelectServerContext::from(context, &new_context_scope);
        println!("**************** select server: {:?}", &cli_select_server);
        Ok(Self::from(Some(cli_select_server), context)?)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        Ok(match self {
            SelectServer::Testnet(server) => {
                let connection_config = crate::common::ConnectionConfig::Testnet;
                server
                    .process(prepopulated_unsigned_transaction, connection_config)
                    .await?;
            }
            SelectServer::Mainnet(server) => {
                let connection_config = crate::common::ConnectionConfig::Mainnet;
                server
                    .process(prepopulated_unsigned_transaction, connection_config)
                    .await?;
            }
            SelectServer::Betanet(server) => {
                let connection_config = crate::common::ConnectionConfig::Betanet;
                server
                    .process(prepopulated_unsigned_transaction, connection_config)
                    .await?;
            }
            SelectServer::Custom(custom_server) => {
                custom_server
                    .process(prepopulated_unsigned_transaction)
                    .await?;
            }
        })
    }
}
