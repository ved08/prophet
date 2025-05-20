use crate::state::Market;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
#[derive(Accounts)]
#[instruction(market_id: u64)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        space = 8 + Market::INIT_SPACE,
        payer = signer,
        seeds = [b"market", market_id.to_be_bytes().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"vault", market.key().as_ref()],
        bump,
    )]
    pub market_vault: SystemAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeMarket<'info> {
    pub fn initalize_market(
        &mut self,
        market_id: u64,
        bumps: &InitializeMarketBumps,
    ) -> Result<()> {
        self.market.set_inner(Market {
            market_id,
            fee_collector: self.signer.to_account_info().key(),
            ended: false,
            total_amount: 0,
            winner_project_id: None,
            bump: bumps.market,
        });
        let lamports = self.rent.minimum_balance(0);
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.market_vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, lamports)?;
        Ok(())
    }
}
