use anchor_lang::prelude::*;
use anchor_lang::Accounts;

use crate::state::bet;

use vrf_program::{self, state::{RandomValue},};

#[derive(Accounts)] // TODO transfer some sol to the revealer on commit
pub struct Claim<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub random_value: Box<Account<'info, RandomValue>>,

    #[account(mut)]
    pub bet: Account<'info, bet::Bet>,

    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<Claim>) -> Result<()> {
    {
        // transfer from user to program
        let random_value = &mut ctx.accounts.random_value;
        let bet = &mut ctx.accounts.bet;

        if random_value.processed && random_value.result < 50 {
            bet.claimed = true

        } else {

        }

    }

    msg!(
        "account is {}",
        ctx.accounts
            .random_value
            .to_account_info()
            .key()
            .to_string()
    );

    msg!(
        "user is {}",
        ctx.accounts.user.to_account_info().key().to_string()
    );

    msg!(
        "system is {}",
        ctx.accounts
            .system_program
            .to_account_info()
            .key()
            .to_string()
    );

    Ok(())
}
