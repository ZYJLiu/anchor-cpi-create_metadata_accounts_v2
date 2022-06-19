use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::{token::{Mint, Token}};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2};

declare_id!("8krpyZVTxvKFcVL7eLaLYBsshRcMh2q1J8beoLzcqDMM");

#[program]
pub mod metadata {
    use super::*;

    pub fn create_mint(ctx: Context<CreateMint>, uri: String, name: String, symbol: String,) -> Result<()> {
        let user = ctx.accounts.user.key();
        
        let (_mint, bump) =
            Pubkey::find_program_address(&["MINT".as_bytes(), ctx.accounts.user.key().as_ref()], ctx.program_id);

        let seeds = &["MINT".as_bytes(), user.as_ref(), &[bump]];
        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        invoke_signed(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.user.key(),
                ctx.accounts.user.key(),
                name,
                symbol,
                uri,
                None,
                0,
                true,
                true,
                None,
                None,
            ),
            account_info.as_slice(),
            &signer
        )?;



        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
        seeds = ["MINT".as_bytes().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint,   
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: 
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub token_metadata_program: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

}
