use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrors {
    InvalidAuthority,
    NotSupported,
    InvalidMintConfig,
    InvalidABWallet,
    Unauthorized,
    InvalidListConfig,
}
