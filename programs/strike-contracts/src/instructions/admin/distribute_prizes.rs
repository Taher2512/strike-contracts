use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

use crate::error::CustomError;

use crate::{MatchAccount, Pool};

pub fn distribute_prizes(ctx: Context<DistributePrizes>, match_id: u64, first_winner: Pubkey, second_winner: Pubkey, third_winner: Pubkey) -> Result<()> {
    let match_account = &mut ctx.accounts.match_account;
    let pool = &mut ctx.accounts.pool;
    let authority = &ctx.accounts.authority;

    require!(pool.authority == authority.key(), CustomError::Unauthorized);

    require!(match_account.match_id == match_id, CustomError::InvalidMatchId);
    require!(!match_account.is_active, CustomError::MatchStillActive);
    require!(!match_account.is_distributed, CustomError::PrizesAlreadyDistributed);

    let total_prize = match_account.total_amount;
    let first_prize = total_prize.checked_mul(50).unwrap().checked_div(100).unwrap();
    let second_prize = total_prize.checked_mul(30).unwrap().checked_div(100).unwrap();
    let third_prize = total_prize.checked_mul(10).unwrap().checked_div(100).unwrap();
    let platform_fee = total_prize.checked_mul(10).unwrap().checked_div(100).unwrap();

    // Transfer Prizes
    invoke(
        &system_instruction::transfer(
            &pool.key(),
            &first_winner,
        first_prize
        ),
        &[
            pool.to_account_info(),
            ctx.accounts.first_winner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    invoke(
        &system_instruction::transfer(
            &pool.key(),
            &second_winner,
        second_prize
        ),
        &[
            pool.to_account_info(),
            ctx.accounts.second_winner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    invoke(
        &system_instruction::transfer(
            &pool.key(),
            &third_winner,
        third_prize
        ),
        &[
            pool.to_account_info(),
            ctx.accounts.third_winner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer Platform Fee
    invoke(
        &system_instruction::transfer(
            &pool.key(),
            &authority.key(),
        platform_fee
        ),
        &[
            pool.to_account_info(),
            authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    match_account.is_distributed = true;
    pool.total_amount = pool.total_amount.checked_sub(total_prize).unwrap();
    match_account.total_amount = 0;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(match_id: u64)]
pub struct DistributePrizes<'info> {
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

    /// CHECK: Just receiving funds
    #[account(mut)]
    pub first_winner: AccountInfo<'info>,

    /// CHECK: Just receiving funds
    #[account(mut)]
    pub second_winner: AccountInfo<'info>,

    /// CHECK: Just receiving funds
    #[account(mut)]
    pub third_winner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}