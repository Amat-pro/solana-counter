use anchor_lang::prelude::*;
use solana_counter::cpi::increment;
use solana_counter::cpi::accounts::IncrementCounter;

declare_id!("4Nzs5xPSUAxPQoA2VWEnyXteuvbhptEGw9x1CVRw94j3");

#[program]
pub mod program_b {
    use super::*;

    pub fn increase(ctx: Context<Increase>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let cpi_accounts = IncrementCounter {
            counter: ctx.accounts.counter.to_account_info(),
        };
        let cpi_program = ctx.accounts.counter_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        increment(cpi_ctx)?; // 调用 solana_counter 的 CPI
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Increase<'info> {
    #[account(mut)]
    pub counter: Account<'info, solana_counter::Counter>,
    #[account(address = solana_counter::ID)]
    pub counter_program: Program<'info, solana_counter::program::SolanaCounter>,
}
