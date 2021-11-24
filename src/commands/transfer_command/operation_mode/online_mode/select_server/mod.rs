use async_recursion::async_recursion;
use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod server;

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = crate::common::Context)]
#[interactive_clap(fn_from = default)]
///Select NEAR protocol RPC server
pub enum SelectServer {
    /// Provide data for the server https://rpc.testnet.near.org
    #[strum_discriminants(strum(message = "Testnet"))]
    Testnet(self::server::Server),
    /// Provide data for the server https://rpc.mainnet.near.org
    #[strum_discriminants(strum(message = "Mainnet"))]
    Mainnet(self::server::Server),
    /// Provide data for the server https://rpc.betanet.near.org
    #[strum_discriminants(strum(message = "Betanet"))]
    Betanet(self::server::Server),
    /// Provide data for a manually specified server
    #[strum_discriminants(strum(message = "Custom"))]
    Custom(self::server::CustomServer),
}

pub struct InteractiveClapContextScopeForSelectServer {
    connection_config: Option<crate::common::ConnectionConfig>,
}

impl crate::common::ToInteractiveClapContextScope for SelectServer {
    type InteractiveClapContextScope = InteractiveClapContextScopeForSelectServer;
}

struct SelectServerContext {
    connection_config: crate::common::ConnectionConfig,
}

impl SelectServerContext {
    fn from_previous_context(
        previous_context: (),
        scope: <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            connection_config: scope.connection_config.unwrap(),
        }
    }
}

impl From<SelectServerContext> for super::super::NetworkContext {
    fn from(item: SelectServerContext) -> Self {
        Self {
            connection_config: Some(item.connection_config),
        }
    }
}


impl SelectServer {
    pub fn from(
        optional_clap_variant: Option<CliSelectServer>,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        match optional_clap_variant.and_then(|clap_variant| match clap_variant {
            CliSelectServer::Testnet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias {
                    connection_config: Some(crate::common::ConnectionConfig::Testnet),
                };
                let new_context: super::super::NetworkContext/*: NetworkContext */ = SelectServerContext::from_previous_context((), new_context_scope).into();
                Some(Self::Testnet(
                    self::server::Server::from(Some(cli_server), &new_context).ok()?,
                ))
            }
            CliSelectServer::Mainnet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias {
                    connection_config: Some(crate::common::ConnectionConfig::Mainnet),
                };
                let new_context: super::super::NetworkContext/*: NetworkContext */ = SelectServerContext::from_previous_context((), new_context_scope).into();                
                Some(Self::Mainnet(
                    self::server::Server::from(Some(cli_server), &new_context).ok()?,
                ))
            }
            CliSelectServer::Betanet(cli_server) => {
                type Alias = <SelectServer as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope;
                let new_context_scope = Alias {
                    connection_config: Some(crate::common::ConnectionConfig::Betanet),
                };
                let new_context: super::super::NetworkContext/*: NetworkContext */ = SelectServerContext::from_previous_context((), new_context_scope).into();
                Some(Self::Betanet(
                    self::server::Server::from(Some(cli_server), &new_context).ok()?,
                ))
            }
            CliSelectServer::Custom(cli_custom_server) => {
                let custom_url = self::server::CustomServer::input_url();
                let new_context_scope = InteractiveClapContextScopeForSelectServer {// <Self as
                    connection_config: Some(crate::common::ConnectionConfig::from_custom_url(&custom_url))
                };
                let new_context: super::super::NetworkContext/*: NetworkContext */ = SelectServerContext::from_previous_context((), new_context_scope).into();
                Some(Self::Custom(
                    self::server::CustomServer::from(Some(cli_custom_server), context.clone()).ok()?,
                ))
            }
        }) {
            Some(x) => {
                println!("++++++++++++ select server: {:?}", &x);
                Ok(x)
            }
            None => {
                println!("------------ select server: ");
                SelectServer::choose_variant(context)
            }
        }
    }
}

impl SelectServer {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        Ok(match self {
            SelectServer::Testnet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Mainnet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Betanet(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
            SelectServer::Custom(server) => {
                server.process(prepopulated_unsigned_transaction).await?;
            }
        })
    }
}
