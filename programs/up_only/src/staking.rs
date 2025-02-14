
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[program]
pub mod staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let staking_pool = &mut ctx.accounts.staking_pool;
        let user_account = &mut ctx.accounts.user_stake_account;

        require!(amount > 0, StakingError::InvalidStakeAmount);

        // Transfer tokens from user to staking pool
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.pool_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update user staking info
        user_account.amount_staked += amount;
        user_account.last_stake_time = Clock::get()?.unix_timestamp;

        // Update pool total
        staking_pool.total_staked += amount;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let staking_pool = &mut ctx.accounts.staking_pool;
        let user_account = &mut ctx.accounts.user_stake_account;

        require!(amount > 0, StakingError::InvalidStakeAmount);
        require!(
            user_account.amount_staked >= amount,
            StakingError::InsufficientStakedBalance
        );

        // Calculate rewards
        let elapsed_time = Clock::get()?.unix_timestamp - user_account.last_stake_time;
        let rewards = (elapsed_time as u64 / 86400) * (amount / 100); // 1% daily reward

        // Transfer staked tokens + rewards back to user
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.pool_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.pool_authority.to_account_info(),
                },
            ),
            amount + rewards,
        )?;

        // Update user staking info
        user_account.amount_staked -= amount;
        user_account.last_stake_time = Clock::get()?.unix_timestamp;

        // Update pool total
        staking_pool.total_staked -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_stake_account: Account<'info, UserStakeInfo>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_stake_account: Account<'info, UserStakeInfo>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"staking_pool"],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,

    #[account(
        seeds = [b"staking_pool_authority"],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingPool {
    pub total_staked: u64,
}

#[account]
pub struct UserStakeInfo {
    pub amount_staked: u64,
    pub last_stake_time: i64,
}

#[error_code]
pub enum StakingError {
    #[msg("Stake amount must be greater than zero.")]
    InvalidStakeAmount,

    #[msg("Insufficient staked balance for withdrawal.")]
    InsufficientStakedBalance,
}
