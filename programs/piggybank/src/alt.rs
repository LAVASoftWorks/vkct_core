// Anchor-based Solana program for NFT vaults - ALTERNATE VERSION
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

// Declare the program ID
declare_id!("PASTE_THE_ACCOUNT_HERE");

#[program]
pub mod nft_vault {
    use super::*;

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // Make sure user owns the NFT
        let nft_account = &ctx.accounts.nft_account;
        require!(nft_account.amount == 1, VaultError::NotNFTOwner);
        require!(nft_account.owner == ctx.accounts.signer.key(), VaultError::NotNFTOwner);

        // Transfer SPL tokens from vault to the user
        let cpi_accounts = Transfer {
            from:      ctx.accounts.vault_token_account.to_account_info(),
            to:        ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        let signer_seeds = &[b"vault", ctx.accounts.nft_mint.key().as_ref(), &[ctx.accounts.vault_bump]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                &[&signer_seeds[..]],
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: NFT token account must be validated manually
    #[account(mut)]
    pub nft_account: AccountInfo<'info>,
    pub nft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA vault authority ("vault", nft_mint)
    #[account(seeds = [b"vault", nft_mint.key().as_ref()], bump)]
    pub vault_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum VaultError {
    #[msg("User does not own the NFT.")]
    NotNFTOwner,
}
