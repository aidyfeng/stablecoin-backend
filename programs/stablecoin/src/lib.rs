use anchor_lang::prelude::*;
use instructions::admin::*;
use crate::instructions::deposit::*;


declare_id!("CZPGAfzFybi2i7tZyyRe2i1FM4E5G3BhkD18rEbL3PgL");

mod constants;
mod instructions;
mod state;
mod error;

#[program]
pub mod stablecoin {



    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::admin::process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>,min_health_factor:u64) -> Result<()>{
        instructions::admin::process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_token(ctx:Context<DepositCollateryAndMintToken>,
        amount_collateral: u64,
        amount_to_mint: u64) -> Result<()>{
            instructions::deposit::process_deposit_collateral_and_mint_token(ctx, amount_collateral, amount_to_mint)
        }
}
