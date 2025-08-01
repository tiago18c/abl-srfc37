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
    /// CHECK:
    pub list_config: Account<'info, ListConfig>,
    /// CHECK:
    pub ab_wallet: UncheckedAccount<'info>,
}

pub fn can_thaw_permissionless(ctx: Context<ThawPermissionless>) -> Result<()> {
    // 3 operation modes
    // allow: only wallets that have been allowlisted can thaw, requires previously created ABWallet account
    // block: only wallets that have been blocklisted can't thaw, thawing requires ABWallet to not exist
    // allow with permissionless eoas: all wallets that can sign can thaw, otherwise requires previously created ABWallet account (for PDAs)
    match ctx.accounts.list_config.mode {
        crate::Mode::Allow => {
            let mut ab_wallet_data: &[u8] = &mut ctx.accounts.ab_wallet.data.borrow_mut();
            let _ = ABWallet::try_deserialize(&mut ab_wallet_data)
                .map_err(|_| ProgramErrors::Unauthorized)?;

            Ok(())
        }
        crate::Mode::AllowWithPermissionlessEOAs => {
            let pt = PodEdwardsPoint(ctx.accounts.owner.key.to_bytes());
            let mut ab_wallet_data: &[u8] = &mut ctx.accounts.ab_wallet.data.borrow_mut();
            let res = ABWallet::try_deserialize(&mut ab_wallet_data);
            require!(
                solana_curve25519::edwards::validate_edwards(&pt) || res.is_ok(),
                ProgramErrors::Unauthorized
            );
            Ok(())
        }
        crate::Mode::Block => {
            let mut ab_wallet_data: &[u8] = &mut ctx.accounts.ab_wallet.data.borrow_mut();
            let res = ABWallet::try_deserialize(&mut ab_wallet_data);

            if res.is_ok() {
                Err(ProgramErrors::Unauthorized.into())
            } else {
                Ok(())
            }
        }
    }
}
