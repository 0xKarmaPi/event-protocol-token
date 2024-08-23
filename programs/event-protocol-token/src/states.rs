use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Master {
    pub owner: Pubkey,
    pub admin: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub foundation_address: Pubkey,
    pub sale_contract_vault_owner_pda: Pubkey,
}
