#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused_variables)]

/**
 * Anchor-based Solana program for NFT vaults
 * 
 * Author: lava.caballero@gmail.com
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use anchor_spl::associated_token::{self, AssociatedToken};
use solana_security_txt::security_txt;

// Note: put here the piggy bank's public key
declare_id!("VaU1t11111111111111111111111111111111111111");

#[program]
pub mod piggybank {
    use super::*;

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // --- 1. Validate NFT ownership (SPL token with amount 1) ---
        // Anchor constraints already check this!
        
        // --- 2. Create destination ATA for user if it doesn't exist ---
        if ctx.accounts.user_token_ata.to_account_info().lamports() == 0 {
            let cpi_accounts = associated_token::Create {
                payer: ctx.accounts.signer.to_account_info(),
                associated_token: ctx.accounts.user_token_ata.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                cpi_accounts,
            );
            associated_token::create(cpi_ctx)?;
        }
        
        // --- 3. Transfer SPL tokens from vault ATA to user ATA ---
        let bump = ctx.bumps.vault;
        let nft_mint_key = ctx.accounts.nft_mint.key();
        let tkn_mint_key = ctx.accounts.token_mint.key();
        let seeds = &[
            b"PoliCromixPiggyBankV3",
            nft_mint_key.as_ref(),
            tkn_mint_key.as_ref(),
            &[ctx.bumps.vault],
        ];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_ata.to_account_info(),
            to: ctx.accounts.user_token_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        let signer_seeds: &[&[&[u8]]] = &[seeds];
        
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        
        token::transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        constraint = nft_token_account.owner == signer.key(),
        constraint = nft_token_account.mint == nft_mint.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    // --- Vault PDA ---
    #[account(
        seeds = [b"PoliCromixPiggyBankV3", nft_mint.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    
    // --- Vault's ATA for SPL token ---
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault
    )]
    pub vault_token_ata: Account<'info, TokenAccount>,
    
    // --- User's ATA for SPL token ---
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = signer
    )]
    pub user_token_ata: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

security_txt! {
   Contact: "security@example.com",
   Acknowledgements: "https://example.com/security-acknowledgements"
}
