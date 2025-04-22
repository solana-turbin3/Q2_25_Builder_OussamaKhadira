#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;

use  instructions::*;

declare_id!("CYjxn7UHUbJYpZbAPhYoQTt3zxq3aZTi9rfRjTqHwbmb");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize> , 
        seed:u64,
        fee:u16,
        authorithy:Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authorithy, ctx.bumps)
        
    }

    pub fn deposit(
        ctx: Context<Deposit> , 
        amount:u64,
        max_x : u64,
        max_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount , max_x , max_y)
        
    }
}

