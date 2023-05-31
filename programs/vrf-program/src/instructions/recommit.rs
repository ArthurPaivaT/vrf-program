use anchor_lang::prelude::*;
use anchor_lang::Accounts;

use crate::state::random_value;

#[derive(Accounts)]
pub struct Recommit<'info> {
    #[account(mut)]
    pub random_value: Account<'info, random_value::RandomValue>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Recommit>) -> Result<()> {
    msg!("Recommiting {}");

    if ctx.accounts.random_value.processed == false {
        return Err(ErrorCode::NotRevealedYet.into());
    }

    ctx.accounts.random_value.processed = false;
    ctx.accounts.random_value.result = 0;
    ctx.accounts.random_value.commits = ctx.accounts.random_value.commits + 1;

    msg!("key {}", ctx.accounts.random_value.key());

    msg!(
        "commit number {} created",
        ctx.accounts.random_value.commits,
    );

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Random Value not revealed yet")]
    NotRevealedYet,
}
