use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod offline_mode;
mod online_mode;

/// инструмент выбора режима online/offline
#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliOperationMode {
    #[clap(subcommand)]
    mode: Option<CliMode>,
}

#[derive(Debug, Clone)]
pub struct OperationMode {
    pub mode: Mode,
}

impl CliOperationMode {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        self.mode
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default()
    }
}

impl From<OperationMode> for CliOperationMode {
    fn from(item: OperationMode) -> Self {
        Self {
            mode: Some(item.mode.into()),
        }
    }
}

impl OperationMode {
    pub fn from(
        optional_clap_variant: Option<CliOperationMode>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let mode = match optional_clap_variant.and_then(|clap_variant| clap_variant.mode) {
            Some(cli_mode) => Mode::from(Some(cli_mode), context)?,
            None => Mode::choose_mode(context)?,
        };
        Ok(Self { mode })
    }
}

impl OperationMode {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.mode.process(prepopulated_unsigned_transaction).await
    }
}

#[derive(Debug, Clone, clap::Clap)]
pub enum CliMode {
    /// Prepare and, optionally, submit a new transaction with online mode
    Network(self::online_mode::CliNetworkArgs),
    /// Prepare and, optionally, submit a new transaction with offline mode
    Offline(self::offline_mode::CliOfflineArgs),
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
pub enum Mode {
    #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
    Network(self::online_mode::NetworkArgs),
    #[strum_discriminants(strum(
        message = "No, I want to work in no-network (air-gapped) environment"
    ))]
    Offline(self::offline_mode::OfflineArgs),
}

impl CliMode {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        match self {
            Self::Network(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("network".to_owned());
                args
            }
            Self::Offline(subcommand) => {
                let mut args = subcommand.to_cli_args();
                args.push_front("offline".to_owned());
                args
            }
        }
    }
}

impl From<Mode> for CliMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Network(network_args) => {
                Self::Network(self::online_mode::CliNetworkArgs::from(network_args))
            }
            Mode::Offline(offline_args) => {
                Self::Offline(self::offline_mode::CliOfflineArgs::from(offline_args))
            }
        }
    }
}

impl Mode {
    fn from(optional_clap_variant: Option<CliMode>, context: ()) -> color_eyre::eyre::Result<Self> {
        match optional_clap_variant.and_then(|clap_variant| match clap_variant {
            CliMode::Network(cli_network_args) => Some(Self::Network(
                self::online_mode::NetworkArgs::from(Some(cli_network_args), context).ok()?,
            )),
            CliMode::Offline(cli_offline_args) => Some(Self::Offline(
                self::offline_mode::OfflineArgs::from(Some(cli_offline_args), context).ok()?,
            )),
        }) {
            Some(x) => Ok(x),
            None => Self::choose_mode(context),
        }
    }
}

impl Mode {
    fn choose_mode(context: ()) -> color_eyre::eyre::Result<Self> {
        println!();
        let variants = ModeDiscriminants::iter().collect::<Vec<_>>();
        let modes = variants
            .iter()
            .map(|p| p.get_message().unwrap().to_owned())
            .collect::<Vec<_>>();
        let selected_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
                 \nDo you want to derive some information required for transaction construction automatically querying it online?"
            )
            .items(&modes)
            .default(0)
            .interact()
            .unwrap();
        let cli_mode = match variants[selected_mode] {
            ModeDiscriminants::Network => CliMode::Network(Default::default()),
            ModeDiscriminants::Offline => CliMode::Offline(Default::default()),
        };
        Ok(Self::from(Some(cli_mode), context)?)
    }

    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Network(network_args) => {
                network_args
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
            Self::Offline(offline_args) => {
                offline_args
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}

pub struct InteractiveClapContextScopeForNetworkContext {
    connection_config: Option<crate::common::ConnectionConfig>,
}

impl crate::common::ToInteractiveClapContextScope for NetworkContext {
    type InteractiveClapContextScope = InteractiveClapContextScopeForNetworkContext;
}

pub struct NetworkContext {
    pub connection_config: Option<crate::common::ConnectionConfig>,
}

// impl NetworkContext {
//     fn from_previous_context(previous_context: (), scope: Network::InteractiveClapContextScope) -> Self {
//         Self {
//             connection_config: Option<crate::common::ConnectionConfig>
//         }
//     }
// }

// impl From<OnlineModeContext> for NetworkContext {
//     fn from(online_context: OnlineModeContext) -> Self {
//         Self {
//             connection_config: Some(online_context.connection_config),
//         }
//     }
// }

// impl From<OfflineModeContext> for NetworkContext {
//     fn from(online_context: OfflineModeContext) -> Self {
//         Self {
//             connection_config: None,
//         }
//     }
// }
