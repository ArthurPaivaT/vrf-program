use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct Bet {
    pub player: Pubkey,
    pub value: u32,
    pub random_value: Pubkey,
    pub claimed: bool
}
