use anchor_lang::prelude::*;

declare_id!("79zUy3DKMd4xfFpjnzBVyxaaEzD6vV6aAVzHS8U7JS1Y");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
