use anchor_lang::prelude::*;

declare_id!("9EG9CqNvAqNHkrNvNSeCSoDyGGBeSp7mXPd6CyEY4zof");

#[program]
pub mod helloworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
