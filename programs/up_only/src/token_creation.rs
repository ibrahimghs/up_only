
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, Token, TokenAccount, InitializeMint, Transfer, MintTo,
};

#[program]
pub mod token_creation {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, decimals: u8) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;

        // Assign token metadata
        mint.decimals = decimals;
        mint.supply = 0; // Supply is 0 at creation
        mint.freeze_authority = Some(ctx.accounts.authority.key());
        mint.mint_authority = ctx.accounts.authority.key();

        // Assign custom metadata
        token_account.name = name;
        token_account.symbol = symbol;

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 128,
        seeds = [b"token_account", payer.key().as_ref()],
        bump
    )]
    pub token_account: Account<'info, CustomTokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct CustomTokenAccount {
    pub name: String,
    pub symbol: String,
}
