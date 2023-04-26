use crate::{Address, Result};
use async_trait::async_trait;
use kaspa_bip32::ExtendedPublicKey;
use std::sync::Arc;

#[async_trait]
pub trait WalletDerivationManagerTrait: Send + Sync {
    async fn from_master_xprv(xprv: &str, is_multisig: bool, account_index: u64) -> Result<Self>
    where
        Self: Sized;

    async fn from_extended_public_key_str(xpub: &str) -> Result<Self>
    where
        Self: Sized;

    async fn from_extended_public_key(extended_public_key: ExtendedPublicKey<secp256k1::PublicKey>) -> Result<Self>
    where
        Self: Sized;

    fn receive_address_manager(&self) -> Arc<dyn AddressDerivationManagerTrait>;
    fn change_address_manager(&self) -> Arc<dyn AddressDerivationManagerTrait>;

    async fn derive_receive_address(&self, index: u32) -> Result<Address>;
    async fn derive_change_address(&self, index: u32) -> Result<Address>;

    async fn new_receive_address(&self) -> Result<Address>;
    async fn new_change_address(&self) -> Result<Address>;
}

#[async_trait]
pub trait AddressDerivationManagerTrait: Send + Sync {
    async fn new_address(&self) -> Result<Address>;
    async fn current_address(&self) -> Result<Address>;
    fn index(&self) -> Result<u32>;
    fn set_index(&self, index: u32) -> Result<()>;
    async fn get_range(&self, range: std::ops::Range<u32>) -> Result<Vec<Address>>;
}
