use anchor_lang::{prelude::*, solana_program::program_option::COption, system_program::{self, Transfer}};
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

    #[account()]
    /// CHECK: checked by seeds
    pub ebalts_mint_config: UncheckedAccount<'info>,

    #[account(
        owner = spl_token_2022::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: extra metas, its checked by seeds
    #[account(
        mut,
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
        mint_config.freeze_authority == ctx.accounts.authority.key() && ctx.accounts.mint.freeze_authority == COption::Some(ctx.accounts.ebalts_mint_config.key()),
        ProgramErrors::InvalidAuthority
    );

    let mut lists = vec![ctx.accounts.list_config.key()];
    //let mut lists = vec![&ctx.accounts.list_config.];
    for acc in ctx.remaining_accounts.iter() {
        let mut mint_config_data: &[u8] = &mut acc.data.borrow_mut();
        let is_valid = ListConfig::try_deserialize(&mut mint_config_data).is_ok();
        require!(is_valid, ProgramErrors::InvalidListConfig);
        lists.push(acc.key());
    }

    let extra_metas_size = get_extra_metas_size(&lists);
    let previous_size = ctx.accounts.extra_metas_thaw.data_len();
    ctx.accounts.extra_metas_thaw.resize(extra_metas_size).unwrap();

    if previous_size > extra_metas_size {
        let rent = Rent::get()?.minimum_balance(previous_size-extra_metas_size);
        let diff = ctx.accounts.extra_metas_thaw.lamports().checked_sub(rent).unwrap();
        ctx.accounts.extra_metas_thaw.sub_lamports(diff)?;
        ctx.accounts.authority.add_lamports(diff)?;
    } else if previous_size < extra_metas_size {
        let rent = Rent::get()?.minimum_balance(extra_metas_size-previous_size);
        let diff = ctx.accounts.extra_metas_thaw.lamports().checked_sub(rent).unwrap();


        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.extra_metas_thaw.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        system_program::transfer(cpi_ctx, diff)?;
    }

    let extra_metas_account = &ctx.accounts.extra_metas_thaw;
    let metas = get_extra_metas(&lists);
    let mut data = extra_metas_account.try_borrow_mut_data().unwrap();
    ExtraAccountMetaList::init::<ebalts_interface::instruction::CanThawPermissionlessInstruction>(
        &mut data, &metas,
    )
    .unwrap();

    Ok(())
}

fn get_extra_metas(lists: &[Pubkey]) -> Vec<ExtraAccountMeta> {
    let mut metas = Vec::with_capacity(lists.len());

    let mut index = 5;
    for list in lists {
        metas.push(ExtraAccountMeta::new_with_pubkey(list, false, false).unwrap());
        metas.push(ExtraAccountMeta::new_with_seeds(
            &[
                Seed::AccountKey { index },
                Seed::AccountData {
                    account_index: 1, // token account
                    data_index: 32,   // ta owner
                    length: 32,
                },
            ],
            false,
            false,
        )
        .unwrap());
        index += 2;
    }
    
    
    metas
}

fn get_extra_metas_size(lists: &[Pubkey]) -> usize {
    ExtraAccountMetaList::size_of(2 * lists.len()).unwrap()
}
