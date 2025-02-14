use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::token_creation::{self, create_token};
use crate::staking::{self, stake};
use crate::governance::{self, cast_vote};
use crate::trading::{self, buy_token, sell_token};
use crate::lock_selling::{self, vote_to_lock, emergency_unlock};

pub mod token_creation;
pub mod staking;
pub mod governance;
pub mod trading;
pub mod lock_selling;

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
        create_token(ctx, name, symbol, decimals)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake(ctx, amount)
    }

    pub fn cast_vote(ctx: Context<CastVote>, in_favor: bool) -> Result<()> {
        cast_vote(ctx, in_favor)
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        buy_token(ctx, amount)
    }

    pub fn sell_token(ctx: Context<SellToken>, amount: u64) -> Result<()> {
        sell_token(ctx, amount)
    }

    pub fn vote_to_lock(ctx: Context<VoteToLock>, token_mint: Pubkey, duration_weeks: u8) -> Result<()> {
        vote_to_lock(ctx, token_mint, duration_weeks)
    }

    pub fn emergency_unlock(ctx: Context<EmergencyUnlock>, token_mint: Pubkey) -> Result<()> {
        emergency_unlock(ctx, token_mint)
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
