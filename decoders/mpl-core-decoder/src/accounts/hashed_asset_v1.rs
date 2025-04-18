use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc56d2e767fef7e32")]
pub struct HashedAssetV1 {
    pub key: Key,
    pub hash: [u8; 32],
}
