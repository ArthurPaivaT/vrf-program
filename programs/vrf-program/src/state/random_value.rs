use anchor_lang::prelude::*;

#[repr(C)]
#[account]

pub struct RandomValue {
    pub min: u32,
    pub max: u32,
    pub result: u32,
    pub processed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User doesn't have tickets")]
    WinnerNotValid,
}
