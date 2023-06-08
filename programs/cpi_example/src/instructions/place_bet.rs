use anchor_lang::prelude::*;
use anchor_lang::Accounts;

use crate::state::bet;

use vrf_program::{self, cpi::accounts::Commit, program::VrfProgram};

#[derive(Accounts)] // TODO transfer some sol to the revealer on commit
pub struct PlaceBet<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<bet::Bet>())]
    pub bet: Account<'info, bet::Bet>,
    pub system_program: Program<'info, System>,

    // cpi
    #[account(mut)]
    pub random_value: Signer<'info>,
    pub vrf_program: Program<'info, VrfProgram>,

    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> PlaceBet<'info> {
    fn commit_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Commit<'info>> {
        CpiContext::new(
            self.vrf_program.to_account_info(),
            Commit {
                random_value: self.random_value.to_account_info(),
                user: self.user.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        )
    }
}


pub fn handler(ctx: Context<PlaceBet>, value: u32) -> Result<()> {
    {
        // transfer from user to program
        // let random_value = ctx.accounts.random_value.to_account_info();
        // let vrf_program = ctx.accounts.vrf_program.to_account_info();

        // create random_value
        vrf_program::cpi::commit(
            ctx.accounts
                .commit_ctx(),
                0, 100
        )?;

        let bet = &mut ctx.accounts.bet;
        bet.value = value;
        bet.claimed = false
    }

    Ok(())
}
