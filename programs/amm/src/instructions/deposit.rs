use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    toke::{mint_to, transfer_checked, Mint, MintTo, Token, TokenAccount, TransferChecked}
};

use constant_product_curve:: ConstantProduct;

use crates::errors::Ammerror;
use crates::config::Config;

#[derive(Accounts)]
pub struct Deposit<'info>{
    
    #[accounts(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,

    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub config: Account<'info, config>,

    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_propgram = token_propgram
    )]
    pub vault_y: Account<'info, TokenAccount>,

    
}