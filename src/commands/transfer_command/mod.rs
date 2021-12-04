use dialoguer::{theme::ColorfulTheme, Select};
use interactive_clap::{ToCli, ToInteractiveClapContextScope};
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage, IntoEnumIterator};

pub mod operation_mode;
mod receiver;
mod sender;
pub mod transfer_near_tokens_type;

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = ())]
pub struct Currency {
    #[interactive_clap(subcommand)]
    currency_selection: CurrencySelection,
}

impl Currency {
    pub fn from(
        optional_clap_variant: Option<CliCurrency>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let currency_selection =
            match optional_clap_variant.and_then(|clap_variant| clap_variant.currency_selection) {
                Some(cli_currency_selection) => {
                    CurrencySelection::from(Some(cli_currency_selection), context)?
                }
                None => CurrencySelection::choose_variant(context)?,
            };
        Ok(Self { currency_selection })
    }
}

impl Currency {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.currency_selection
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///What do you want to transfer?
pub enum CurrencySelection {
    /// The transfer is carried out in NEAR tokens
    #[strum_discriminants(strum(message = "NEAR tokens"))]
    Near(self::operation_mode::OperationMode),
}

impl CurrencySelection {
    async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            Self::Near(operation_mode) => {
                operation_mode
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}
