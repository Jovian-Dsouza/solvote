use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Dao {
    pub total_proposals: u64,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub desc: String,
    pub options: Vec<VoteOption>,
}
impl Proposal {
    const MAXIMUM_TITLE_LENGTH : usize= 30;
    const MAXIMUM_DESC_LENGTH: usize = 150;
    const MAXIMUM_OPTIONS: usize = 10;
    pub const MAX_SIZE: usize = 8 + Self::MAXIMUM_TITLE_LENGTH + Self::MAXIMUM_DESC_LENGTH + 4 + (Self::MAXIMUM_OPTIONS * (32 + 8));
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VoteOption {
    pub name: String,
    pub count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct UserDao {
    pub points: u64, //Reward points
}


#[account]
#[derive(InitSpace)]
pub struct UserVote { //Derived with user key and proposal id
    pub vote: Option<u64>, 
}
