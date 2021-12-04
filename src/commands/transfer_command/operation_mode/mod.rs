use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::{ToCli, ToInteractiveClapContextScope};
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

mod offline_mode;
mod online_mode;

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = ())]
pub struct OperationMode {
    #[interactive_clap(subcommand)]
    pub mode: Mode,
}

impl OperationMode {
    pub fn from(
        optional_clap_variant: Option<CliOperationMode>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let mode = match optional_clap_variant.and_then(|clap_variant| clap_variant.mode) {
            Some(cli_mode) => Mode::from(Some(cli_mode), context)?,
            None => Mode::choose_variant(context)?,
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

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
///Do you want to derive some information required for transaction construction automatically querying it online?
pub enum Mode {
    /// Prepare and, optionally, submit a new transaction with online mode
    #[strum_discriminants(strum(message = "Yes, I keep it simple"))]
    Network(self::online_mode::NetworkArgs),
    /// Prepare and, optionally, submit a new transaction with offline mode
    #[strum_discriminants(strum(
        message = "No, I want to work in no-network (air-gapped) environment"
    ))]
    Offline(self::offline_mode::OfflineArgs),
}

impl Mode {
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

pub struct TransferCommandNetworkContext {
    pub connection_config: Option<crate::common::ConnectionConfig>,
}
