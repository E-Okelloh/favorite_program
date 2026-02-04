use anchor_lang::prelude::*;

// Your program's unique ID (get from `anchor keys list` after first build)
declare_id!("ww9C83noARSQVBnqmCUmaVdbJjmiwcV9j2LkXYMoUCV");

#[program]
pub mod favorite_program {  // Module name MUST match Cargo.toml
    use super::*;

    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        let user_pubkey = ctx.accounts.user.key();
        msg!("User {} set favorites!", user_pubkey);
        msg!("Number: {}, Color: {}", number, color);
        msg!("Hobbies: {:?}", hobbies);

        // Save data to PDA
        let favorites = &mut ctx.accounts.favorites;
        favorites.number = number;
        favorites.color = color;
        favorites.hobbies = hobbies;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8 + 4 + 200 + 4 + 5*50,  // Disc + u64 + vec len + color + hobbies len + hobbies
        seeds = [b"favorites", user.key().as_ref()],  // FIXED: "seeds" (not sedds)
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}
