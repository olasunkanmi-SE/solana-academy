use anchor_lang::{prelude::*, solana_program::system_instruction};

#[derive(Accounts)]
pub struct TransferSol<'info> {
    /// CHECK: This is the account that will send SOL
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is the account that will receive SOL
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is the system program
    pub system_program: AccountInfo<'info>,
}

pub trait SolTransfer {
    fn transfer(&self, amount: u64) -> Result<()>;
}

impl<'info> SolTransfer for TransferSol<'info> {
    fn transfer(&self, amount: u64) -> Result<()> {
        let from_account_key = self.from.key;
        let to_account_key = self.to.key;
        let transfer_instruction =
            system_instruction::transfer(from_account_key, to_account_key, amount);
        let account_infos = [
            self.from.clone(),
            self.to.clone(),
            self.system_program.to_account_info(),
        ];

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &account_infos,
            &[],
        )?;

        Ok(())
    }
}

pub fn transfer_sol<T: SolTransfer>(accounts: &T, amount: u64) -> Result<()> {
    accounts.transfer(amount)?;
    Ok(())
}
