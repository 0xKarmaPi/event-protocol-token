use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token, Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::Master;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct SplitEventTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    // ====== TOKEN =====
    mint_of_event_token: Account<'info, Mint>,
    /// CHECK
    #[account(
        mut,
        seeds = [b"vault_event_token_owner"],
        bump
    )]
    vault_event_token_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"vault_event_token", mint_of_event_token.key().as_ref()],
        bump,
    )]
    vault_event_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_of_event_token, 
        associated_token::authority = foundation_address,
    )]
    foundation_event_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint_of_event_token,
        token::authority = sale_contract_vault_owner_pda,
    )]
    sale_contract_vault_event_token: Account<'info, TokenAccount>,
    /// CHECK
    foundation_address: AccountInfo<'info>,
    /// CHECK 
    sale_contract_address: AccountInfo<'info>,
    /// CHECK
    sale_contract_vault_owner_pda: AccountInfo<'info>,
    // ====== =====

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}

pub fn split_event_token(ctx: Context<SplitEventTokenCtx>) -> Result<()> {
    let clock  = Clock::get()?;
    let master = &ctx.accounts.master;

    let mint_of_event_token = &ctx.accounts.mint_of_event_token;
    let vault_event_token = &ctx.accounts.vault_event_token;
    let amount_event_token = vault_event_token.amount;

    let foundation_address = &ctx.accounts.foundation_address.key();
    let sale_contract_vault_owner_pda = &ctx.accounts.sale_contract_vault_owner_pda.key();

    require_keys_eq!(ctx.accounts.master.admin, ctx.accounts.signer.key());
    require_keys_eq!(master.foundation_address, *foundation_address);
    require_keys_eq!(master.sale_contract_vault_owner_pda, *sale_contract_vault_owner_pda);
    require!(clock.unix_timestamp as u64 >= ctx.accounts.master.start_time, CustomError::StartTimeNotReached);
    require!(amount_event_token >= 1_000_000_000u64.pow(mint_of_event_token.decimals as u32), CustomError::AmountOfEventTokenIsNotEnough);

    let bump = ctx.bumps.vault_event_token_owner;
    let seeds = &[b"vault_event_token_owner".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    // transfer 990.000.000 EVENT token to foundation wallet
    let transfer_foundation_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_event_token.to_account_info(),
            to: ctx.accounts.foundation_event_token.to_account_info(),
            authority: ctx.accounts.vault_event_token_owner.to_account_info(),
        },
        signer,
    );
    msg!("amount transferred to foundation: {}", amount_event_token * 10u64.pow(mint_of_event_token.decimals as u32));
    transfer(transfer_foundation_ctx, 990_000_000 * (mint_of_event_token.decimals as u64))?;

    // transfer 10.000.000 EVENT token to sales contract (IVO contract)
    let transfer_sales_contract_ctx= CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_event_token.to_account_info(),
            to: ctx.accounts.sale_contract_vault_event_token.to_account_info(),
            authority: ctx.accounts.vault_event_token_owner.to_account_info(),
        },
        signer,
    );
    msg!("amount transferred to sales contract: {}", amount_event_token * 10u64.pow(mint_of_event_token.decimals as u32));
    transfer(transfer_sales_contract_ctx, 10_000_000 * (mint_of_event_token.decimals as u64))?;

    Ok(())
}