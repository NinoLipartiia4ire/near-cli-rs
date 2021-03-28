use dialoguer::{theme::ColorfulTheme, Input, Select};
use structopt::StructOpt;

mod select_server;
use select_server::server::{CliSendFrom, SendFrom};
use select_server::{CliSelectServer, SelectServer};

#[derive(Debug, Default, StructOpt)]
pub struct CliOperationMode {
    #[structopt(subcommand)]
    mode: Option<CliMode>,
}

#[derive(Debug)]
pub struct OperationMode {
    pub mode: Mode,
}

impl OperationMode {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self.mode {
            Mode::Online(online_args) => {
                online_args.process(prepopulated_unsigned_transaction).await
            }
            Mode::Offline(offline_args) => {
                offline_args
                    .process(prepopulated_unsigned_transaction)
                    .await
            }
        }
    }
}

impl From<CliOperationMode> for OperationMode {
    fn from(item: CliOperationMode) -> Self {
        let cli_mode = match item.mode {
            Some(cli_mode) => cli_mode,
            None => Mode::choose_mode(),
        };
        Self { mode: Mode::from(cli_mode) }
    }
}

#[derive(Debug)]
pub enum Mode {
    Online(OnlineArgs),
    Offline(OfflineArgs),
}

impl Mode {
    pub fn choose_mode() -> CliMode {
        let choose_mode = vec![
            "Yes, I keep it simple",
            "No, I want to work in no-network (air-gapped) environment",
        ];
        println!();
        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
                 \nDo you want to derive some information required for transaction construction automatically querying it online?"
            )
            .items(&choose_mode)
            .default(0)
            .interact()
            .unwrap();
        match choose_mode[select_mode] {
            "Yes, I keep it simple" => {
                CliMode::Online(Default::default())
            }
            "No, I want to work in no-network (air-gapped) environment" => {
                CliMode::Offline(Default::default())
            }
            _ => unreachable!("Error"),
        }
    }
}

#[derive(Debug)]
pub struct OfflineArgs {
    nonce: u64,
    block_hash: near_primitives::hash::CryptoHash,
    send_from: SendFrom,
}

#[derive(Debug, Default, StructOpt)]
pub struct CliOfflineArgs {
    #[structopt(long)]
    nonce: Option<u64>,
    #[structopt(long)]
    block_hash: Option<crate::common::BlockHashAsBase58>,
    #[structopt(subcommand)]
    pub send_from: Option<CliSendFrom>,
}

#[derive(Debug)]
pub struct OnlineArgs {
    selected_server: SelectServer,
}

#[derive(Debug, Default, StructOpt)]
pub struct CliOnlineArgs {
    #[structopt(subcommand)]
    selected_server: Option<CliSelectServer>,
}

impl From<CliOnlineArgs> for OnlineArgs {
    fn from(item: CliOnlineArgs) -> Self {
        let cli_selected_server = match item.selected_server {
            Some(cli_selected_server) => cli_selected_server,
            None => SelectServer::select_server(),
        };
        OnlineArgs { selected_server: SelectServer::from(cli_selected_server) }
    }
}

impl From<CliOfflineArgs> for OfflineArgs {
    fn from(item: CliOfflineArgs) -> Self {
        let nonce: u64 = match item.nonce {
            Some(cli_nonce) => cli_nonce,
            None => OfflineArgs::input_nonce(),
        };
        let block_hash = match item.block_hash {
            Some(cli_block_hash) => cli_block_hash.inner,
            None => OfflineArgs::input_block_hash(),
        };
        let cli_send_from: CliSendFrom = match item.send_from {
            Some(cli_send_from) => cli_send_from,
            None => SendFrom::send_from(),
        };
        OfflineArgs {
            nonce,
            block_hash,
            send_from: SendFrom::from(cli_send_from),
        }
    }
}

impl OfflineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        println!("OfflineArgs process self:\n        {:?}", &self);
        println!(
            "OfflineArgs process prepopulated_unsigned_transaction:\n        {:?}",
            prepopulated_unsigned_transaction
        );
        let selected_server_url = None;
        let nonce = self.nonce.clone();
        let block_hash = self.block_hash.clone();
        let unsigned_transaction = near_primitives::transaction::Transaction {
            block_hash,
            nonce,
            ..prepopulated_unsigned_transaction
        };
        self.send_from
            .process(unsigned_transaction, selected_server_url)
            .await
    }
    fn input_nonce() -> u64 {
        Input::new()
            .with_prompt(
                "Enter transaction nonce (query the access key information with \
                `near-cli utils view-access-key frol4.testnet ed25519:...` incremented by 1)",
            )
            .interact_text()
            .unwrap()
    }
    fn input_block_hash() -> near_primitives::hash::CryptoHash {
        let input_block_hash: crate::common::BlockHashAsBase58 = Input::new()
            .with_prompt("Enter recent block hash")
            .interact_text()
            .unwrap();
        input_block_hash.inner
    }
}

impl OnlineArgs {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        println!(
            "OnlineArgs process:\n        {:?}",
            prepopulated_unsigned_transaction
        );
        self.selected_server
            .process(prepopulated_unsigned_transaction)
            .await
    }
}

#[derive(Debug, StructOpt)]
pub enum CliMode {
    Online(CliOnlineArgs),
    Offline(CliOfflineArgs),
}

impl From<CliMode> for Mode {
    fn from(item: CliMode) -> Self {
        match item {
            CliMode::Online(cli_online_args) => {
                let online_args: OnlineArgs = OnlineArgs::from(cli_online_args);
                Mode::Online(online_args)
            }
            CliMode::Offline(cli_offline_args) => {
                let offline_args: OfflineArgs = OfflineArgs::from(cli_offline_args);
                Mode::Offline(offline_args)
            }
        }
    }
}
