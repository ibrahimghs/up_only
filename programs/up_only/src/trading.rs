use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

#[program]
pub mod trading {
    use super::*;

    pub fn sell_token(ctx: Context<SellToken>, amount: u64) -> Result<()> {
        let token_lock = &ctx.accounts.token_lock;
        let current_time = Clock::get()?.unix_timestamp;

        // Prevent selling if locked
        require!(
            !token_lock.is_locked || current_time >= token_lock.lock_until,
            TradingError::TokenLocked
        );

        // Perform token transfer (sale)
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellToken<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(has_one = seller)]
    pub token_lock: Account<'info, TokenLock>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum TradingError {
    #[msg("Token selling is currently locked.")]
    TokenLocked,
}
