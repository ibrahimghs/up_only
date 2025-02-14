use anchor_lang::prelude::*;

#[program]
pub mod lock_selling {
    use super::*;

    /// Casts a vote to lock or unlock token selling
    pub fn vote_lock_selling(ctx: Context<VoteLockSelling>, vote: bool) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let voter_account = &ctx.accounts.voter_account;

        // Ensure the voter has governance power
        require!(
            voter_account.holdings > 0,
            LockSellingError::NoVotingPower
        );

        // Apply vote weight based on governance power
        if vote {
            governance.lock_votes += voter_account.holdings;
        } else {
            governance.unlock_votes += voter_account.holdings;
        }

        // **Check if selling should be locked**
        let lock_threshold = (governance.total_supply as f64 * governance.majority_threshold) as u64;
        if governance.lock_votes >= lock_threshold {
            governance.selling_locked = true;
            governance.lock_end_timestamp = Clock::get()?.unix_timestamp + governance.min_lock_time;
            msg!("Selling has been locked by majority vote.");
        }

        // **Check if selling should be unlocked**
        let unlock_threshold = (governance.total_supply as f64 * governance.majority_threshold) as u64;
        if governance.unlock_votes >= unlock_threshold {
            require!(
                Clock::get()?.unix_timestamp >= governance.lock_end_timestamp,
                LockSellingError::CannotUnlockYet
            );
            governance.selling_locked = false;
            governance.lock_votes = 0;
            governance.unlock_votes = 0;
            msg!("Selling has been unlocked by majority vote.");
        }

        Ok(())
    }
}

/// **Context for Voting on Selling Lock**
#[derive(Accounts)]
pub struct VoteLockSelling<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(mut)]
    pub voter_account: Account<'info, Voter>,

    #[account(
        mut,
        seeds = [b"governance"],
        bump
    )]
    pub governance: Account<'info, Governance>,
}

/// **Governance Account (Manages Voting and Locks)**
#[account]
pub struct Governance {
    pub selling_locked: bool,      // Whether selling is currently locked
    pub lock_votes: u64,           // Total votes in favor of locking
    pub unlock_votes: u64,         // Total votes in favor of unlocking
    pub total_supply: u64,         // Total token supply for governance calculations
    pub majority_threshold: f64,   // % of votes required to make a decision (e.g., 0.6 for 60%)
    pub lock_end_timestamp: i64,   // Unix timestamp when selling can be unlocked
    pub min_lock_time: i64,        // Minimum lock period (in seconds)
}

/// **Voter Account (Tracks Governance Power)**
#[account]
pub struct Voter {
    pub holdings: u64, // Number of tokens held (governance power)
}

/// **Errors for Lock Selling Mechanism**
#[error_code]
pub enum LockSellingError {
    #[msg("User has no governance power to vote.")]
    NoVotingPower,
    #[msg("Cannot unlock selling yet; minimum lock period has not passed.")]
    CannotUnlockYet,
}
