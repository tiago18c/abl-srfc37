use anchor_lang::prelude::*;

use crate::{ABWallet, ListConfig};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct AddWalletToList<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub list_config: Account<'info, ListConfig>,

    #[account(
        init,
        payer = authority,
        space = 8 + ABWallet::INIT_SPACE,
        seeds = [list_config.seed.as_ref(), wallet.as_ref()],
        bump
    )]
    pub ab_wallet: Account<'info, ABWallet>,

    pub system_program: Program<'info, System>,
}

pub fn add_wallet_to_list(ctx: Context<AddWalletToList>, wallet: Pubkey) -> Result<()> {
    ctx.accounts.ab_wallet.set_inner(ABWallet { wallet });
    Ok(())
}
