use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    pda::find_master_edition_account, pda::find_metadata_account, state::CollectionDetails,
};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use mpl_token_metadata::state::Creator;
use solana_program::pubkey::Pubkey;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("FxnUsQGyFyBsChLbWA8Rd9NTHN2KhGLoity1dMMt26of");

#[program]
mod donate3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let seeds = b"collection";
        let bump = *ctx.bumps.get("collection_mint").unwrap();
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.collection_mint.to_account_info(),
                authority: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
            },
            signer,
        );

        token::mint_to(mint_ctx, 1)?;

        let creators = vec![Creator {
            address: ctx.accounts.signer.key(),
            verified: false,
            share: 100,
        }];

        let collection = Some(CollectionDetails::V1 { size: 0 });
        invoke_signed(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.collection_mint.key(),
                "Donate3".to_string(),
                "DT3".to_string(),
                "https://arweave.net/VJZNf86VXF-S1o5F_xfSi6Ur9jyqmEv_hVj_6g8v8Lg".to_string(),
                Some(creators),
                0,
                false,
                false,
                None,
                None,
                collection,
            ),
            vec![
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ].as_slice(),
            signer,
        )?;

        invoke_signed(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.signer.key(),
                Some(0),
            ),
            vec![
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.collection_mint.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ].as_slice(),
            signer,
        )?;

        Ok(())
    }

    pub fn mint(ctx: Context<MintNFT>) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        let creators = vec![Creator {
            address: ctx.accounts.signer.key(),
            verified: false,
            share: 100,
        }];

        let collection = Some(CollectionDetails::V1 { size: 0 });
        invoke(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.token_mint.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.signer.key(),
                "Donate3".to_string(),
                "DT3".to_string(),
                "https://arweave.net/VJZNf86VXF-S1o5F_xfSi6Ur9jyqmEv_hVj_6g8v8Lg".to_string(),
                Some(creators),
                0,
                false,
                false,
                None,
                None,
                None,
            ),
            vec![
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ].as_slice(),
        )?;

        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.token_mint.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.signer.key(),
                Some(0),
            ),
            vec![
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ].as_slice(),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    init,
    payer = signer,
    mint::decimals = 0,
    mint::authority = signer,
    mint::freeze_authority = signer)]
    pub token_mint: Account<'info, Mint>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    init,
    payer = signer,
    associated_token::mint = token_mint,
    associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    mut,
    address = find_master_edition_account(& token_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    mut,
    address = find_metadata_account(& token_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init_if_needed,
    seeds = [b"collection"],
    bump,
    payer = signer,
    mint::decimals = 0,
    mint::authority = collection_mint,
    mint::freeze_authority = collection_mint
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(
    init_if_needed,
    payer = signer,
    associated_token::mint = collection_mint,
    associated_token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    mut,
    address = find_master_edition_account(& collection_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(
    mut,
    address = find_metadata_account(& collection_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

