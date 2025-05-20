use anchor_lang::prelude::*;

declare_id!("8ZDbq8atQsKTx6ag2v2ZgFa79L5kc6KgisyoCnxmxGEZ");

#[program]
pub mod nemora {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
