use crate::{ABWallet, ListConfig, ProgramErrors};
use anchor_lang::{prelude::*, solana_program::instruction::get_stack_height};

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
            // we don't change any state, so if it gets called by the wrong SC there is no harm
            // if called by the right SC -> stack height is 2
            // for authority == owner and authority.is_signer == true, this requires that the permissionless thaw is the top level instruction in a transaction
            // (authority gets de-escalated, so we cant check signer flag)
            // for PDAs to mimic this behavior, stack height would be > 2
            // alternatively, PDAs can mimic this with stack height == 2, but then the caller wouldn't be ebalts and can't thaw, so we're good
            if ctx.accounts.authority.key == ctx.accounts.owner.key && get_stack_height() == 2 {
                Ok(())
            } else {
                let mut ab_wallet_data: &[u8] = &mut ctx.accounts.ab_wallet.data.borrow_mut();
                let _ = ABWallet::try_deserialize(&mut ab_wallet_data)
                    .map_err(|_| ProgramErrors::Unauthorized)?;

                Ok(())
            }
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
