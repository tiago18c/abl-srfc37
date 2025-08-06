use crate::{ABWallet, ListConfig, ProgramErrors};
use anchor_lang::{prelude::*, solana_program::instruction::get_stack_height};
use solana_curve25519::edwards::PodEdwardsPoint;

#[derive(Accounts)]
pub struct ThawPermissionless<'info> {
    /// CHECK:
    pub authority: UncheckedAccount<'info>,
    /// CHECK:
    pub token_account: UncheckedAccount<'info>,
    /// CHECK:
    pub mint: UncheckedAccount<'info>,
    /// CHECK:
    pub owner: UncheckedAccount<'info>,
    /// CHECK:
    pub extra_metas: UncheckedAccount<'info>,
}

pub fn can_thaw_permissionless(ctx: Context<ThawPermissionless>) -> Result<()> {

    // remaining accounts should be pairs of list and ab_wallet
    let mut remaining_accounts = ctx.remaining_accounts.iter();
    while let Some(list) = remaining_accounts.next() {
        let ab_wallet = remaining_accounts.next().unwrap();
        validate_thaw_list(list, ab_wallet).map_err(|_| {
            msg!("Failed to pass validation for list {:?}", list.key());
            ProgramErrors::Unauthorized })?;
    }

    Ok(())
}


fn validate_thaw_list<'info>(list: &AccountInfo<'info>, ab_wallet: &AccountInfo<'info>) -> Result<()> {
    let mut list_data: &[u8] = &mut list.data.borrow_mut();
    let list_config = ListConfig::try_deserialize(&mut list_data).map_err(|_| ProgramErrors::InvalidListConfig)?;

    // 3 operation modes
    // allow: only wallets that have been allowlisted can thaw, requires previously created ABWallet account
    // block: only wallets that have been blocklisted can't thaw, thawing requires ABWallet to not exist
    // allow with permissionless eoas: all wallets that can sign can thaw, otherwise requires previously created ABWallet account (for PDAs)
    match list_config.mode {
        crate::Mode::Allow => {
            let mut ab_wallet_data: &[u8] = &mut ab_wallet.data.borrow_mut();
            let _ = ABWallet::try_deserialize(&mut ab_wallet_data)
                .map_err(|_| ProgramErrors::Unauthorized)?;

            Ok(())
        }
        crate::Mode::AllowWithPermissionlessEOAs => {
            let pt = PodEdwardsPoint(ab_wallet.key.to_bytes());
            let mut ab_wallet_data: &[u8] = &mut ab_wallet.data.borrow_mut();
            let res = ABWallet::try_deserialize(&mut ab_wallet_data);
            require!(
                solana_curve25519::edwards::validate_edwards(&pt) || res.is_ok(),
                ProgramErrors::Unauthorized
            );
            Ok(())
        }
        crate::Mode::Block => {
            let mut ab_wallet_data: &[u8] = &mut ab_wallet.data.borrow_mut();
            let res = ABWallet::try_deserialize(&mut ab_wallet_data);

            if res.is_ok() {
                Err(ProgramErrors::Unauthorized.into())
            } else {
                Ok(())
            }
        }
    }
}