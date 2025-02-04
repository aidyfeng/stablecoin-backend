use anchor_lang::prelude::*;

declare_id!("CZPGAfzFybi2i7tZyyRe2i1FM4E5G3BhkD18rEbL3PgL");

mod constants;
use constants::*;
mod instructions;
use instructions::*;
mod state;
use state::*;

#[program]
pub mod stablecoin {

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>,min_health_factor:u64) -> Result<()>{
        process_update_config(ctx, min_health_factor)
    }
}
