use anchor_lang::prelude::*;
use solana_program::clock::Clock;

#[program]
pub mod governance {
    use super::*;

    /// Cast a vote in favor or against a proposal
    pub fn cast_vote(ctx: Context<CastVote>, in_favor: bool) -> Result<()> {
        let governance_account = &mut ctx.accounts.governance_account;

        if in_favor {
            governance_account.votes_in_favor += 1;
        } else {
            governance_account.votes_against += 1;
        }

        msg!(
            "Vote Casted: In Favor: {}, Against: {}",
            governance_account.votes_in_favor,
            governance_account.votes_against
        );

        Ok(())
    }

    /// Lock token selling if the majority approves
    pub fn lock_selling(ctx: Context<LockSelling>) -> Result<()> {
        let governance_account = &mut ctx.accounts.governance_account;
        let lock_account = &mut ctx.accounts.lock_account;

        let total_votes = governance_account.votes_in_favor + governance_account.votes_against;

        // Ensure there are votes before calculating percentage
        if total_votes == 0 {
            return Err(GovernanceError::NotEnoughVotesToLock.into());
        }

        let approval_percentage =
            (governance_account.votes_in_favor as f64 / total_votes as f64) * 100.0;

        require!(
            approval_percentage >= 60.0,
            GovernanceError::NotEnoughVotesToLock
        );

        require!(
            !lock_account.is_locked,
            GovernanceError::AlreadyLocked
        );

        lock_account.is_locked = true;
        lock_account.lock_start_time = Clock::get()?.unix_timestamp;
        lock_account.lock_duration = 604800; // Minimum lock duration: 1 week

        msg!(
            "Token Selling Locked! Lock Duration: {} seconds",
            lock_account.lock_duration
        );

        Ok(())
    }

    /// Unlock token selling if the majority approves an emergency unlock
    pub fn emergency_unlock(ctx: Context<EmergencyUnlock>) -> Result<()> {
        let governance_account = &mut ctx.accounts.governance_account;
        let lock_account = &mut ctx.accounts.lock_account;

        let total_votes = governance_account.votes_in_favor + governance_account.votes_against;

        // Ensure there are votes before calculating percentage
        if total_votes == 0 {
            return Err(GovernanceError::NotEnoughVotesToUnlock.into());
        }

        let approval_percentage =
            (governance_account.votes_against as f64 / total_votes as f64) * 100.0;

        require!(
            approval_percentage >= 60.0,
            GovernanceError::NotEnoughVotesToUnlock
        );

        lock_account.is_locked = false;
        lock_account.lock_start_time = 0;
        lock_account.lock_duration = 0;

        msg!("Token Selling Unlocked by Majority Vote!");

        Ok(())
    }
}

/// **Context for Casting a Vote**
#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [b"governance"],
        bump
    )]
    pub governance_account: Account<'info, Governance>,
}

/// **Context for Locking Token Selling**
#[derive(Accounts)]
pub struct LockSelling<'info> {
    #[account(
        mut,
        seeds = [b"governance"],
        bump
    )]
    pub governance_account: Account<'info, Governance>,

    #[account(
        mut,
        seeds = [b"lock_account"],
        bump
    )]
    pub lock_account: Account<'info, LockAccount>,
}

/// **Context for Emergency Unlocking**
#[derive(Accounts)]
pub struct EmergencyUnlock<'info> {
    #[account(
        mut,
        seeds = [b"governance"],
        bump
    )]
    pub governance_account: Account<'info, Governance>,

    #[account(
        mut,
        seeds = [b"lock_account"],
        bump
    )]
    pub lock_account: Account<'info, LockAccount>,
}

/// **Governance Account Struct**
#[account]
pub struct Governance {
    pub votes_in_favor: u64,
    pub votes_against: u64,
}

/// **Lock Account Struct**
#[account]
pub struct LockAccount {
    pub is_locked: bool,
    pub lock_start_time: i64,
    pub lock_duration: i64, // Lock duration in seconds (Min: 1 week, Max: 3 months)
}

/// **Governance Errors**
#[error_code]
pub enum GovernanceError {
    #[msg("Not enough votes to lock selling.")]
    NotEnoughVotesToLock,
    #[msg("Not enough votes to unlock selling.")]
    NotEnoughVotesToUnlock,
    #[msg("Selling is already locked.")]
    AlreadyLocked,
}
