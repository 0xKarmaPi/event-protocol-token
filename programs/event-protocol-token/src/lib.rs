use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TokenAccount, transfer, Transfer, Mint}};

declare_id!("7AKNwCkSvf1ACnCxJBHZMvinFhnmrTCRvhQDiTqEmr7h");

pub mod states;
pub mod errors;

use crate::errors::CustomError;
use crate::states::Master;

#[program]
pub mod event_protocol_token {
    use super::*;

    pub fn hello(_ctx: Context<HelloCtx>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }

    pub fn init_master(ctx: Context<InitMasterCtx>, admin: Pubkey, start_time: u64, end_time: u64, foundation_address: Pubkey, sale_contract_vault_owner_pda: Pubkey) -> Result<()> {
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

        anchor_spl::token::transfer(cpi_ctx, amount * (10u64.pow(mint_of_event_token.decimals as u32)))?;

        vault_event_token.reload()?;
        msg!("amount transferred to vault event token: {}", vault_event_token.amount);

        Ok(())
    }

    pub fn withdraw_event_token(ctx: Context<TransferTokenCtx>, amount: u64, total: bool) -> Result<()> {
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

        msg!("amount transferred to sales contract: {}", vault_event_token_amount);

        if total == false {
            transfer(withdraw_event_token_ctx, amount * (10u64.pow(mint_of_event_token.decimals as u32)))?;
        } else {
            transfer(withdraw_event_token_ctx, vault_event_token_amount)?;
        }

        Ok(())
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

    pub fn update_foundation_wallet(ctx: Context<UpdateMasterCtx>, new_foundation_wallet: Pubkey) -> Result<()> {
        let master = &mut ctx.accounts.master;

        require_keys_eq!(master.owner, ctx.accounts.signer.key());

        master.foundation_address = new_foundation_wallet;

        msg!("Foundation wallet updated: {}", new_foundation_wallet);
        Ok(())
    }

    pub fn update_sale_contract_vault_owner_pda(ctx: Context<UpdateMasterCtx>, new_sale_contract_vault_owner_pda: Pubkey) -> Result<()> {
        let master = &mut ctx.accounts.master;

        require_keys_eq!(master.owner, ctx.accounts.signer.key());

        master.sale_contract_vault_owner_pda = new_sale_contract_vault_owner_pda;

        msg!("Sale contract vault owner PDA updated: {}", new_sale_contract_vault_owner_pda);
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
}

#[derive(Accounts)]
pub struct HelloCtx {}

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

#[derive(Accounts)]
pub struct InitVaultEventTokenCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    // ===== TOKEN =====
    /// CHECK
    #[account(
        init,
        payer = signer,
        seeds = [b"vault_event_token_owner"],
        bump,
        space = 8,
    )]
    vault_event_token_owner: AccountInfo<'info>,

    mint_of_event_token: Account<'info, Mint>,
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

#[derive(Accounts)]
pub struct SplitEventTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    // ====== TOKEN =====
    /// CHECK
    #[account(
        mut,
        seeds = [b"vault_event_token_owner"],
        bump
    )]
    vault_event_token_owner: AccountInfo<'info>,
    mint_of_event_token: Account<'info, Mint>,
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

