use anchor_lang::prelude::*;

#[program]
pub mod hello_world {
    use super::*;
    pub fn greet(ctx: Context<Greet>) -> ProgramResult {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Greet {}
