use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use std::str::FromStr;

use crate::state::random_value;

pub const REVEALER: &str = "2xhBxVVuXkdq2MRKerE9mr2s1szfHSedy21MVqf8gPoM";

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account()]
    pub random_value: Account<'info, random_value::RandomValue>,
    #[account(mut, address = Pubkey::from_str(REVEALER).unwrap())]
    pub revealer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reveal>, result: u64) -> Result<()> {
    let random_value = &mut ctx.accounts.random_value;
    random_value.processed = true;
    random_value.result = result;

    Ok(())
}
