use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::{hashv};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgzqVt7Y84hP"); // Remplace par l'ID déployé

#[program]
pub mod pfc {
    use super::*;

    pub fn create_game(
        ctx: Context<CreateGame>,
        hashed_choice: [u8; 32],
        bet_amount: u64,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let player = &ctx.accounts.player;

        require!(bet_amount > 0, CustomError::InvalidBetAmount);

        game.player1 = player.key();
        game.hashed_choice1 = hashed_choice;
        game.bet_amount = bet_amount;
        game.status = GameStatus::WaitingOpponent;
        game.commission_wallet = ctx.accounts.commission_wallet.key();
        game.winner = None;

        // Transfer bet_amount from player to game PDA
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.player_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: player.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi_ctx, bet_amount)?;

        Ok(())
    }

    // ... autres fonctions (join_game, reveal_choice, cancel_game, etc.)
}

// Comptes, erreurs, enum GameStatus etc. ici...

