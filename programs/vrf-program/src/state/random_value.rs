use anchor_lang::prelude::*;

/// whitelists are used to control what gems can/can't go into the vault
/// currently 2 types of vault lists are supported: by mint and by creator
/// if the whitelist PDA exists, then the mint/creator is considered accepted
/// if at least 1 whitelist PDA exists total, then all deposit attempts will start getting checked
#[repr(C)]
#[account]

pub struct RandomValue {
    pub min: u64,
    pub max: u64,
    pub result: u64,
    pub processed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("User doesn't have tickets")]
    WinnerNotValid,
}
