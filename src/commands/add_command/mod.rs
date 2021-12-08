use interactive_clap::ToCli;
use interactive_clap_derive::InteractiveClap;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

// mod access_key;
mod contract_code;
// mod implicit_account;
// mod stake_proposal;
// mod sub_account;

#[derive(Debug, Clone, InteractiveClap)]
#[interactive_clap(context = ())]
pub struct AddAction {
    #[interactive_clap(subcommand)]
    pub action: Action,
}

impl AddAction {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        self.action.process(prepopulated_unsigned_transaction).await
    }
}

#[derive(Debug, Clone, EnumDiscriminants, InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
/// What do you want to add?
pub enum Action {
    // #[strum_discriminants(strum(message = "Add a new access key for an account"))]
    // AccessKey(self::access_key::operation_mode::OperationMode),
    #[strum_discriminants(strum(message = "Add a new contract code"))]
    ///Add contract code
    ContractCode(self::contract_code::operation_mode::OperationMode),
    // #[strum_discriminants(strum(message = "Add an implicit-account"))]
    // ImplicitAccount(self::implicit_account::ImplicitAccount),
    // #[strum_discriminants(strum(message = "Add a new stake proposal"))]
    // StakeProposal(self::stake_proposal::operation_mode::OperationMode),
    // #[strum_discriminants(strum(message = "Add a new sub-account"))]
    // SubAccount(self::sub_account::operation_mode::OperationMode),
}

impl Action {
    pub async fn process(
        self,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        match self {
            // Action::AccessKey(operation_mode) => {
            //     operation_mode
            //         .process(prepopulated_unsigned_transaction)
            //         .await
            // }
            Action::ContractCode(operation_mode) => {
                operation_mode
                    .process(prepopulated_unsigned_transaction)
                    .await
            } // Action::ImplicitAccount(generate_keypair) => generate_keypair.process().await,
              // Action::StakeProposal(operation_mode) => {
              //     operation_mode
              //         .process(prepopulated_unsigned_transaction)
              //         .await
              // }
              // Action::SubAccount(operation_mode) => {
              //     operation_mode
              //         .process(prepopulated_unsigned_transaction)
              //         .await
              // }
        }
    }
}
