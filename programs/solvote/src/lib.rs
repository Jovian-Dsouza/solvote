use anchor_lang::prelude::*;

declare_id!("6kmJTPwSUH7VNZiToz2z2vtaszBG57khoVziz7ibHv6A");

#[program]
pub mod solvote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
