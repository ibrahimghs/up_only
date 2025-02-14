use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::clock::Clock;

#[program]
pub mod staking {
    use super::*;

    /// Function to stake tokens into the staking pool
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_pool;
        let clock = Clock::get()?;

        // Ensure user has enough tokens to stake
        require!(
            ctx.accounts.user_token_account.amount >= amount,
            StakingError::InsufficientFunds
        );

        // Transfer tokens from the user's account to the staking pool
        let seeds = &[b"staking_pool", ctx.accounts.staking_pool.key().as_ref(), &[ctx.accounts.staking_pool.bump]];
        let signer_seeds = &[&seeds[..]];
        
        token::transfer(
            ctx.accounts.transfer_context().with_signer(signer_seeds),
            amount,
        )?;

        // Record staking details
        let staker_account = &mut ctx.accounts.staker_account;
        staker_account.staker = ctx.accounts.user.key();
        staker_account.amount_staked += amount;
        staker_account.stake_start_time = clock.unix_timestamp;

        // Update total staked in the pool
        staking_account.total_staked += amount;

        msg!(
            "User {} staked {} tokens. Total staked: {}",
            ctx.accounts.user.key(),
            amount,
            staking_account.total_staked
        );

        Ok(())
    }

    /// Function to unstake tokens from the staking pool
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let staking_pool = &mut ctx.accounts.staking_pool;
        let staker_account = &mut ctx.accounts.staker_account;
        let clock = Clock::get()?;

        require!(
            staker_account.amount_staked > 0,
            StakingError::NoStakeFound
        );

        // Calculate staking duration
        let staking_duration = clock.unix_timestamp - staker_account.stake_start_time;

        // Check for lock period (minimum staking period of 7 days)
        require!(
            staking_duration >= 604800, // 7 days in seconds
            StakingError::StakeLocked
        );

        let amount = staker_account.amount_staked;

        // Transfer tokens back to the user
        let seeds = &[b"staking_pool", ctx.accounts.staking_pool.key().as_ref(), &[ctx.accounts.staking_pool.bump]];
        let signer_seeds = &[&seeds[..]];
        
        token::transfer(
            ctx.accounts.transfer_context().with_signer(signer_seeds),
            amount,
        )?;

        // Reset user stake record
        staker_account.amount_staked = 0;
        staker_account.stake_start_time = 0;

        // Update total staked amount in pool
        staking_pool.total_staked -= amount;

        msg!(
            "User {} unstaked {} tokens. Total staked: {}",
            ctx.accounts.user.key(),
            amount,
            staking_pool.total_staked
        );

        Ok(())
    }
}

/// **Accounts for Staking**
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,

    #[account(mut, has_one = user)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer = user, space = 8 + 32 + 8 + 8)]
    pub staker_account: Account<'info, StakerAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// **Accounts for Unstaking**
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,

    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,

    #[account(mut, has_one = user)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut, has_one = user)]
    pub staker_account: Account<'info, StakerAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// **Staking Pool Struct**
#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub bump: u8, // Added to store PDA bump seed
}

/// **User's Stake Account**
#[account]
pub struct StakerAccount {
    pub staker: Pubkey,
    pub amount_staked: u64,
    pub stake_start_time: i64,
}

/// **Staking Errors**
#[error_code]
pub enum StakingError {
    #[msg("Insufficient funds to stake.")]
    InsufficientFunds,
    #[msg("No staked tokens found.")]
    NoStakeFound,
    #[msg("Tokens are locked. You must wait for the minimum staking period to withdraw.")]
    StakeLocked,
}

/// **Transfer Context Implementation**
impl<'info> Stake<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.staking_token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> Unstake<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.staking_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.staking_pool.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
