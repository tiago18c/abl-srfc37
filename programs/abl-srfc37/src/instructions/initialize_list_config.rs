use anchor_lang::prelude::*;

use crate::{constants::LIST_CONFIG_SEED, ListConfig, Mode};

#[derive(Accounts)]
#[instruction(args: InitializeListConfigArgs)]
pub struct InitializeListConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + ListConfig::INIT_SPACE,
        seeds = [LIST_CONFIG_SEED, authority.key().as_ref(), args.seed.as_ref()],
        bump
    )]
    pub list_config: Account<'info, ListConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeListConfigArgs {
    pub seed: Pubkey,
    pub mode: Mode,
}

pub fn initialize_list_config(
    ctx: Context<InitializeListConfig>,
    args: InitializeListConfigArgs,
) -> Result<()> {
    ctx.accounts.list_config.set_inner(ListConfig {
        authority: ctx.accounts.authority.key(),
        seed: args.seed,
        mode: args.mode,
        bump: ctx.bumps.list_config,
    });
    Ok(())
}
