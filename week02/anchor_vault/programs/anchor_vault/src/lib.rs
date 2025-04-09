use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("7raUqiDQw7dEb8jg2g6RfoJX1NHuvzrku8sLouuCZ3p3");

#[program]
pub mod anchor_vault {
    use super::*;
    use anchor_lang::context;

    

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount:u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Payments>, amount:u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
        }


    pub fn close(ctx: Context<Close>) -> Result<()>{
        ctx.accounts.close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump,
    )]
   pub state: Account<'info, VaultState >,

   #[account(
    seeds = [b"vault", state.key().as_ref()],
    bump
   )]
   pub vault : SystemAccount<'info>,

   pub system_program : Program<'info , System>

}

impl <'info> Initialize<'info> {
    pub fn initialize(&mut self , bumps: &InitializeBumps) -> Result<()>{

        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        self.state.amount = 0;

        Ok(())

    }
    
    
}

#[derive(Accounts)]
pub struct Payments<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state" , user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info , VaultState>,


    #[account(
        seeds = [b"vault" , user.key().as_ref()],
        bump = state.vault_bump
    )]

    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
} 


impl <'info> Payments<'info> {

    pub fn deposit(&mut self, amount:u64 ) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        self.state.amount += amount;

        Ok(())

    }
    


pub fn withdraw(&mut self, amount:u64) -> Result<()>{
    let cpi_program = self.system_program.to_account_info();

    let cpi_account = Transfer {
        from: self.vault.to_account_info(),
        to: self.user.to_account_info()
    };
    let seeds = &[
        b"vault",
        self.state.to_account_info().key.as_ref(),
        &[self.state.vault_bump],

    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

    transfer(cpi_context, amount)?;

    Ok(())
}
}

#[derive(Accounts)]
pub struct Close<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state" , user.key().as_ref()],
        bump = state.state_bump,
        close = user
    )]
    pub state: Account<'info , VaultState>,


    #[account(
        mut,
        seeds = [b"state" , user.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,

}

impl<'info> Close<'info> {
    pub fn close(&mut self)-> Result<()>{
        let balance = self.vault.get_lamports();
        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
        b"vault",
        self.state.to_account_info().key.as_ref(),
        &[self.state.vault_bump],

    ];
    let signer_seeds = &[&seeds[..]];
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    transfer(cpi_context, balance)?;

    Ok(())
    }
    
}


#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump : u8,
    pub state_bump: u8,
    pub amount:u64
}