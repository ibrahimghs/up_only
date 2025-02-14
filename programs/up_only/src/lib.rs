
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::governance::Governance;
use crate::staking::StakingPool;
mod token_creation;
mod governance;
mod staking;
mod trading;

#[program]
mod up_only {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String) -> Result<()> {
        token_creation::create_token(ctx, name, symbol)
    }

    pub fn cast_vote(ctx: Context<CastVote>, in_favor: bool) -> Result<()> {
        governance::cast_vote(ctx, in_favor)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        staking::stake(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init_if_needed, payer = payer, space = 8 + 128)]
    pub token_account: Account<'info, CustomTokenAccount>,
}

#[account]
pub struct CustomTokenAccount {
    pub name: String,
    pub symbol: String,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
}
