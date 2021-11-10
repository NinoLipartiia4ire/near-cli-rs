use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, InteractiveClap)]
pub struct Server {
    #[interactive_clap(skip)]
    pub connection_config: Option<crate::common::ConnectionConfig>,
    #[interactive_clap(named_arg)]
    ///Specify a sender
    pub send_from: super::super::super::super::sender::Sender,
}

impl ToCli for crate::common::ConnectionConfig {
    type CliVariant = crate::common::ConnectionConfig;
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

impl CliServer {
    pub fn into_server(
        self,
        connection_config: crate::common::ConnectionConfig,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Server> {
        let optional_clap_variant = Some(self);
        let context = crate::common::Context {
            connection_config: Some(connection_config.clone()),
            ..context
        };
        // let send_from = match self.send_from {
        //     Some(cli_send_from) => SendFrom::from(cli_send_from, context)?,
        //     None => SendFrom::choose_variant(context)?,
        // };
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            context,
        )?;
        Ok(Server {
            connection_config: Some(connection_config),
            send_from,
        })
    }
}

impl CliCustomServer {
    pub fn into_custom_server(
        self,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<CustomServer> {
        let optional_clap_variant = Some(self.clone());
        let url: crate::common::AvailableRpcServerUrl = match self.url {
            Some(url) => url,
            None => Input::new()
                .with_prompt("What is the RPC endpoint?")
                .interact_text()
                .unwrap(),
        };
        let connection_config = Some(crate::common::ConnectionConfig::from_custom_url(&url));
        let context = crate::common::Context {
            connection_config: connection_config.clone(),
            ..context
        };
        // let send_from = match self.send_from {
        //     Some(cli_send_from) => SendFrom::from(cli_send_from, context)?,
        //     None => SendFrom::choose_variant(context)?,
        // };
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForCustomServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            context,
        )?;
        Ok(CustomServer { url, send_from })
    }
}

impl Server {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.send_from
            .process(prepopulated_unsigned_transaction, self.connection_config)
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
