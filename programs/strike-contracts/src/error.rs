use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid bet type")]
    InvalidBetType,
    #[msg("Invalid match ID")]
    InvalidMatchId,
    #[msg("Match already exists")]
    MatchAlreadyExists,
    #[msg("Match not found")]
    MatchNotFound,
    #[msg("Bet already placed")]
    BetAlreadyPlaced,
    #[msg("Bet not found")]
    BetNotFound,
    #[msg("Match not active")]
    MatchNotActive,
    #[msg("Invalid bet amount")]
    InvalidBetAmount,
    #[msg("Match still active")]
    MatchStillActive,
    #[msg("Prizes already distributed")]
    PrizesAlreadyDistributed,
}