use anchor_lang::prelude::*;

use crate::error::CustomError;

use crate::state::{Pool, MatchAccount};

pub fn process_create_match(ctx: Context<CreateMatch>, match_id: u64) -> Result<()> {
    let match_account = &mut ctx.accounts.match_account;
    let pool = &mut ctx.accounts.pool;

    require!(pool.authority == ctx.accounts.authority.key(), CustomError::Unauthorized);

    match_account.match_id = match_id;
    match_account.total_bets = 0;
    match_account.total_amount = 0;
    match_account.is_active = true;
    match_account.is_distributed = false;
    pool.match_id = match_id;

    Ok(())
}

#[derive(Accounts)]
#[instruction(match_id: u64)]
pub struct CreateMatch<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = authority,
        space = 8 + MatchAccount::INIT_SPACE,
        seeds = [b"match", pool.key().as_ref(), &match_id.to_le_bytes()],
        bump,
    )]
    pub match_account: Account<'info, MatchAccount>,

    #[account(
        mut,
        constraint = pool.authority == authority.key(),
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}