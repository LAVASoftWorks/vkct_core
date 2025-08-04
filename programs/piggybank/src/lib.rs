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

const SEED_VAULT:               &[u8] = b"VkctPiggyBankV050vaultSeed";
const SEED_TOKEN_REGISTRY:      &[u8] = b"VkctPiggyBankV050tRegistry";
const SEED_COLLECTION_REGISTRY: &[u8] = b"VkctPiggyBankV050cRegistry";

const MAX_ALLOWED_TOKENS: usize = 256;
const MAX_COLLECTIONS:    usize = 256;

#[program]
pub mod piggybank {
    use super::*;
    
    // Valid SPL tokens registry initializer
    
    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        
        let registry = &mut ctx.accounts.registry;
        registry.admin = ctx.accounts.admin.key();
        registry.allowed_mints = Vec::new();
        Ok(())
    }
    
    // Add a token to the registry
    
    pub fn add_token(ctx: Context<AddToken>, token_mint: Pubkey) -> Result<()> {
        
        let registry = &mut ctx.accounts.registry;
        require!(ctx.accounts.admin.key() == registry.admin, CustomError::Unauthorized);

        if !registry.allowed_mints.contains(&token_mint) {
            registry.allowed_mints.push(token_mint);
        }
        Ok(())
    }
    
    // Valid collections registry initializer
    
    pub fn initialize_collection_registry(ctx: Context<InitializeCollectionRegistry>) -> Result<()> {
        
        let registry = &mut ctx.accounts.collection_registry;
        registry.admin = ctx.accounts.admin.key();
        registry.collections = Vec::new(); // Start empty
        Ok(())
    }
    

    // Add to the collections registry
    
    pub fn add_collection(ctx: Context<AddCollection>, new_collection: Pubkey) -> Result<()> {
        
        let registry = &mut ctx.accounts.collection_registry;

        require!(ctx.accounts.admin.key() == registry.admin, CustomError::Unauthorized);

        require!(
            !registry.collections.contains(&new_collection),
            CustomError::CollectionAlreadyExists
        );

        require!(
            registry.collections.len() < MAX_COLLECTIONS,
            CustomError::RegistryFull
        );

        registry.collections.push(new_collection);

        Ok(())
    }
    
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        
        let token_mint = ctx.accounts.token_mint.key();
        let token_registry = &ctx.accounts.registry;

        let collection_mint = ctx.accounts.collection_mint.key();
        let collection_registry = &ctx.accounts.collection_registry;
        
        // --- 1. Check if the NFT we want to withdraw from is part of a valid collection.

        require!(
            collection_registry.collections.contains(&collection_mint),
            CustomError::CollectionNotWhitelisted
        );

        // --- 2. Check if the SPL token attempting to be withdrawn is in the valid tokens registry
        
        require!(
            token_registry.allowed_mints.contains(&token_mint),
            CustomError::InvalidToken
        );
        
        // --- 3. Validate NFT ownership (SPL token with amount 1) ---
        
        // Anchor constraints already check this!
        
        // --- 4. Create destination ATA for user if it doesn't exist ---
        
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
        
        // --- 5. Transfer SPL tokens from vault ATA to user ATA ---
        let bump = ctx.bumps.vault;
        let nft_mint_key = ctx.accounts.nft_mint.key();
        let tkn_mint_key = ctx.accounts.token_mint.key();
        let seeds = &[
            SEED_VAULT,
            nft_mint_key.as_ref(),
            ctx.accounts.collection_mint.key.as_ref(),
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

// Segment: token registry structs

#[account]
pub struct TokenRegistry {
    pub admin: Pubkey,
    pub allowed_mints: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + (4 + MAX_ALLOWED_TOKENS * 32),
        seeds = [SEED_TOKEN_REGISTRY],
        bump
    )]
    pub registry: Account<'info, TokenRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddToken<'info> {
    #[account(mut, seeds = [SEED_TOKEN_REGISTRY], bump)]
    pub registry: Account<'info, TokenRegistry>,
    pub admin: Signer<'info>,
}

// Segment: collection registry structs

#[account]
pub struct CollectionRegistry {
    pub admin: Pubkey,
    pub collections: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct InitializeCollectionRegistry<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 4 + (MAX_COLLECTIONS * 32), // 8 = anchor header, 32 = admin, 4 = Vec len, rest = elements
        seeds = [SEED_COLLECTION_REGISTRY],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddCollection<'info> {
    #[account(
        mut,
        seeds = [SEED_COLLECTION_REGISTRY],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,

    pub admin: Signer<'info>,
}

// Segment: Withdraw function struct

#[derive(Accounts)]
pub struct Withdraw<'info> {

    // --- [0] The NFT owner ---
    #[account(mut)]
    pub signer: Signer<'info>,

    // --- [1] The NFT itself ---
    pub nft_mint: Account<'info, Mint>,

    // --- [2] The SPL token wanted to be taken out ---
    pub token_mint: Account<'info, Mint>,

    // --- [3] The user's ATA holding the NFT ---
    #[account(
        constraint = nft_token_account.owner  == signer.key(),
        constraint = nft_token_account.mint   == nft_mint.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    // --- [4] Valid SPL Token registry PDA
    #[account(seeds = [SEED_TOKEN_REGISTRY], bump)]
    pub registry: Account<'info, TokenRegistry>,

    // --- [5] Vault PDA ---
    /// CHECK: Vault PDA, derived by the provided string and the NFT pubkey that owns it.
    /// Only used as authority for token transfers. No data is read or written.
    #[account(
        seeds = [SEED_VAULT, nft_mint.key().as_ref(), collection_mint.key().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    
    // --- [6] Collection mint address ---
    /// CHECK: Passed in by client. Checked against registry manually.
    pub collection_mint: UncheckedAccount<'info>,
    
    // --- [7] Collection registry account
    #[account(
        seeds = [SEED_COLLECTION_REGISTRY],
        bump
    )]
    pub collection_registry: Account<'info, CollectionRegistry>,
    
    // --- [8] Vault's ATA for SPL token ---
    #[account(
        mut,
        associated_token::mint      = token_mint,
        associated_token::authority = vault
    )]
    pub vault_token_ata: Account<'info, TokenAccount>,
    
    // --- [9] User's ATA for SPL token ---
    /// CHECK: Constraints on nft_user_account and vault are strictly set, avoiding re-initialization attacks.
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint      = token_mint,
        associated_token::authority = signer
    )]
    pub user_token_ata: Account<'info, TokenAccount>,

    // --- [10, 11, 12] Solana programs ---
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// Segment: other stuff

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized: only admin can perform this action.")]
    Unauthorized,

    #[msg("Token mint is not allowed.")]
    InvalidToken,

    #[msg("This collection is not whitelisted.")]
    CollectionNotWhitelisted,

    #[msg("Collection already exists in the registry.")]
    CollectionAlreadyExists,

    #[msg("The registry has reached maximum capacity.")]
    RegistryFull,
}

security_txt! {
    // Required fields
    name:        "Example",
    project_url: "http://example.com",
    contacts:    "email:example@example.com,link:https://example.com/security,discord:example#1234",
    policy:      "https://github.com/solana-labs/solana/blob/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code:         "https://github.com/example/example"
    // auditors:            "None",
    // acknowledgements:    ""
}
