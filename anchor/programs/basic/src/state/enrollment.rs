use anchor_lang::prelude::*;

#[account]
pub struct Enrollment {
    pub student: Pubkey,
    pub course: Pubkey,
    pub enrolled_at: i64,
    pub completed: bool,
}

impl Enrollment {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}
