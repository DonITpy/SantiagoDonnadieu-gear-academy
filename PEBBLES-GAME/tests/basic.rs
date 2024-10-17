use gtest::{Program, System};

#[gtest]
mod pebbles_game_tests {
    use super::*;

    #[test]
    fn test_init() {
        let system = System::new();
        let program = Program::new();
    
        // Test valid initialization
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        };
        program.send(init_data);
    
        // Check that the game state is initialized correctly
        let state: GameState = program.state();
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 2);
        assert_eq!(state.pebbles_remaining, 15);
        assert!(state.winner.is_none());
    
        // Test invalid initialization (zero pebbles)
        let invalid_init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 0,
            max_pebbles_per_turn: 2,
        };
        let result = program.send(invalid_init_data);
        assert!(result.is_err()); // Expect an error
    }

    #[test]
    fn test_handle() {
        let system = System::new();
        let program = Program::new();
    
        // Initialize the game
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        };
        program.send(init_data);
    
        // User's turn - valid move
        let action = PebblesAction::Turn(2);
        program.send(action);
    
        // Check the state after the user's turn
        let state: GameState = program.state();
        assert_eq!(state.pebbles_remaining, 13); // 15 - 2
    
        // User wins
        program.send(PebblesAction::Turn(13)); // User takes the last pebbles
        let event: PebblesEvent = program.reply();
        assert!(matches!(event, PebblesEvent::Won(Player::User )));
    
        // Program's turn (simulate)
        let action = PebblesAction::Turn(1);
        program.send(action);
    
        // Check if the program is handling the turn correctly
        let state: GameState = program.state();
        assert_eq!(state.pebbles_remaining, 12); // Assuming the program's logic has been implemented
    
        // User gives up
        program.send(PebblesAction::GiveUp);
        let event: PebblesEvent = program.reply();
        assert!(matches!(event, PebblesEvent::Won(Player::Program)));
    }

    #[test]
    fn test_state() {
        let system = System::new();
        let program = Program::new();
    
        // Initialize the game
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        };
        program.send(init_data);
    
        // Check the initial state
        let state: GameState = program.state();
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 2);
        assert_eq!(state.pebbles_remaining, 15);
        assert!(state.winner.is_none());
    
        // Simulate some turns
        program.send(PebblesAction::Turn(1));
        let state_after_turn: GameState = program.state();
        assert_eq!(state_after_turn.pebbles_remaining, 14); // 15 - 1
    
        // User gives up
        program.send(PebblesAction::GiveUp);
        let state_after_give_up: GameState = program.state();
        assert_eq!(state_after_give_up.winner, Some(Player::Program));
    }
}