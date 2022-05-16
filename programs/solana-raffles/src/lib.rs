use anchor_lang::prelude::*;

declare_id!("4ZEPy6oo8oHzbU6bkiY2m8pLb7aNzyzZaMpAZ6CeZQQf");

#[program]
pub mod solana_raffles {
    use super::*;

    pub fn create_raffle(ctx: Context<CreateRaffle>, price: u64, ends: i64, title: String, description: String) -> Result<()> {

        if title.chars().count() > 50 {
            return Err(RaffleError::InputError.into())
        }
        if description.chars().count() > 100 {
            return Err(RaffleError::InputError.into())
        }

        let raffle = &mut ctx.accounts.raffle;
        raffle.authority = ctx.accounts.authority.key();
        raffle.ends = ends;
        raffle.price = price;
        raffle.title = title;
        raffle.description = description;
        Ok(())
    }

    pub fn purchase_ticket(ctx: Context<CreateTicket>) -> Result<()> {
        
        let clock: Clock = Clock::get().unwrap();
        let raffle  = &ctx.accounts.raffle;

        if raffle.ends < clock.unix_timestamp {
            return Err(RaffleError::RaffleEnded.into());
        }

        let ticket = &mut ctx.accounts.ticket;
        ticket.raffle = ctx.accounts.raffle.key();
        ticket.participant = ctx.accounts.participant.key();
        Ok(())
    }

    pub fn end_raffle(_ctx: Context<EndRaffle>) -> Result<()> { Ok(()) }
 
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Raffle::LEN
    )]
    pub raffle: Box<Account<'info, Raffle>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTicket<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub participant: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub raffle: Box<Account<'info, Raffle>>,

    #[account(
        init,
        payer = participant,
        space = 8 + Ticket::LEN
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndRaffle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
        close = authority 
    )]
    pub raffle: Box<Account<'info, Raffle>>,
}

#[account]
pub struct Raffle {
    pub authority: Pubkey,
    pub ends: i64,
    pub price: u64,

    pub title: String, // 50 * 4
    pub description: String, // 100 * 4
}

#[account]
pub struct Ticket {
    pub raffle: Pubkey,
    pub participant: Pubkey
}

impl Raffle {
    pub const LEN: usize = 32 + 16 + 8 + 50 * 4 + 100 * 4;
}

impl Ticket {
    pub const LEN: usize = 32 + 32;
}

#[error_code]
pub enum RaffleError {
    #[msg("Raffle Has Ended")]
    RaffleEnded,
    #[msg("Input Error")]
    InputError,
}