use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WriteExternalPluginAdapterDataV1Args {
    pub key: ExternalPluginAdapterKey,
    pub data: Option<Vec<u8>>,
}
