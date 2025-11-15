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
    pub mint_lp: Account<'info, Mint>,

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

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn deposit(
    &mut self,
    amount: u64,
    max_x: u64,
    max_y: u64
) -> Result<()>{

    require!(self.config.locked == false, AmmError::PoolLocked);

    require!(amount ! = 0, AmmError::InvalidAmount);

    let (x,y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
        true => (max_x, max_y),

        false => {
            let amounts = ConstantProduct::xy_deposit_amount_from_lp(
                self.vault_x.amount,
                self.vault_y.amount,
                self.mint_lp.amount,
                amount,
                6
            ).unwrap();
            (amount.x, amount.y)
        }
    };
}