use anchor_lang::prelude::*;

pub mod errors;
pub use errors::ErrorCode;

pub mod state;
pub use state::*;

pub mod contexts;
use contexts::*;

declare_id!("6kmJTPwSUH7VNZiToz2z2vtaszBG57khoVziz7ibHv6A");

#[program]
pub mod solvote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.total_proposals = 0;
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, desc: String, options: Vec<String>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;

        proposal.title = title;
        proposal.desc = desc;
        proposal.options = options.into_iter()
                                    .map(|opt| VoteOption { name: opt, count: 0})
                                    .collect();

        dao.total_proposals += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _proposal_id: u64, option_index: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let user_dao = &mut ctx.accounts.user_dao;
        
        // todo add check for valid option
        // if proposal.voters.contains(&user.key()) {
        //     return Err(ErrorCode::AlreadyVoted.into());
        // }

        proposal.options[option_index as usize].count += 1;

        user_dao.points += 1;
        Ok(())
    }
}



