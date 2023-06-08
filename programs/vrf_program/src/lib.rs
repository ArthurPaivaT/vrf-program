use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("8xZUNqKPrgs6sQ3YE1dvT5rcmUvBWkGFyCB6UaJinBeG");

#[program]
pub mod vrf_program {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized {}", ctx.program_id.key());
        Ok(())
    }

    pub fn commit(ctx: Context<Commit>, min: u32, max: u32) -> Result<()> {
        instructions::commit::handler(ctx, min, max)
    }

    pub fn reveal(ctx: Context<Reveal>, result: u32) -> Result<()> {
        instructions::reveal::handler(ctx, result)
    }

    pub fn recommit(ctx: Context<Recommit>) -> Result<()> {
        instructions::recommit::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
}
