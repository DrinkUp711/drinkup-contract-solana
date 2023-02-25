use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod drink_challenge_task {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn start_challenge(ctx: Context<StartChallenge>) -> Result<()> {
        // TODO: start_challenge
        Ok(())
    }

    pub fn end_challenge(ctx: Context<EndChallenge>) -> Result<()> {
        // TODO: end_challenge
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct StartChallenge<'info> {
    // nft owner
    #[account(mut)]
    pub owner: Signer<'info>,

    // nft mint address(token address)
    pub nft_mint: Account<'info, TokenAccount>,

    // nft holder associated token account
    #[account(
    has_one = owner
    )]
    pub holder: Account<'info, TokenAccount>,

    #[account(
    init,
    payer = owner,
    space = FirstOwner::LEN,
    seeds = [
    b"first-owner".as_ref(),
    nft_mint.key().as_ref()
    ],
    bump
    )]
    pub first_owner: Account<'info, FirstOwner>,

    #[account(
    init,
    payer = owner,
    space = ChallengeNFTList::LEN,
    seeds = [
    b"challenge-nft-list".as_ref(),
    owner.key().as_ref()
    ],
    bump
    )]
    pub challenge_nft_list: Account<'info, ChallengeNFTList>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndChallenge<'info> {
    // nft owner
    #[account(mut)]
    pub owner: Signer<'info>,

    // nft mint address(token address)
    pub nft_mint: Account<'info, TokenAccount>,

    // nft holder associated token account
    pub holder: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [
    b"challenge-nft-list".as_ref(),
    owner.key().as_ref()
    ],
    bump = challenge_nft_list.bump
    )]
    pub challenge_nft_list: Account<'info, ChallengeNFTList>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct FirstOwner {
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub holder: Pubkey,
    pub challenge_time: i64,
    pub bump: u8,
}

#[account]
pub struct ChallengeNFTList {
    pub owner: Pubkey,
    pub nft_list: Vec<ChallengeNFT>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct ChallengeNFT {
    pub holder: Pubkey,
    pub nft_mint: Pubkey,
    pub challenge_time: i64,
}

impl FirstOwner {
    const LEN: usize = 8 + 32 + 32 + 8 + 1;
}

impl ChallengeNFTList {
    // max challenge nft list length is 10
    const LEN: usize = 8 + 10 * (32 + 32 + 8) + 8;
}