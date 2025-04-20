use anchor_lang::prelude::*;

use crate::{CustomError, MatchAccount, Pool};

pub fn end_match(ctx: Context<EndMatch>, match_id: u64) -> Result<()> {
    let match_account = &mut ctx.accounts.match_account;
    let pool = &mut ctx.accounts.pool;

    require!(pool.authority == ctx.accounts.authority.key(), CustomError::Unauthorized);

    require!(match_account.match_id == match_id, CustomError::InvalidMatchId);
    require!(match_account.is_active, CustomError::MatchNotActive);

    match_account.is_active = false;

    Ok(())
}

#[derive(Accounts)]
#[instruction(match_id: u64)]
pub struct EndMatch<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [b"match", pool.key().as_ref(), &match_id.to_le_bytes()],
        bump,
    )]
    pub match_account: Account<'info, MatchAccount>,

    #[account(
        constraint = pool.authority == authority.key(),
    )]
    pub authority: Signer<'info>,
}