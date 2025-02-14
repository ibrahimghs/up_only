use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub mod token_creation;
pub mod staking;
pub mod governance;
pub mod trading;
pub mod lock_selling;

pub use crate::token_creation::create_token;
pub use crate::staking::stake;
pub use crate::governance::cast_vote;
pub use crate::trading::{buy_token, sell_token};
pub use crate::lock_selling::{vote_to_lock, emergency_unlock};

declare_id!("YourProgramPublicKeyHere");

#[program]
pub mod up_only {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        token_creation::create_token(ctx, name, symbol, decimals) // ✅ Correct function call
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        staking::stake(ctx, amount) // ✅ Correct function call
    }

    pub fn cast_vote(ctx: Context<governance::CastVote>, in_favor: bool) -> Result<()> {
        governance::cast_vote(ctx, in_favor)
    }    

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        trading::buy_token(ctx, amount) // ✅ Correct function call
    }

    pub fn sell_token(ctx: Context<SellToken>, amount: u64) -> Result<()> {
        trading::sell_token(ctx, amount) // ✅ Correct function call
    }

    pub fn vote_to_lock(ctx: Context<VoteToLock>, token_mint: Pubkey, duration_weeks: u8) -> Result<()> {
        lock_selling::vote_to_lock(ctx, token_mint, duration_weeks) // ✅ Correct function call
    }

    pub fn emergency_unlock(ctx: Context<EmergencyUnlock>, token_mint: Pubkey) -> Result<()> {
        lock_selling::emergency_unlock(ctx, token_mint) // ✅ Correct function call
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init_if_needed, payer = payer, space = 8 + 128)]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub staker_count: u32, // ✅ Tracks total stakers
    pub bump: u8,          // ✅ Required for PDAs
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Governance {
    pub votes: i64,
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellToken<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteToLock<'info> {
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyUnlock<'info> {
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
    pub system_program: Program<'info, System>,
}
