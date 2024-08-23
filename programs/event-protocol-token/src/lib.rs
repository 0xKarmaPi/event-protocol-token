use anchor_lang::prelude::*;

declare_id!("7AKNwCkSvf1ACnCxJBHZMvinFhnmrTCRvhQDiTqEmr7h");

pub mod processor;
pub mod states;
pub mod errors;

use crate::processor::*;

#[program]
pub mod event_protocol_token {
    use super::*;

    pub fn hello(ctx: Context<HelloCtx>) -> Result<()> {
        processor::hello::hello(ctx)
    }

    pub fn init_master(ctx: Context<InitMasterCtx>, admin: Pubkey, start_time: u64, end_time: u64, foundation_address: Pubkey, sale_contract_vault_owner_pda: Pubkey) -> Result<()> {
        processor::master::init_master(ctx, admin, start_time, end_time, foundation_address, sale_contract_vault_owner_pda)
    }

    pub fn init_vault_event_token(ctx: Context<InitVaultEventTokenCtx>) -> Result<()> {
        processor::vault::init_vault_event_token(ctx)
    }

    pub fn deposit_event_token(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
        processor::vault::deposit_event_token(ctx, amount)
    }

    pub fn withdraw_event_token(ctx: Context<TransferTokenCtx>, amount: u64, total: bool) -> Result<()> {
        processor::vault::withdraw_event_token(ctx, amount, total)
    }

    pub fn split_event_token(ctx: Context<SplitEventTokenCtx>) -> Result<()> {
        processor::split::split_event_token(ctx)
    }

    pub fn update_owner(ctx: Context<UpdateMasterCtx>, new_owner: Pubkey) -> Result<()> {
        processor::master::update_owner(ctx, new_owner)
    }

    pub fn update_admin(ctx: Context<UpdateMasterCtx>, new_admin: Pubkey) -> Result<()> {
        processor::master::update_admin(ctx, new_admin)
    }

    pub fn update_foundation_wallet(ctx: Context<UpdateMasterCtx>, new_foundation_wallet: Pubkey) -> Result<()> {
        processor::master::update_foundation_wallet(ctx, new_foundation_wallet)
    }

    pub fn update_sale_contract_vault_owner_pda(ctx: Context<UpdateMasterCtx>, new_sale_contract_vault_owner_pda: Pubkey) -> Result<()> {
        processor::master::update_sale_contract_vault_owner_pda(ctx, new_sale_contract_vault_owner_pda)
    }

    pub fn update_start_time(ctx: Context<UpdateMasterCtx>, new_start_time: u64) -> Result<()> {
        processor::master::update_start_time(ctx, new_start_time)
    }

    pub fn update_end_time(ctx: Context<UpdateMasterCtx>, new_end_time: u64) -> Result<()> {
        processor::master::update_end_time(ctx, new_end_time)
    }
}