use anchor_lang::prelude::*;
use anchor_lang::Accounts;

use crate::state::random_value;

#[derive(Accounts)] // TODO transfer some sol to the revealer on commit
pub struct Commit<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<random_value::RandomValue>())]
    pub random_value: Account<'info, random_value::RandomValue>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<Commit>, min: u32, max: u32) -> Result<()> {
    {
        let random_value = &mut ctx.accounts.random_value;
        random_value.processed = false;
        random_value.result = 0;
        random_value.min = min;
        random_value.max = max;
        random_value.commits = 0;
        random_value.committer = ctx.accounts.user.key();
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
