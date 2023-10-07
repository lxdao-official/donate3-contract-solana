use anchor_lang::{
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    },
    system_program,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v3},
    pda::{find_master_edition_account, find_metadata_account},
    state::{CollectionDetails, Creator},
};
use solana_program::pubkey::Pubkey;

declare_id!("3yz5aQ6A5w5mWicriDkAHAqGEC4LDU2A9gLmtxTUbmdx");

#[program]
pub mod donate3 {
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

    pub fn transfer_lamports(
        ctx: Context<TransferLamports>,
        amount: u64,
        message: String,
    ) -> Result<()> {
        let from_account = &ctx.accounts.signer;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction =
            system_instruction::transfer(from_account.key, to_account.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
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

    pub fn transfer_spl_tokens(
        ctx: Context<TransferSpl>,
        amount: u64,
        message: String,
    ) -> Result<()> {
        let destination = &ctx.accounts.to_ata;
        let source = &ctx.accounts.from_ata;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.from;
        // Transfer tokens from taker to initializer
        let cpi_accounts = Transfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSpl<'info> {
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
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
