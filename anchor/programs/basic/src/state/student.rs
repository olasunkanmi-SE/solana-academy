use anchor_lang::prelude::*;

#[account]
pub struct Student {
    pub student_id: u64,
    pub student_nft: Pubkey,
    pub enrolled_classes: Vec<String>,
    pub owned_books: Vec<String>,
}
