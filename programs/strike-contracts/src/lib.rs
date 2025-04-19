use anchor_lang::prelude::*;

use state::*;
mod state;
use instructions::*;
mod instructions;
use error::*;
mod error;

declare_id!("9NsJPaJigfHd99L9py29Gqpn4uXbDxzpDNWZgbViymko");

#[program]
pub mod strike_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
