use anchor_lang::prelude::*;

declare_id!("2uEqPRsMGp3xjATCpW2MsGVBM1vv1SWGyxHbiXPxn7jr");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
