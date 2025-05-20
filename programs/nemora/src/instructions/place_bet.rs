use crate::errors::MarketErrors;
use crate::state::{Bet, Market};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
#[derive(Accounts)]
#[instruction(bet_id: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub bettor: Signer<'info>,
    #[account(
        mut,
        seeds = [b"market", market.market_id.to_le_bytes().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = bettor,
        seeds = [b"bet", market.market_id.to_le_bytes().as_ref()],
        space = 8 + Bet::INIT_SPACE,
        bump,
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        mut,
        seeds = [b"vault", market.key().as_ref()],
        bump,
    )]
    pub market_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, bet_id: u64, project_id: u64, amount: u64) -> Result<()> {
        require!(self.market.ended == false, MarketErrors::MarketExpiredError);
        require!(amount > 0, MarketErrors::AmountCannotBeNegative);

        self.bet.set_inner(Bet {
            bet_id,
            user: self.bettor.to_account_info().key(),
            market: self.market.key(),
            project_id,
            amount,
        });
        let accounts = Transfer {
            from: self.bettor.to_account_info(),
            to: self.market_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)?;
        Ok(())
    }
}
