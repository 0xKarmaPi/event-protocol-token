use anchor_lang::prelude::*;

use crate::states::Master;

#[derive(Accounts)]
pub struct InitMasterCtx<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"master"],
        bump,
        space = 8 + Master::INIT_SPACE,
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMasterCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn init_master(
    ctx: Context<InitMasterCtx>,
    admin: Pubkey,
    start_time: u64,
    end_time: u64,
    foundation_address: Pubkey,
    sale_contract_vault_owner_pda: Pubkey,
) -> Result<()> {
    let master = &mut ctx.accounts.master;

    master.owner = ctx.accounts.signer.key();
    master.admin = admin;
    master.start_time = start_time;
    master.end_time = end_time;
    master.foundation_address = foundation_address;
    master.sale_contract_vault_owner_pda = sale_contract_vault_owner_pda;

    msg!("Init master");
    Ok(())
}

pub fn update_owner(ctx: Context<UpdateMasterCtx>, new_owner: Pubkey) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.admin = new_owner;

    msg!("Owner updated: {}", new_owner);
    Ok(())
}

pub fn update_admin(ctx: Context<UpdateMasterCtx>, new_admin: Pubkey) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.admin = new_admin;

    msg!("Admin updated: {}", new_admin);
    Ok(())
}

pub fn update_foundation_wallet(
    ctx: Context<UpdateMasterCtx>,
    new_foundation_wallet: Pubkey,
) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.foundation_address = new_foundation_wallet;

    msg!("Foundation wallet updated: {}", new_foundation_wallet);
    Ok(())
}

pub fn update_sale_contract_vault_owner_pda(
    ctx: Context<UpdateMasterCtx>,
    new_sale_contract_vault_owner_pda: Pubkey,
) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.sale_contract_vault_owner_pda = new_sale_contract_vault_owner_pda;

    msg!(
        "Sale contract vault owner PDA updated: {}",
        new_sale_contract_vault_owner_pda
    );
    Ok(())
}

pub fn update_start_time(ctx: Context<UpdateMasterCtx>, new_start_time: u64) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.start_time = new_start_time;

    msg!("Start time updated: {}", new_start_time);
    Ok(())
}

pub fn update_end_time(ctx: Context<UpdateMasterCtx>, new_end_time: u64) -> Result<()> {
    let master = &mut ctx.accounts.master;

    require_keys_eq!(master.owner, ctx.accounts.signer.key());

    master.end_time = new_end_time;

    msg!("End time updated: {}", new_end_time);
    Ok(())
}
