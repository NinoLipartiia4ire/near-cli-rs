pub mod select_server;
use interactive_clap::{ToCli, ToInteractiveClapContextScope};
use interactive_clap_derive::InteractiveClap;

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = ())]
pub struct NetworkArgs {
    #[interactive_clap(subcommand)]
    selected_server: self::select_server::SelectServer,
}

impl NetworkArgs {
    pub fn from(
        optional_clap_variant: Option<CliNetworkArgs>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let selected_server =
            match optional_clap_variant.and_then(|clap_variant| clap_variant.selected_server) {
                Some(cli_selected_server) => {
                    self::select_server::SelectServer::from(Some(cli_selected_server), context)?
                }
                None => self::select_server::SelectServer::choose_variant(context)?,
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
