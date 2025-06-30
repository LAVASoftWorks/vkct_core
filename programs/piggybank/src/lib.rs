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
use anchor_spl::associated_token::get_associated_token_address; // You may need to add this dependency.
use solana_security_txt::security_txt;

// Note: put here the piggy bank's public key
declare_id!("VaU1t11111111111111111111111111111111111111");

#[program]
pub mod piggybank {
    use super::*;

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let signer = &ctx.accounts.signer;
        let nft_mint = &ctx.accounts.nft_mint;
        let token_mint = &ctx.accounts.token_mint;

        // 1. Derive user's NFT token account (ATA)
        let user_nft_ata = get_associated_token_address(&signer.key(), &nft_mint.key());

        // 2. Confirm signer owns the NFT
        let nft_account = &ctx.accounts.user_nft_account;
        require!(
            nft_account.owner == signer.key() && nft_account.amount == 1,
            VaultError::NotNFTOwner
        );
        require!(
            nft_account.mint == nft_mint.key(),
            VaultError::NotNFTOwner
        );

        // 3. Derive vault authority PDA and vault token ATA
        let (vault_authority, vault_bump) = Pubkey::find_program_address(
            &[b"PoliCromixPiggyBankV2", nft_mint.key().as_ref()],
            ctx.program_id,
        );
        require!(
            vault_authority == ctx.accounts.vault_authority.key(),
            VaultError::InvalidVaultAuthority
        );
        let vault_token_ata = get_associated_token_address(&vault_authority, &token_mint.key());
        require!(
            vault_token_ata == ctx.accounts.vault_token_account.key(),
            VaultError::InvalidVaultAccount
        );

        // 4. Derive user's destination token ATA for fungible token
        let user_token_ata = get_associated_token_address(&signer.key(), &token_mint.key());
        require!(
            user_token_ata == ctx.accounts.user_token_account.key(),
            VaultError::InvalidDestination
        );

        // 5. Transfer tokens
        let nft_mint_key = nft_mint.key();
        let signer_seeds: &[&[u8]] = &[
            b"PoliCromixPiggyBankV2",
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
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: User's NFT account (should be the NFT ATA)
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    
    /// CHECK: The NFT mint (input)
    pub nft_mint: Account<'info, Mint>,
    
    /// CHECK: The fungible token mint (input)
    pub token_mint: Account<'info, Mint>,
    
    /// CHECK: User's destination ATA for the fungible token (derived inside, for require checks)
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Vault's SPL token account for this fungible token
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: PDA vault authority ("PoliCromixPiggyBankV2", nft_mint)
    #[account(seeds = [b"PoliCromixPiggyBankV2", nft_mint.key().as_ref()], bump)]
    pub vault_authority: AccountInfo<'info>,
    
    /// CHECK: SPL Token program
    pub token_program: Program<'info, Token>,
}

// Vault errors extended for clarity
#[error_code]
pub enum VaultError {
    #[msg("The sender does not own the NFT.")]
    NotNFTOwner,
    #[msg("Invalid vault authority.")]
    InvalidVaultAuthority,
    #[msg("Invalid vault account.")]
    InvalidVaultAccount,
    #[msg("Invalid destination ATA.")]
    InvalidDestination,
}

security_txt! {
   Contact: "security@example.com",
   Acknowledgements: "https://example.com/security-acknowledgements"
}
