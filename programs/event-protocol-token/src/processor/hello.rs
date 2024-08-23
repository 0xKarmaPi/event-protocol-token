use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct HelloCtx {}

pub fn hello(_ctx: Context<HelloCtx>) -> Result<()> {
    msg!("Hello, world!");
    Ok(())
}
