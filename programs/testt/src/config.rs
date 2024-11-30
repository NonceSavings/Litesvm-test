use crate::{constants::*, errors::TokenMillError, state::TokenMillConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(init, payer = payer, space = 8 + TokenMillConfig::INIT_SPACE)]
    pub config: Account<'info, TokenMillConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateConfig>,
    authority: Pubkey,
    protocol_fee_recipient: Pubkey,
    protocol_fee_share: u16,
    referral_fee_share: u16,
) -> Result<()> {
    require!(
        protocol_fee_share <= MAX_BPS as u16 && referral_fee_share <= MAX_BPS as u16,
        TokenMillError::InvalidFeeShare
    );

    let config = &mut ctx.accounts.config;

    config.initialize(
        authority,
        protocol_fee_recipient,
        protocol_fee_share,
        referral_fee_share,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // use
}
