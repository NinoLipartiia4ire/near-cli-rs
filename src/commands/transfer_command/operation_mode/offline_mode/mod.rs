use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;

#[derive(Debug, Clone, InteractiveClap)]
pub struct OfflineArgs {
    #[interactive_clap(named_arg)]
    send_from: super::super::sender::Sender,
}

impl OfflineArgs {
    pub fn from(
        item: CliOfflineArgs,
        context: crate::common::Context,
    ) -> color_eyre::eyre::Result<Self> {
        let optional_clap_variant = Some(item);
        // let send_from = match item.send_from {
        //     Some(cli_send_from) => {
        //         super::online_mode::select_server::server::SendFrom::from(cli_send_from, context)?
        //     }
        //     None => super::online_mode::select_server::server::SendFrom::choose_variant(context)?,
        // };
        let send_from = super::super::sender::Sender::from(
            optional_clap_variant.and_then(|clap_variant| match clap_variant.send_from {
                Some(ClapNamedArgSenderForOfflineArgs::SendFrom(cli_sender)) => Some(cli_sender),
                None => None,
            }),
            context,
        )?;
        Ok(Self { send_from })
    }
}

impl OfflineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let selected_server_url = None;
        self.send_from
            .process(prepopulated_unsigned_transaction, selected_server_url)
            .await
    }
}
