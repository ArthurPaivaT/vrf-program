use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use std::str::FromStr;

use crate::state::random_value;

pub const REVEALER: &str = "8L9kkeGCa8vN3BrsfSgHkukk1TQNvWHvn5gRxWBJTzEt";

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account(mut)]
    pub random_value: Account<'info, random_value::RandomValue>,
    #[account(mut, address = Pubkey::from_str(REVEALER).unwrap())]
    pub revealer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reveal>, result: u32) -> Result<()> {
    msg!("Entered {}", result);

    ctx.accounts.random_value.processed = true;
    ctx.accounts.random_value.result = result;

    msg!("key {}", ctx.accounts.random_value.key());

    msg!(
        "rv is {}, {},{}",
        ctx.accounts.random_value.processed,
        ctx.accounts.random_value.commits,
        ctx.accounts.random_value.result
    );

    Ok(())
}
