use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token;
use anchor_spl::token::{self, MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use mpl_token_metadata::state::Creator;

declare_id!("3TthbtqUhRwetmU9RgDcdgTesstLyx8GQRugeyqMa8xt");

#[program]
mod nft_minter {

    use super::*;
    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
    ) -> Result<()> {
        msg!("Minting NFT");

        let creators = vec![Creator {
            address: ctx.accounts.payer.key(),
            verified: false,
            share: 100,
        }];

        let ix = create_metadata_accounts_v3(
            ctx.accounts.mpl_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            name,
            symbol,
            uri,
            Some(creators),
            seller_fee_basis_points,
            true,
            true,
            None,
            None,
            None,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;
        msg!("Metadata Account Created");

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.reciever_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("Minting NFT");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        msg!("TOKENMINTED");

        let ix = create_master_edition_v3(
            ctx.accounts.mpl_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.metadata_mint.key(),
            ctx.accounts.update_auth.key(),
            ctx.accounts.update_auth.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.update_auth.key(),
            Some(0),
        );

        invoke(
            &ix,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata_mint.to_account_info(),
                ctx.accounts.update_auth.to_account_info(),
                ctx.accounts.update_auth.to_account_info(),
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub metadata_mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub update_auth: UncheckedAccount<'info>,
    #[account(address = mpl_token_metadata::id())]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub mpl_program: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub reciever_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Okay for demo because we're the only one who's passing them in.
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub ata_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
