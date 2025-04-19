use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub total_amount: u64,
    pub is_active: bool,
    pub match_id: u64,
}

#[account]
#[derive(InitSpace)]
pub struct MatchAccount {
    pub match_id: u64,
    pub total_bets: u64,
    pub total_amount: u64,
    pub is_active: bool,
    pub is_distributed: bool,
}