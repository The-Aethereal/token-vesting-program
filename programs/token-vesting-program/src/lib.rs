use anchor_lang::prelude::*;

declare_id!("4uhEjSs5JGewiu49UFxDU6DSxAVfcg2z33n4y31Boukj");

#[program]
pub mod token_vesting_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
