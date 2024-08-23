use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::states::Master;

#[derive(Accounts)]
pub struct InitVaultEventTokenCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    // ===== TOKEN =====
    mint_of_event_token: Account<'info, Mint>,
    /// CHECK
    #[account(
        init,
        payer = signer,
        seeds = [b"vault_event_token_owner"],
        bump,
        space = 8,
    )]
    vault_event_token_owner: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"vault_event_token", mint_of_event_token.key().as_ref()],
        bump,
        token::mint = mint_of_event_token,
        token::authority = vault_event_token_owner,
    )]
    vault_event_token: Account<'info, TokenAccount>,
    // ==== =====
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferTokenCtx<'info> {
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
        bump,
    )]
    vault_event_token_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"vault_event_token", mint_of_event_token.key().as_ref()],
        bump,
    )]
    vault_event_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_of_event_token,
        associated_token::authority = signer,
    )]
    sender_event_token_account: Account<'info, TokenAccount>,
    // ====== =====
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn init_vault_event_token(ctx: Context<InitVaultEventTokenCtx>) -> Result<()> {
    require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key());

    msg!("Init vault event token");
    Ok(())
}

pub fn deposit_event_token(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
    let mint_of_event_token = &ctx.accounts.mint_of_event_token;
    let vault_event_token = &mut ctx.accounts.vault_event_token;

    let transfer_ix = Transfer {
        from: ctx.accounts.sender_event_token_account.to_account_info(),
        to: vault_event_token.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_ix);

    anchor_spl::token::transfer(
        cpi_ctx,
        amount * (10u64.pow(mint_of_event_token.decimals as u32)),
    )?;

    vault_event_token.reload()?;
    msg!(
        "amount transferred to vault event token: {}",
        vault_event_token.amount
    );

    Ok(())
}

pub fn withdraw_event_token(
    ctx: Context<TransferTokenCtx>,
    amount: u64,
    total: bool,
) -> Result<()> {
    // only owner can withdraw EVENT token
    require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key());

    let mint_of_event_token = &ctx.accounts.mint_of_event_token;
    let sender_event_token_account = &ctx.accounts.sender_event_token_account;
    let vault_event_token = &ctx.accounts.vault_event_token;
    let vault_event_token_owner = &ctx.accounts.vault_event_token_owner;

    let vault_event_token_amount = vault_event_token.amount;

    let bump = ctx.bumps.vault_event_token_owner;
    let seeds = &[b"vault_event_token_owner".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let withdraw_event_token_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: vault_event_token.to_account_info(),
            to: sender_event_token_account.to_account_info(),
            authority: vault_event_token_owner.to_account_info(),
        },
        signer,
    );

    msg!(
        "amount transferred to sales contract: {}",
        vault_event_token_amount
    );

    if total == false {
        transfer(
            withdraw_event_token_ctx,
            amount * (10u64.pow(mint_of_event_token.decimals as u32)),
        )?;
    } else {
        transfer(withdraw_event_token_ctx, vault_event_token_amount)?;
    }

    Ok(())
}
