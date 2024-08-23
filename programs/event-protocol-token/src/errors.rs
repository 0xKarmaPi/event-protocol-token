use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The start time is not reached yet")]
    StartTimeNotReached,
    
    #[msg("The amount of event token is not enough")]
    AmountOfEventTokenIsNotEnough,
}