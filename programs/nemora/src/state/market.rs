use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub market_id: u64,
    pub fee_collector: Pubkey,
    pub ended: bool,
    pub total_amount: u64,
    pub winner_project_id: Option<u64>,
    pub bump: u8,
}
#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub bet_id: u64,
    pub user: Pubkey,
    pub market: Pubkey,
    pub project_id: u64,
    pub amount: u64,
}
