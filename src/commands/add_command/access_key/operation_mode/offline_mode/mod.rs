/// аргументы, необходимые для создания трансфера в offline mode
#[derive(Debug, Default, Clone, clap::Clap)]
#[clap(
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands)
)]
pub struct CliOfflineArgs {
    #[clap(subcommand)]
    pub send_from: Option<super::super::sender::CliSendFrom>,
}

#[derive(Debug, Clone)]
// #[interactive_clap(input_context = (), output_context = super::NetworkContext)]
pub struct OfflineArgs {
    send_from: super::super::sender::SendFrom,
}

pub struct InteractiveClapContextScopeForOfflineArgs {}

impl crate::common::ToInteractiveClapContextScope for OfflineArgs {
    type InteractiveClapContextScope = InteractiveClapContextScopeForOfflineArgs;
}

struct OfflineArgsContext {}

impl OfflineArgsContext {
    fn from_previous_context(
        previous_context: (),
        scope: <OfflineArgs as crate::common::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> Self {
        Self {}
    }
}

impl From<OfflineArgsContext> for super::NetworkContext {
    fn from(item: OfflineArgsContext) -> Self {
        Self {
            connection_config: None,
        }
    }
}

impl CliOfflineArgs {
    pub fn to_cli_args(&self) -> std::collections::VecDeque<String> {
        self.send_from
            .as_ref()
            .map(|subcommand| subcommand.to_cli_args())
            .unwrap_or_default()
    }
}

impl From<OfflineArgs> for CliOfflineArgs {
    fn from(offline_args: OfflineArgs) -> Self {
        Self {
            send_from: Some(offline_args.send_from.into()),
        }
    }
}

// impl OfflineArgs {
//     pub fn from(
//         item: CliOfflineArgs,
//         context: crate::common::Context,
//     ) -> color_eyre::eyre::Result<Self> {
//         let send_from = match item.send_from {
//             Some(cli_send_from) => {
//                 super::online_mode::select_server::server::SendFrom::from(cli_send_from, context)?
//             }
//             None => super::online_mode::select_server::server::SendFrom::choose_send_from(context)?,
//         };
//         Ok(Self { send_from })
//     }
// }

impl OfflineArgs {
    pub fn from(
        optional_clap_variant: Option<CliOfflineArgs>,
        context: (),
    ) -> color_eyre::eyre::Result<Self> {
        let new_context_scope = InteractiveClapContextScopeForOfflineArgs {
            // todo <Self as
        };
        let new_context: super::NetworkContext/*: NetworkContext */ = OfflineArgsContext::from_previous_context((), new_context_scope).into();
        let send_from = super::super::sender::SendFrom::from(
            optional_clap_variant.and_then(|clap_variant| clap_variant.send_from),
            &new_context,
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
