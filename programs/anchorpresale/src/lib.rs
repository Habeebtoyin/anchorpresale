use anchor_lang::prelude::*;

declare_id!("4rucZkUgtAGLjPsiTtTepa3pGcZtPs43Q11BsYi4wRF7");

#[program]
pub mod anchorpresale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
