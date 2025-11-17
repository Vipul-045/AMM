use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::states::Config;

#[derive(Accounts)]
pub struct Swap<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_x : Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_y : Account<'info, Mint>,

    pub config : Account<'info, Config>,

    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x : Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y : Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>, 
}

pub fn Swap(&mut self, is_x: bool, amount_in:u64, min_amount_out: u64) -> Result <()> {
    require!(!self.config.locked , AmmError::PoolLoacked);
    require!(amount_in > 0, AmmError::InvalidAmount);

    let mut curve = ConstantProduct::init(
        self.vault_x.amount,
        self.vault_y.amount,
        self.mint_lp.supply,
        self.config.fee,
        None,
    ).map_err(AmmError::from)?;

    let p = match is_x{
        true => LiquidityPair::X,
        false => LiquidityPair::Y
    };

    let swap_result = curve.swap(p, amount_in, min_amount_out).map_err(AmmError::from)?;

    require!(swap_result.deposit != 0, AmmError::InvalidAmount);
    require!(swap_result.withdrawn != 0, AmmError::InvalidAmount);

    self.deposit_tokens(is_x, swap_result.deposit)?;
    self.withdraw_tokens(is_x, swap_result.withdraw)?;

    Ok(())
}
