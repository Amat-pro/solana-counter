use anchor_lang::prelude::*;

declare_id!("By1LmF3KvZYupqCdEqzqfRx1HFAyEUEj3k796ritfdgi");

#[program]
pub mod solana_counter {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).unwrap();

        // event
        emit!(EventCounterIncrease {
            count: counter.count
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 8)] // 8字节用于账户头 + 8字节用于u64
    pub counter: Account<'info, Counter>, // Account 用来访问 本 Program 自己管理的数据账户
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[event]
pub struct EventCounterIncrease {
    count: u64,
}

// Account 用来访问 本 Program 自己管理的数据账户
// InterfaceAccount 用来访问 外部 Program（比如 SPL Token Program）定义的账户
