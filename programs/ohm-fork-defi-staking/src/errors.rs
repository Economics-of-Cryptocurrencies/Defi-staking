use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Not owner")]
    InvalidOwner,

    #[msg("Wrong amount")]
    WrongAmount,

    #[msg("Not enough")]
    NotEnough,

    #[msg("Not allow meme coin")]
    NotAllowMemeCoin
}