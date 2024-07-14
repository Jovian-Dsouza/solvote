use anchor_lang::prelude::*;
use crate::Dao;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + Dao::INIT_SPACE)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
