use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x457d73daf5baf2c4")]
pub struct SwapRouterBaseIn {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

pub struct SwapRouterBaseInInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapRouterBaseIn {
    type ArrangedAccounts = SwapRouterBaseInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, input_token_account, input_token_mint, token_program, token_program2022, memo_program] =
            accounts
        else {
            return None;
        };

        Some(SwapRouterBaseInInstructionAccounts {
            payer: payer.pubkey,
            input_token_account: input_token_account.pubkey,
            input_token_mint: input_token_mint.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
