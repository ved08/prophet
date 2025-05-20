use anchor_lang::prelude::*;

declare_id!("8ZDbq8atQsKTx6ag2v2ZgFa79L5kc6KgisyoCnxmxGEZ");
mod instructions;
use instructions::*;
mod errors;
mod state;
#[program]
pub mod nemora {
    use super::*;

    pub fn initalize_market(ctx: Context<InitializeMarket>, market_id: u64) -> Result<()> {
        ctx.accounts.initalize_market(market_id, &ctx.bumps)?;
        Ok(())
    }
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        bet_id: u64,
        project_id: u64,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.place_bet(bet_id, project_id, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
