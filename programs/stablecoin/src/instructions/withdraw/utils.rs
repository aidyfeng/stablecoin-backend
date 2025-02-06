use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::{burn, Burn, Token2022},
    token_interface::{Mint, TokenAccount},
};

use crate::constants::SEED_SOL_ACCOUNT;

pub fn withdraw_sol<'info>(
    bump: u8,
    depositor_key: &Pubkey,
    system_program: &Program<'info, System>,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];

    let cpi_context = CpiContext::new_with_signer(
        system_program.to_account_info(),
        Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        },
        signer_seeds,
    );
    transfer(cpi_context, amount)
}

pub fn burn_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    mint_account: &InterfaceAccount<'info, Mint>,
    from: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    let ctx_context = CpiContext::new(
        token_program.to_account_info(),
        Burn {
            mint: mint_account.to_account_info(),
            from: from.to_account_info(),
            authority: authority.to_account_info(),
        },
    );
    burn(ctx_context, amount)
}
