use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};

#[program]
pub mod lock_selling {
    use super::*;

    pub fn lock_sales(ctx: Context<LockSales>, lock_until: i64) -> Result<()> {
        let lock_state = &mut ctx.accounts.token_lock;
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            lock_until > current_time + (7 * 24 * 60 * 60) && lock_until <= current_time + (90 * 24 * 60 * 60),
            LockError::InvalidLockPeriod
        );

        lock_state.lock_until = lock_until;
        lock_state.is_locked = true;

        Ok(())
    }

    pub fn unlock_sales(ctx: Context<UnlockSales>) -> Result<()> {
        let lock_state = &mut ctx.accounts.token_lock;
        lock_state.is_locked = false;

        Ok(())
    }
}

#[account]
pub struct TokenLock {
    pub lock_until: i64,  // Timestamp when the lock expires
    pub is_locked: bool,  // Whether sales are currently locked
}

#[derive(Accounts)]
pub struct LockSales<'info> {
    #[account(mut)]
    pub governance_account: Signer<'info>,  // Only governance can lock selling
    #[account(init_if_needed, payer = governance_account, space = 8 + 16)]
    pub token_lock: Account<'info, TokenLock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockSales<'info> {
    #[account(mut)]
    pub governance_account: Signer<'info>,  // Only governance can unlock selling
    #[account(mut)]
    pub token_lock: Account<'info, TokenLock>,
}

#[error_code]
pub enum LockError {
    #[msg("Lock period must be between 1 week and 3 months.")]
    InvalidLockPeriod,
}
