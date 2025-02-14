use anchor_lang::prelude::*;

#[program]
pub mod lock_selling {
    use super::*;

    pub fn vote_lock_selling(ctx: Context<VoteLockSelling>, vote: bool) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        let voter = &ctx.accounts.voter;

        // Ensure the voter has governance power
        require!(
            voter.holdings > 0,
            LockSellingError::NoVotingPower
        );

        if vote {
            governance.lock_votes += voter.holdings;
        } else {
            governance.unlock_votes += voter.holdings;
        }

        // Check if lock should be applied
        if governance.lock_votes as f64
            >= (governance.total_supply as f64 * governance.majority_threshold)
        {
            governance.selling_locked = true;
            governance.lock_end_timestamp = Clock::get()?.unix_timestamp + governance.min_lock_time;
        }

        // Check if unlock should be applied
        if governance.unlock_votes as f64
            >= (governance.total_supply as f64 * governance.majority_threshold)
        {
            require!(
                Clock::get()?.unix_timestamp >= governance.lock_end_timestamp,
                LockSellingError::CannotUnlockYet
            );
            governance.selling_locked = false;
            governance.lock_votes = 0;
            governance.unlock_votes = 0;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VoteLockSelling<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(mut)]
    pub governance: Account<'info, Governance>,
}

#[account]
pub struct Governance {
    pub selling_locked: bool, // Whether selling is currently locked
    pub lock_votes: u64,      // Total votes in favor of locking
    pub unlock_votes: u64,    // Total votes in favor of unlocking
    pub total_supply: u64,    // Total token supply for governance calculations
    pub majority_threshold: f64, // % of votes required to make a decision (e.g., 0.6 for 60%)
    pub lock_end_timestamp: i64, // Unix timestamp when selling can be unlocked
    pub min_lock_time: i64,   // Minimum lock period (in seconds)
}

#[account]
pub struct Voter {
    pub holdings: u64, // Number of tokens held (governance power)
}

#[error_code]
pub enum LockSellingError {
    #[msg("User has no governance power to vote.")]
    NoVotingPower,
    #[msg("Cannot unlock selling yet; minimum lock period has not passed.")]
    CannotUnlockYet,
}
