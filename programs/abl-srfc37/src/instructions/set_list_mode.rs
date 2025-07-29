use anchor_lang::prelude::*;

use crate::{ListConfig, Mode};

#[derive(Accounts)]
pub struct SetListMode<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub list_config: Account<'info, ListConfig>,
}

pub fn set_list_mode(ctx: Context<SetListMode>, mode: Mode) -> Result<()> {
    ctx.accounts.list_config.mode = mode;
    Ok(())
}
