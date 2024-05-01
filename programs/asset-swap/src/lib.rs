use anchor_lang::prelude::*;

pub mod state;
pub mod error;


declare_id!("9Zph5GtVtGeGdzwBMUeC816zqZgPaH53srZEfg1EoqXf");

#[program]
pub mod asset_swap {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
