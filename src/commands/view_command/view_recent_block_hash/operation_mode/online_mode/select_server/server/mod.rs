use dialoguer::Input;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::SelectServerContext)]
pub struct Server {}

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(input_context = super::SelectServerContext)]
#[interactive_clap(output_context = super::ViewRecentBlockHashCommandNetworkContext)]
pub struct CustomServer {
    #[interactive_clap(skip_default_from_cli)]
    #[interactive_clap(long)]
    pub url: crate::common::AvailableRpcServerUrl,
}

struct CustomServerContext {
    pub url: crate::common::AvailableRpcServerUrl,
}

impl CustomServerContext {
    fn _from_previous_context(
        _previous_context: super::SelectServerContext,
        scope: &<CustomServer as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {
            url: scope.url.clone(),
        }
    }
}

impl From<CustomServerContext> for super::ViewRecentBlockHashCommandNetworkContext {
    fn from(item: CustomServerContext) -> Self {
        Self {
            connection_config: crate::common::ConnectionConfig::from_custom_url(&item.url),
        }
    }
}

impl Server {
    pub async fn process(
        self,
        connection_config: crate::common::ConnectionConfig,
    ) -> crate::CliResult {
        recent_block_hash_status(connection_config).await
    }
}

impl CustomServer {
    fn from_cli_url(
        optional_cli_url: Option<
            <crate::common::AvailableRpcServerUrl as interactive_clap::ToCli>::CliVariant,
        >,
        context: &super::SelectServerContext,
    ) -> color_eyre::eyre::Result<crate::common::AvailableRpcServerUrl> {
        match optional_cli_url {
            Some(url) => Ok(url),
            None => {
                if let Ok(network) = std::env::var("CUSTOM_NETWORK") {
                    match network.parse() {
                        Ok(url) => {
                            println!("Using the URL address from CUSTOM_NETWORK: {}", network);
                            return Ok(url)
                        },
                        Err(err) => println!("Couldn't use the URL address from CUSTOM_NETWORK: {}. Error: {}", network, err),
                    }
                }
                Self::input_url(context)
            }
        }
    }

    pub fn input_url(
        _context: &super::SelectServerContext,
    ) -> color_eyre::eyre::Result<crate::common::AvailableRpcServerUrl> {
        Ok(Input::new()
            .with_prompt("What is the RPC endpoint?")
            .interact_text()?)
    }

    pub async fn process(self) -> crate::CliResult {
        let connection_config = crate::common::ConnectionConfig::from_custom_url(&self.url);
        recent_block_hash_status(connection_config).await
    }
}

async fn recent_block_hash_status(
    connection_config: crate::common::ConnectionConfig,
) -> crate::CliResult {
    let status = near_jsonrpc_client::JsonRpcClient::connect(connection_config.rpc_url().as_str())
        .call(near_jsonrpc_client::methods::status::RpcStatusRequest)
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to fetch public key information for nonce: {:?}",
                err
            ))
        })?;
    println!(
        "recent block hash: {:?}",
        status.sync_info.latest_block_hash
    );
    Ok(())
}
