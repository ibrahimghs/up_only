use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[program]
pub mod trading {
    use super::*;

    /// Allows a user to buy tokens from a seller
    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        let token_mint = &ctx.accounts.token_mint;
        let seller_token_account = &ctx.accounts.seller_token_account;
        let buyer_token_account = &ctx.accounts.buyer_token_account;

        // Ensure seller has enough tokens
        require!(
            seller_token_account.amount >= amount,
            TradingError::InsufficientSupply
        );

        // Transfer tokens from seller to buyer
        token::transfer(
            ctx.accounts
                .transfer_context()
                .with_signer(&[&ctx.accounts.seller.key().as_ref()]),
            amount,
        )?;

        // Update trading pool metrics
        let trading_pool = &mut ctx.accounts.trading_pool;
        trading_pool.total_traded += amount;

        msg!(
            "User {} bought {} tokens from {}",
            ctx.accounts.buyer.key(),
            amount,
            ctx.accounts.seller.key()
        );

        Ok(())
    }

    /// Allows a user to sell tokens into the trading pool if selling is not locked
    pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
        let trading_pool = &mut ctx.accounts.trading_pool;
        let governance_account = &ctx.accounts.governance;

        // Ensure that selling is not locked
        require!(
            !governance_account.selling_locked,
            TradingError::SellingLocked
        );

        // Ensure seller has enough tokens
        require!(
            ctx.accounts.seller_token_account.amount >= amount,
            TradingError::InsufficientBalance
        );

        // Transfer tokens from seller to trading pool
        token::transfer(
            ctx.accounts
                .transfer_context()
                .with_signer(&[&ctx.accounts.seller.key().as_ref()]),
            amount,
        )?;

        // Update trading pool total traded amount
        trading_pool.total_traded += amount;

        msg!(
            "User {} sold {} tokens. Total traded in pool: {}",
            ctx.accounts.seller.key(),
            amount,
            trading_pool.total_traded
        );

        Ok(())
    }
}

/// **Context for Buying Tokens**
#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trading_pool"],
        bump
    )]
    pub trading_pool: Account<'info, TradingPool>,

    pub token_program: Program<'info, Token>,
}

/// **Context for Selling Tokens**
#[derive(Accounts)]
pub struct SellTokens<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"trading_pool"],
        bump
    )]
    pub trading_pool: Account<'info, TradingPool>,

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trading_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"governance"],
        bump
    )]
    pub governance: Account<'info, Governance>,

    pub token_program: Program<'info, Token>,
}

/// **Trading Pool Account**
#[account]
pub struct TradingPool {
    pub total_traded: u64,
}

/// **Governance Account**
#[account]
pub struct Governance {
    pub selling_locked: bool, // Whether selling is locked
}

/// **Trading Errors**
#[error_code]
pub enum TradingError {
    #[msg("Insufficient token supply for purchase.")]
    InsufficientSupply,
    #[msg("Insufficient balance to sell tokens.")]
    InsufficientBalance,
    #[msg("Selling of tokens is currently locked by governance.")]
    SellingLocked,
}

impl<'info> BuyTokens<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.seller_token_account.to_account_info(),
            to: self.buyer_token_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> SellTokens<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.seller_token_account.to_account_info(),
            to: self.trading_token_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
