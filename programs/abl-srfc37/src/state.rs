use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ABWallet {
    pub wallet: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct ListConfig {
    pub authority: Pubkey,
    pub seed: Pubkey,
    pub mode: Mode,
    pub bump: u8,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub enum Mode {
    Allow,
    AllowWithPermissionlessEOAs,
    Block,
}
