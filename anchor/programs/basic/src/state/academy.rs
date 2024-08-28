use anchor_lang::prelude::*;

#[account]
pub struct Academy {
    pub name: String,
    pub admin: Pubkey,
    pub course_count: u64,
}

impl Academy {
    pub const LEN: usize = 8 + 32 + 32 + 8;
}
