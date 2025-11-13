use anchor_lang::prelude::*;

use anchor_spl::{
  associated_token::AssociatedToken,
  token::{Mint, Token, TokenAccount}  
};

use crate::states::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {

    #[acoount(mut)]
    pub admin: signer<'info>,

    pub mint_x : Account<'info, Mint>,

    pub mint_y : Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", seeds.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config : Account<'info, Config>,

    


}