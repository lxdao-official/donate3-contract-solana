use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    pda::find_master_edition_account, pda::find_metadata_account,
};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use solana_program::pubkey::Pubkey;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("3y2CujbRo3nz6zQVaRWFHW61332R9RqfCmgAhGVpiESa");

#[program]
mod hello_anchor {
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
        invoke_signed(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.collection_mint.key(),
                ctx.accounts.signer.key(),
                ctx.accounts.collection_mint.key(),
                "name".to_string(),
                "syb".to_string(),
                "uri".to_string(),
                None,
                0,
                false,
                false,
                None,
                None,
                None,
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

    #[account(
    mut,
    address = find_master_edition_account(& collection_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,
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

