use anchor_lang::prelude::*;

#[repr(C)]
#[account]

pub struct RandomValue {
    pub min: u32,
    pub max: u32,
    pub result: u32,
    pub processed: bool,
    pub commits: u32,
}
