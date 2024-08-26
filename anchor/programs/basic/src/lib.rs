pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("79zUy3DKMd4xfFpjnzBVyxaaEzD6vV6aAVzHS8U7JS1Y");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
