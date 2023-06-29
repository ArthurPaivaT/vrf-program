use anchor_lang::prelude::*;

declare_id!("6P6QEkEsmVaUUmxHkahZkSvQnihJ2nHPFYK5gpT6tm7R");

pub mod instructions;
pub mod state;

#[program]
pub mod cpi_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized {}", ctx.program_id.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
