use crate::common::{display_access_key_list, display_account_info, ConnectionConfig};

use near_primitives::types::{AccountId, BlockId, BlockReference};

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::super::operation_mode::online_mode::select_server::ViewAccountSummaryCommandNetworkContext)]
pub struct BlockIdHash {
    ///Type the block ID hash for this account
    block_id_hash: crate::types::crypto_hash::CryptoHash,
}

impl BlockIdHash {
    pub async fn process(self, account_id: AccountId, conf: ConnectionConfig) -> crate::CliResult {
        let block_ref = BlockReference::BlockId(BlockId::Hash(self.block_id_hash.clone().into()));
        display_account_info(account_id.clone(), &conf, block_ref.clone()).await?;
        display_access_key_list(account_id, &conf, block_ref).await?;
        Ok(())
    }
}
