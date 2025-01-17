use anchor_lang::prelude::*;

declare_id!("6N9BmKR9aNbGCM8auMBQ8ZfRbkgxpWDJJJA4jvd2sk1Y");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub count: u64,
}
