use anchor_lang::prelude::*;
use anchor_spl::{token_2022::spl_token_2022, token_interface::Mint};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

use crate::{ListConfig, ProgramErrors};

#[derive(Accounts)]
pub struct SetExtraMetasThaw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub list_config: Account<'info, ListConfig>,

    #[account(
        seeds = [ebalts::state::MintConfig::SEED_PREFIX, mint.key().as_ref()],
        bump,
        seeds::program = ebalts::ID,
    )]
    /// CHECK: checked by seeds
    pub ebalts_mint_config: UncheckedAccount<'info>,

    #[account(
        owner = spl_token_2022::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: extra metas, its checked by seeds
    #[account(
        init_if_needed,
        payer = authority,
        space = get_extra_metas_size(),
        seeds = [ebalts_interface::THAW_EXTRA_ACCOUNT_METAS_SEED, mint.key().as_ref()],
        bump
    )]
    pub extra_metas_thaw: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn set_extra_metas_thaw(ctx: Context<SetExtraMetasThaw>) -> Result<()> {
    let mint_config_data = ctx.accounts.ebalts_mint_config.data.borrow();
    let mint_config = ebalts::state::load_mint_config(&mint_config_data)
        .map_err(|_| ProgramErrors::InvalidMintConfig)?;

    // only the selected freeze authority should be able to set the extra metas

    require!(
        mint_config.freeze_authority == ctx.accounts.authority.key(),
        ProgramErrors::InvalidAuthority
    );

    let extra_metas_account = &ctx.accounts.extra_metas_thaw;
    let metas = get_extra_metas(&ctx.accounts.list_config.key());
    let mut data = extra_metas_account.try_borrow_mut_data().unwrap();
    ExtraAccountMetaList::init::<ebalts_interface::instruction::CanThawPermissionlessInstruction>(
        &mut data, &metas,
    )
    .unwrap();

    Ok(())
}

fn get_extra_metas(config: &Pubkey) -> Vec<ExtraAccountMeta> {
    [
        // [5] ListConfig
        ExtraAccountMeta::new_with_pubkey(config, false, false).unwrap(),
        // [6] ABWallet
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::AccountKey { index: 5 },
                Seed::AccountData {
                    account_index: 1, // token account
                    data_index: 32,   // ta owner
                    length: 32,
                },
            ],
            false,
            false,
        )
        .unwrap(),
    ]
    .to_vec()
}

fn get_extra_metas_size() -> usize {
    ExtraAccountMetaList::size_of(1).unwrap()
}
