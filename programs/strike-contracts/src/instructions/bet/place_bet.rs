use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction}};

use crate::{error::CustomError, Bet, MatchAccount, Pool};

pub fn place_bet(ctx: Context<PlaceBet>, match_id: u64, amount: u64, team_id: String) -> Result<()> {
    let match_account = &mut ctx.accounts.match_account;
    let pool = &mut ctx.accounts.pool;
    let bet = &mut ctx.accounts.bet;
    let user = &ctx.accounts.user;

    require!(match_account.is_active, CustomError::MatchNotActive);
    require!(match_account.match_id == match_id, CustomError::InvalidMatchId);
    require!(
        amount == 10_000_000 || // 0.01 SOL
        amount == 50_000_000 || // 0.05 SOL
        amount == 100_000_000 || // 0.1 SOL
        amount == 500_000_000 || // 0.5 SOL
        amount == 1_000_000_000, // 1 SOL
        CustomError::InvalidBetAmount
    );

    // Transfer SOL from user to pool
    invoke(
        &system_instruction::transfer(
            &user.key(),
            &pool.key(),
            amount),
        &[
            user.to_account_info(),
            pool.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Set bet data
    bet.user = user.key();
    bet.match_id = match_id;
    bet.amount = amount;
    bet.team_id = team_id;
    bet.bet_id = match_account.total_bets;
    bet.timestamp = Clock::get()?.unix_timestamp;
    bet.is_distributed = false;

    msg!("FANTASY_CRICKET_BET_ID:{}", bet.bet_id);

    match_account.total_bets += 1;
    match_account.total_amount += amount;

    pool.total_amount += amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(match_id: u64, amount: u64, team_id: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [b"match", pool.key().as_ref(), &match_id.to_le_bytes()],
        bump,
    )]
    pub match_account: Account<'info, MatchAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + Bet::INIT_SPACE,
        seeds = [
            b"bet",
            match_account.key().as_ref(),
            user.key().as_ref(),
            &match_account.total_bets.to_le_bytes()
        ],
        bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}