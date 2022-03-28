use strum::{EnumDiscriminants, EnumIter, EnumMessage};
use std::env;

//use crate::commands::login::operation_mode::online_mode::select_server::SelectServerContext;

pub mod server;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = SelectServerContext)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = SelectServerContext)]
#[interactive_clap(skip_default_from_cli)]
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

impl SelectServer {
    pub fn from_cli(
        optional_clap_variant: Option<
            <SelectServer as interactive_clap::ToCli>::CliVariant,
        >,
        context: SelectServerContext,
    ) -> color_eyre::eyre::Result<Self> {
        let selected_network: SelectServer = match env::var("NETWORK") {
            Ok(network) => {
                panic!("Found a network");
                /*match &network.to_lowercase() {
                    String::from("testnet") => panic!("testnet")/*SelectServer::Testnet(crate::consts::TESTNET_API_SERVER_URL)*/,
                    String::from("mainnet") => panic!("mainnet")/*SelectServer::Mainnet(crate::consts::MAINNET_API_SERVER_URL)*/,
                    String::from("betanet") => panic!("betanet")/*SelectServer::Betanet(crate::consts::BETANET_API_SERVER_URL)*/,
                    _ => {
                        panic!("custom");
                        /*let separator_index = network.find(':');
                        assert!(separator_index < network.len(), "Enter the URL");
                        let url = network[separator_index + 1:]
                        if network.contains("custom") {
                            SelectServer::Custom(url),
                        } else {
                            panic!("Incorrect network name")
                        }*/
                    }, 
                }*/

            },
            Err(_) => {
                panic!("PANIC FOR NOW");
            },
        };
        Ok(selected_network)
    }
}

#[derive(Clone)]
pub struct SelectServerContext {
    selected_server: SelectServerDiscriminants,
}

impl SelectServerContext {
    fn from_previous_context(
        _previous_context: (),
        scope: &<SelectServer as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            selected_server: scope.clone(),
        }
    }
}

impl From<SelectServerContext> for super::super::AddContractCodeCommandNetworkContext {
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

impl SelectServer {
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
