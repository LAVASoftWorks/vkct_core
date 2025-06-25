#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused_variables)]

/**
 * Anchor-based Solana program for NFT vaults
 * 
 * Author: lava.caballero@gmail.com
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_security_txt::security_txt;

// Note: put here the piggy bank's public key
declare_id!("PASTE_THE_ACCOUNT_HERE");

#[program]
pub mod piggybank {
    use super::*;

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        
        let nft_account = &ctx.accounts.nft_account;
        require!(nft_account.amount == 1, VaultError::NotNFTOwner);
        require!(nft_account.owner == ctx.accounts.signer.key(), VaultError::NotNFTOwner);

        let nft_mint_key = ctx.accounts.nft_mint.key();
        let vault_bump = ctx.bumps.vault_authority;
        let signer_seeds = &[
            b"piggybank",
            nft_mint_key.as_ref(),
            &[vault_bump],
        ];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                &[signer_seeds],
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    
    /// CHECK: The signer / withdrawal issues
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: The token account that holds the NFT
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    
    /// CHECK: The NFT mint address. Used to derive the vault PDA.
    pub nft_mint: Account<'info, Mint>,
    
    /// CHECK: The user’s destination account.
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Vault’s SPL token storage account.
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: PDA vault authority ("vault", nft_mint)
    #[account(seeds = [b"piggybank", nft_mint.key().as_ref()], bump)]
    pub vault_authority: AccountInfo<'info>,
    
    /// CHECK: SPL Token program.
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum NFTvalidationError {
    #[msg("The piggy bank is not a valid NFT.")]
    NotanNFT,
}

#[error_code]
pub enum VaultError {
    #[msg("The sender does not own the NFT.")]
    NotNFTOwner,
}

security_txt! {
   Contact: "security@example.com",
   Acknowledgements: "https://example.com/security-acknowledgements"
}
