use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::{self, mint_to, Token2022},
    token_interface::{Mint, TokenAccount},
};

use crate::constants::SEED_MINT_ACCOUNT;

pub fn mint_tokens<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    bump: u8,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];
    let cpi_context = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token_2022::MintTo {
            mint: mint_account.to_account_info(),
            to: token_account.to_account_info(),
            authority: mint_account.to_account_info(),
        },
        signer_seeds,
    );

    mint_to(cpi_context, amount)?;
    Ok(())
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    let cpi_context = CpiContext::new(
        system_program.to_account_info(),
        Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        },
    );
    transfer(cpi_context, amount)
}
