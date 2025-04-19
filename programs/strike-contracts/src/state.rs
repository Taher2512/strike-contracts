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

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub user: Pubkey,
    pub match_id: u64,
    pub amount: u64,
    #[max_len(64)]
    pub team_id: String,
    pub bet_id: u64,
    pub timestamp: i64,
    pub is_distributed: bool,
}