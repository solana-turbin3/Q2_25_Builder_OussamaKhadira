use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::{ListingState, Marketplace, DeviceRegistry, PurchaseRecord};

#[derive(Accounts)]
#[instruction(listing_id: [u8; 32], units_requested: u64)]
pub struct PurchaseListing<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = buyer,
        //constraint = buyer_ata.amount >= price_for_units @ ErrorCode::InsufficientFunds,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = listing_state.seller,
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"listing", device_registry.key().as_ref(), listing_id.as_ref()],
        bump = listing_state.bump,
        constraint = listing_state.status == 0 @ ErrorCode::ListingNotActive,
        constraint = listing_state.seller != buyer.key() @ ErrorCode::CannotBuyOwnListing,
    )]
    pub listing_state: Account<'info, ListingState>,

    #[account(
        seeds = [b"marketplace", marketplace.admin.as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"device", marketplace.key().as_ref(), device_registry.device_id.as_bytes()],
        bump = device_registry.bump,
        constraint = device_registry.is_active @ ErrorCode::DeviceInactive,
    )]
    pub device_registry: Account<'info, DeviceRegistry>,

    pub usdc_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,

    #[account(
        init,
        payer = buyer,
        space = 8 + PurchaseRecord::INIT_SPACE,
        seeds = [
            b"purchase",
            listing_state.key().as_ref(),
            &listing_state.purchase_count.to_le_bytes()
        ],
        bump,
    )]
    pub purchase_record: Account<'info, PurchaseRecord>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct ListingPurchased {
    pub listing_id: [u8; 32],
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub units_purchased: u64,
    pub price_paid: u64,
    pub fee: u64,
    pub remaining_units: u64,
    pub timestamp: i64,
}

pub fn handler(
    ctx: Context<PurchaseListing>,
    listing_id: [u8; 32],
    units_requested: u64,
) -> Result<()> {
    // Snapshot immutable fields to avoid borrow conflicts
    let listing_key = ctx.accounts.listing_state.key();
    let curr_count = ctx.accounts.listing_state.purchase_count;

    let marketplace = &ctx.accounts.marketplace;
    let clock = Clock::get()?;

    // Mutable borrow of listing_state
    let listing = &mut ctx.accounts.listing_state;

    // ensure still active & not buying own
    require!(listing.status == 0, ErrorCode::ListingNotActive);
    require!(listing.seller != ctx.accounts.buyer.key(), ErrorCode::CannotBuyOwnListing);

    // ensure enough units
    require!(units_requested <= listing.remaining_units, ErrorCode::InsufficientUnits);

    // compute amounts
    let price_for_units = listing.price_per_unit
        .checked_mul(units_requested)
        .ok_or(ErrorCode::MathOverflow)?;
    let fee = (price_for_units as u128)
        .checked_mul(marketplace.seller_fee as u128)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::MathOverflow)?
        .try_into()
        .map_err(|_| ErrorCode::MathOverflow)?;
    let amount_to_seller = price_for_units.checked_sub(fee).ok_or(ErrorCode::MathOverflow)?;

    // 1) transfer buyer → seller
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from:      ctx.accounts.buyer_ata.to_account_info(),
                to:        ctx.accounts.seller_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        amount_to_seller,
    )?;

    // 2) transfer fee → treasury
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from:      ctx.accounts.buyer_ata.to_account_info(),
                to:        ctx.accounts.treasury_ata.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        fee,
    )?;

    // 3) update listing state
    listing.remaining_units = listing
        .remaining_units
        .checked_sub(units_requested)
        .ok_or(ErrorCode::MathOverflow)?;
    if listing.remaining_units == 0 {
        listing.status = 1; // Sold out
    }
    listing.updated_at = clock.unix_timestamp;

    // 4) populate the PurchaseRecord
    let record = &mut ctx.accounts.purchase_record;
    record.listing         = listing_key;
    record.buyer           = ctx.accounts.buyer.key();
    record.units_purchased = units_requested;
    record.price_paid      = price_for_units;
    record.fee             = fee;
    record.timestamp       = clock.unix_timestamp;

    // 5) bump the counter so the next purchase gets a fresh PDA
    listing.purchase_count = curr_count.checked_add(1).unwrap();

    // 6) emit the event for off-chain indexing
    emit!(ListingPurchased {
        listing_id,
        buyer:            ctx.accounts.buyer.key(),
        seller:           listing.seller,
        units_purchased:  units_requested,
        price_paid:       price_for_units,
        fee,
        remaining_units:  listing.remaining_units,
        timestamp:        clock.unix_timestamp,
    });

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Listing is not active")]
    ListingNotActive,
    #[msg("Cannot buy your own listing")]
    CannotBuyOwnListing,
    #[msg("Device is inactive")]
    DeviceInactive,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Insufficient units available")]
    InsufficientUnits,
}
