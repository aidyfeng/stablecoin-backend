use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("CZPGAfzFybi2i7tZyyRe2i1FM4E5G3BhkD18rEbL3PgL");

mod constants;
mod error;
mod instructions;
mod state;

#[program]
pub mod stablecoin {

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        instructions::process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_token(
        ctx: Context<DepositCollateryAndMintToken>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        instructions::process_deposit_collateral_and_mint_token(
            ctx,
            amount_collateral,
            amount_to_mint,
        )
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        instructions::process_redeem_collateral_and_burn_tokens(
            ctx,
            amount_collateral,
            amount_to_burn,
        )
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        instructions::process_liquidate(ctx, amount_to_burn)
    }
}
