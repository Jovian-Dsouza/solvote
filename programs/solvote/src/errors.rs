use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
}
