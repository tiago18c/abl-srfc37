use anchor_lang::prelude::*;

use crate::{ABWallet, ListConfig};

#[derive(Accounts)]
pub struct RemoveWalletFromList<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub list_config: Account<'info, ListConfig>,

    #[account(
        mut,
        close = authority,
        seeds = [list_config.key().as_ref(), ab_wallet.wallet.as_ref()],
        bump
    )]
    pub ab_wallet: Account<'info, ABWallet>,

    pub system_program: Program<'info, System>,
}

pub fn remove_wallet_from_list(_ctx: Context<RemoveWalletFromList>) -> Result<()> {
    Ok(())
}
