use anchor_lang::prelude::*;

#[account]
pub struct Course {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub tuition_fee: u64,
    pub enrollment_count: u64,
}

impl Course {
    pub const LEN: usize = 8 + 8 + 32 + 64 + 8 + 8 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CourseData {
    pub name: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub tuition_fee: u64,
}
