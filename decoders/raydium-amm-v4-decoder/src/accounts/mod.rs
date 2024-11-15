use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::RaydiumAmmV4Decoder;
pub mod amm_info;
pub mod fees;
pub mod target_orders;

pub enum RaydiumAmmV4Account {
    TargetOrders(target_orders::TargetOrders),
    Fees(fees::Fees),
    AmmInfo(amm_info::AmmInfo),
}

impl<'a> AccountDecoder<'a> for RaydiumAmmV4Decoder {
    type AccountType = RaydiumAmmV4Account;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {

        // I suggest matching on the account data size instead of relying on the discriminator for rust native solana programs
        // this is just a POC that it works

        let data_size = account.data.len();
        match data_size {
            amm_info::AMM_INFO_SIZE => {
                if let Some(decoded_account) =
                    amm_info::AmmInfo::deserialize(account.data.as_slice())
                {
                    Some(carbon_core::account::DecodedAccount {
                        lamports: account.lamports,
                        data: RaydiumAmmV4Account::AmmInfo(decoded_account),
                        owner: account.owner,
                        executable: account.executable,
                        rent_epoch: account.rent_epoch,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }

        //     if let Some(decoded_account) = target_orders::TargetOrders::deserialize(account.data.as_slice()) {
        //     return Some(carbon_core::account::DecodedAccount {
        //         lamports: account.lamports,
        //         data: RaydiumAmmV4Account::TargetOrders(decoded_account),
        //         owner: account.owner,
        //         executable: account.executable,
        //         rent_epoch: account.rent_epoch,
        //     });
        // }

        //     if let Some(decoded_account) = fees::Fees::deserialize(account.data.as_slice()) {
        //     return Some(carbon_core::account::DecodedAccount {
        //         lamports: account.lamports,
        //         data: RaydiumAmmV4Account::Fees(decoded_account),
        //         owner: account.owner,
        //         executable: account.executable,
        //         rent_epoch: account.rent_epoch,
        //     });
        // }

        //     if let Some(decoded_account) = amm_info::AmmInfo::deserialize(account.data.as_slice()) {
        //     return Some(carbon_core::account::DecodedAccount {
        //         lamports: account.lamports,
        //         data: RaydiumAmmV4Account::AmmInfo(decoded_account),
        //         owner: account.owner,
        //         executable: account.executable,
        //         rent_epoch: account.rent_epoch,
        //     });
        // }

        // None
    }
}
