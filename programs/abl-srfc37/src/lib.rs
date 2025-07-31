#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use ebalts_interface::instruction::{
    CanFreezePermissionlessInstruction, CanThawPermissionlessInstruction,
};
use spl_discriminator::SplDiscriminate;

mod constants;
mod errors;
mod instructions;
mod state;

pub use errors::*;
use instructions::*;
pub use state::*;

declare_id!("8hNxmWetsVptuZ5LGYC6fM4xTpoUfPijz3NyYctyM79N");


#[program]
pub mod abl_srfc37 {
    use super::*;

    pub fn initialize_list_config(
        ctx: Context<InitializeListConfig>,
        args: InitializeListConfigArgs,
    ) -> Result<()> {
        instructions::initialize_list_config::initialize_list_config(ctx, args)
    }

    pub fn add_wallet_to_list(ctx: Context<AddWalletToList>, wallet: Pubkey) -> Result<()> {
        instructions::add_wallet_to_list::add_wallet_to_list(ctx, wallet)
    }

    pub fn remove_wallet_from_list(ctx: Context<RemoveWalletFromList>) -> Result<()> {
        instructions::remove_wallet_from_list::remove_wallet_from_list(ctx)
    }

    pub fn set_list_mode(ctx: Context<SetListMode>, mode: Mode) -> Result<()> {
        instructions::set_list_mode::set_list_mode(ctx, mode)
    }

    #[instruction(discriminator = CanThawPermissionlessInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn can_thaw_permissionless(ctx: Context<ThawPermissionless>) -> Result<()> {
        instructions::can_thaw_permissionless::can_thaw_permissionless(ctx)
    }

    #[instruction(discriminator = CanFreezePermissionlessInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn can_freeze_permissionless(_ctx: Context<CanFreezePermissionless>) -> Result<()> {
        Err(ProgramErrors::NotSupported.into())
    }

    pub fn set_extra_metas_thaw(ctx: Context<SetExtraMetasThaw>) -> Result<()> {
        instructions::set_extra_metas_thaw::set_extra_metas_thaw(ctx)
    }
}
