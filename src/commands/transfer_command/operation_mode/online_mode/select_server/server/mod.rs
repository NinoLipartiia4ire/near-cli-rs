use dialoguer::Input;
use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = super::SelectServerContext)]
pub struct Server {
    #[interactive_clap(named_arg)]
    ///Specify a sender
    pub send_from: super::super::super::super::sender::Sender,
}

pub struct InteractiveClapContextScopeForServer {}

impl crate::common::ToInteractiveClapContextScope for Server {
    type InteractiveClapContextScope = InteractiveClapContextScopeForServer;
}

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(input_context = SelectServerContext)]
#[interactive_clap(output_context = super::super::super::TransferCommandNetworkContext)]
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

impl From<CustomServerContext> for super::super::super::TransferCommandNetworkContext {
    fn from(item: CustomServerContext) -> Self {
        Self {
            connection_config: Some(crate::common::ConnectionConfig::from_custom_url(&item.url)),
        }
    }
}

impl Server {
    pub fn from(
        optional_clap_variant: Option<CliServer>,
        context: super::SelectServerContext,
    ) -> color_eyre::eyre::Result<Self> {
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            context.into(),
        )?;
        Ok(Self { send_from })
    }

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
    pub fn from(
        optional_clap_variant: Option<CliCustomServer>,
        context: super::SelectServerContext,
    ) -> color_eyre::eyre::Result<Self> {
        let url: crate::common::AvailableRpcServerUrl = match optional_clap_variant
            .clone()
            .and_then(|clap_variant| clap_variant.url)
        {
            Some(url) => url,
            None => Self::input_url(),
        };
        let new_context_scope = InteractiveClapContextScopeForCustomServer { url };
        let new_context =
            CustomServerContext::from_previous_context(context, &new_context_scope).into();
        let send_from = super::super::super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForCustomServer::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            new_context,
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
