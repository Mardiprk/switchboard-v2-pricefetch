use anchor_lang::prelude::*;

declare_id!("5NMAuN2UdTN5Qm1RYwUJ6HEGhzGSDeAca1TM2bFcKdqH");

#[program]
pub mod switchboard {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
