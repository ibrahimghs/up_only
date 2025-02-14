use anchor_lang::prelude::*;

#[program]
pub mod governance {
    use super::*;

    pub fn vote_lock(ctx: Context<VoteLock>, in_favor: bool) -> Result<()> {
        let vote = &mut ctx.accounts.governance_account;

        if in_favor {
            vote.lock_votes += 1;
        } else {
            vote.unlock_votes += 1;
        }

        Ok(())
    }

    pub fn finalize_lock(ctx: Context<FinalizeLock>, lock_until: i64) -> Result<()> {
        let governance = &ctx.accounts.governance_account;

        require!(
            governance.lock_votes as f64 / (governance.lock_votes + governance.unlock_votes) as f64 >= 0.6,
            GovernanceError::NotEnoughVotes
        );

        lock_selling::lock_sales(ctx.accounts.into(), lock_until)?;

        Ok(())
    }

    pub fn finalize_unlock(ctx: Context<FinalizeUnlock>) -> Result<()> {
        let governance = &ctx.accounts.governance_account;

        require!(
            governance.unlock_votes as f64 / (governance.lock_votes + governance.unlock_votes) as f64 >= 0.6,
            GovernanceError::NotEnoughVotes
        );

        lock_selling::unlock_sales(ctx.accounts.into())?;

        Ok(())
    }
}

#[account]
pub struct Governance {
    pub lock_votes: u64,
    pub unlock_votes: u64,
}

#[derive(Accounts)]
pub struct VoteLock<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
}

#[derive(Accounts)]
pub struct FinalizeLock<'info> {
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
}

#[derive(Accounts)]
pub struct FinalizeUnlock<'info> {
    #[account(mut)]
    pub governance_account: Account<'info, Governance>,
}

#[error_code]
pub enum GovernanceError {
    #[msg("Not enough votes to proceed.")]
    NotEnoughVotes,
}
