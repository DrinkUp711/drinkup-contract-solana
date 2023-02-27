use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod drink_challenge_task {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn start_challenge(ctx: Context<StartChallenge>) -> Result<()> {
        let first_owner = &mut ctx.accounts.first_owner;
        let clock = Clock::get()?;
        // challenge_time == 0 => first_owner is not initialized, then store the first owner
        if first_owner.challenge_time == 0 {
            first_owner.nft_mint = ctx.accounts.nft_mint.key();
            first_owner.owner = ctx.accounts.owner.key();
            first_owner.holder = ctx.accounts.holder.key();
            first_owner.challenge_time = clock.unix_timestamp;
            first_owner.bump = *ctx.bumps.get("first_owner").unwrap();
        } else {
            // TODO: check - if the first_owner is initialized, then first_owner.owner should equal to ctx.accounts.owner.key()
        }

        // if first_owner.nft_mint == *ctx.accounts.system_program.key {
        //
        // }

        let challenge_nft_list = &mut ctx.accounts.challenge_nft_list;

        // challenge_nft_list.owner == *ctx.accounts.system_program.key PublicKey(11111111111111111111111111111111)
        // challenge_nft_list is not initialized
        if challenge_nft_list.owner == *ctx.accounts.system_program.key {
            challenge_nft_list.owner = ctx.accounts.owner.key();
            challenge_nft_list.bump = *ctx.bumps.get("challenge_nft_list").unwrap();
        }

        // TODO: check if nft_mint exists
        challenge_nft_list.nft_list.push(ChallengeNFT {
            holder: ctx.accounts.holder.key(),
            nft_mint: ctx.accounts.nft_mint.key(),
            challenge_time: clock.unix_timestamp,
        });

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
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    // nft holder associated token account
    #[account(
    has_one = owner
    )]
    pub holder: Account<'info, TokenAccount>,

    #[account(
    init_if_needed,
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
    init_if_needed,
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
    pub nft_mint: Account<'info, Mint>,

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
    // https://book.anchor-lang.com/anchor_references/space.html
    const LEN: usize = 8 + 32 + 32 + 32 + 8 + 1;
}

impl ChallengeNFTList {
    // https://book.anchor-lang.com/anchor_references/space.html
    // max challenge nft list length is 10
    const LEN: usize = 8 + 32 + (4 + 10 * (32 + 32 + 8)) + 1;
}