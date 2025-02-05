use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT}, instructions::utils::check_health_factor, state::{Collateral, Config}
};

use super::utils::{deposit_sol, mint_tokens};

#[derive(Accounts)]
pub struct DepositCollateryAndMintToken<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT,depositor.key().as_ref()],
        bump
    )]
    pub collateral: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT,depositor.key().as_ref()],
        bump
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn process_deposit_collateral_and_mint_token(
    ctx: Context<DepositCollateryAndMintToken>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_initialize {
        collateral_account.is_initialize = true;
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
    }

    check_health_factor(&ctx.accounts.collateral, &ctx.accounts.config_account, &ctx.accounts.price_update)?;

    deposit_sol(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    mint_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        amount_to_mint,
        ctx.accounts.config_account.bump,
    )?;

    Ok(())
}
