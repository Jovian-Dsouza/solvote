use anchor_lang::prelude::*;
use crate::Dao;
use crate::Proposal;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(
              init, 
              payer = user, 
              space = Proposal::MAX_SIZE, 
              seeds = [dao.key().as_ref(), dao.total_proposals.to_le_bytes().as_ref()], 
              bump
            )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
