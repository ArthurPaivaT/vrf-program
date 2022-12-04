use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use solana_program::program::invoke;
use solana_program::system_instruction;

use crate::state::random_value;

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account()]
    pub random_value: Account<'info, random_value::RandomValue>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reveal>, result: u64) -> Result<()> {
    let random_value = &mut ctx.accounts.random_value;
    random_value.processed = true;
    random_value.result = result;

    Ok(())
}
