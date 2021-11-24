pub mod select_server;

/// аргументы, необходимые для создания транзакции в online mode
#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliNetworkArgs {
    #[clap(subcommand)]
    selected_server: Option<self::select_server::CliSelectServer>,
}

#[derive(Debug, Clone)]
pub struct NetworkArgs {
    selected_server: self::select_server::SelectServer,
}

impl CliNetworkArgs {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        self.selected_server
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default()
    }
}

impl From<NetworkArgs> for CliNetworkArgs {
    fn from(network_args: NetworkArgs) -> Self {
        Self {
            selected_server: Some(network_args.selected_server.into()),
        }
    }
}

impl NetworkArgs {
    pub fn from(
        optional_clap_variant: Option<CliNetworkArgs>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let selected_server =
            match optional_clap_variant.and_then(|clap_variant| {
                match clap_variant.selected_server {
                    Some(cli_selected_server) => Some(
                        self::select_server::SelectServer::from(Some(cli_selected_server), context)
                            .unwrap(),
                    ),
                    None => {
                        Some(self::select_server::SelectServer::choose_server(context).unwrap())
                    }
                }
            }) {
                Some(x) => x,
                None => self::select_server::SelectServer::choose_server(context)?,
            };
        Ok(Self { selected_server })
    }
}

impl NetworkArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.selected_server
            .process(prepopulated_unsigned_transaction)
            .await
    }
}
