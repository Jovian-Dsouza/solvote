use anchor_lang::prelude::*;
use crate::Proposal;
use crate::UserDao;
use crate::Dao;
use crate::UserVote;

#[derive(Accounts)]
#[instruction(_proposal_id : u64)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [dao.key().as_ref(), _proposal_id.to_le_bytes().as_ref()], 
        bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init_if_needed,
        seeds = [dao.key().as_ref(), user.key().as_ref()],
        payer = user,
        space = 8 + UserDao::INIT_SPACE,
        bump
    )]
    pub user_dao: Account<'info, UserDao>,

    #[account(
        init_if_needed,
        seeds = [proposal.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + UserVote::INIT_SPACE,
    )]
    pub voter: Account<'info, UserVote>,

    #[account()]
    pub dao: Account<'info, Dao>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
