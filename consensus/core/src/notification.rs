use crate::{block::Block, tx::TransactionId, utxo::utxo_diff::UtxoDiff};
use derive_more::Display;
use hashes::Hash;
use kaspa_notify::{
    events::EventType,
    full_featured,
    notification::Notification as NotificationTrait,
    subscription::{
        single::{OverallSubscription, UtxosChangedSubscription, VirtualSelectedParentChainChangedSubscription},
        Single,
    },
};
use std::sync::Arc;

full_featured! {
#[derive(Clone, Debug, Display)]
pub enum Notification {
    #[display(fmt = "BlockAdded notification: block hash {}", "_0.block.header.hash")]
    BlockAdded(BlockAddedNotification),

    #[display(fmt = "VirtualSelectedParentChainChanged notification: {} removed blocks, {} added blocks, {} accepted transactions", "_0.removed_chain_block_hashes.len()", "_0.added_chain_block_hashes.len()", "_0.accepted_transaction_ids.len()")]
    VirtualSelectedParentChainChanged(VirtualSelectedParentChainChangedNotification),

    #[display(fmt = "FinalityConflict notification: violating block hash {}", "_0.violating_block_hash")]
    FinalityConflict(FinalityConflictNotification),

    #[display(fmt = "FinalityConflict notification: violating block hash {}", "_0.finality_block_hash")]
    FinalityConflictResolved(FinalityConflictResolvedNotification),

    #[display(fmt = "UtxosChanged notification")]
    UtxosChanged(UtxosChangedNotification),

    #[display(fmt = "VirtualSelectedParentBlueScoreChanged notification: virtual selected parent blue score {}", "_0.virtual_selected_parent_blue_score")]
    VirtualSelectedParentBlueScoreChanged(VirtualSelectedParentBlueScoreChangedNotification),

    #[display(fmt = "VirtualDaaScoreChanged notification: virtual DAA score {}", "_0.virtual_daa_score")]
    VirtualDaaScoreChanged(VirtualDaaScoreChangedNotification),

    #[display(fmt = "PruningPointUtxoSetOverride notification")]
    PruningPointUtxoSetOverride(PruningPointUtxoSetOverrideNotification),

    #[display(fmt = "NewBlockTemplate notification")]
    NewBlockTemplate(NewBlockTemplateNotification),
}
}

impl NotificationTrait for Notification {
    fn apply_overall_subscription(&self, subscription: &OverallSubscription) -> Option<Self> {
        match subscription.active() {
            true => Some(self.clone()),
            false => None,
        }
    }

    fn apply_virtual_selected_parent_chain_changed_subscription(
        &self,
        subscription: &VirtualSelectedParentChainChangedSubscription,
    ) -> Option<Self> {
        match subscription.active() {
            true => {
                if let Notification::VirtualSelectedParentChainChanged(ref payload) = self {
                    if !subscription.include_accepted_transaction_ids() && !payload.accepted_transaction_ids.is_empty() {
                        return Some(Notification::VirtualSelectedParentChainChanged(VirtualSelectedParentChainChangedNotification {
                            removed_chain_block_hashes: payload.removed_chain_block_hashes.clone(),
                            added_chain_block_hashes: payload.added_chain_block_hashes.clone(),
                            accepted_transaction_ids: Arc::new(vec![]),
                        }));
                    }
                }
                Some(self.clone())
            }
            false => None,
        }
    }

    fn apply_utxos_changed_subscription(&self, _subscription: &UtxosChangedSubscription) -> Option<Self> {
        Some(self.clone())
    }

    fn event_type(&self) -> EventType {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct BlockAddedNotification {
    pub block: Arc<Block>,
}

#[derive(Debug, Clone)]
pub struct VirtualSelectedParentChainChangedNotification {
    pub added_chain_block_hashes: Arc<Vec<Hash>>,
    pub removed_chain_block_hashes: Arc<Vec<Hash>>,
    pub accepted_transaction_ids: Arc<Vec<TransactionId>>,
}

#[derive(Debug, Clone, Default)]
pub struct FinalityConflictNotification {
    pub violating_block_hash: Arc<Hash>,
}

#[derive(Debug, Clone, Default)]
pub struct FinalityConflictResolvedNotification {
    pub finality_block_hash: Arc<Hash>,
}

#[derive(Debug, Clone)]
pub struct UtxosChangedNotification {
    /// Accumulated UTXO diff between the last virtual state and the current virtual state
    pub accumulated_utxo_diff: Arc<UtxoDiff>,
}

#[derive(Debug, Clone)]
pub struct VirtualSelectedParentBlueScoreChangedNotification {
    pub virtual_selected_parent_blue_score: u64,
}

#[derive(Debug, Clone)]
pub struct VirtualDaaScoreChangedNotification {
    pub virtual_daa_score: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PruningPointUtxoSetOverrideNotification {}

#[derive(Debug, Clone)]
pub struct NewBlockTemplateNotification {}
