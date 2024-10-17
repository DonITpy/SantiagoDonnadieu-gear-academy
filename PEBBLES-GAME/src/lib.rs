use gstd::{msg, exec};
use io::{PebblesInit, GameState, Player, DifficultyLevel};

pub fn init() {
    let init_data: PebblesInit = msg::load().expect("Failed to load init data");
    
    // Validate input data
    if init_data.pebbles_count == 0 || init_data.max_pebbles_per_turn == 0 {
        panic!("Invalid pebbles count or max pebbles per turn.");
    }

    // Randomly choose the first player
    let first_player = if exec::random(msg::id().into()).unwrap().0[0] % 2 == 0 {
        Player::User 
    } else {
        Player::Program
    };

    let game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty,
        first_player,
        winner: None,
    };

    // Process the first turn if the first player is Program
    if let Player::Program = first_player {
        // Logic for Program's first turn
        let turn = get_random_u32() % game_state.max_pebbles_per_turn + 1;
        game_state.pebbles_remaining -= turn;
        msg::reply(PebblesEvent::CounterTurn(turn)).expect("Failed to send event");
    }
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load action");
    
    // Validate action
    match action {
        PebblesAction::Turn(pebbles) => {
            // Validate turn
            if pebbles < 1 || pebbles > game_state.max_pebbles_per_turn {
                panic!("Invalid number of pebbles.");
            }

            game_state.pebbles_remaining -= pebbles;

            // Check if the user wins
            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);
            } else {
                // Process the Program's turn
                let turn = if game_state.difficulty == DifficultyLevel::Easy {
                    get_random_u32() % game_state.max_pebbles_per_turn + 1
                } else {
                    // Logic for finding the best pebbles count at the hard level
                    todo!()
                };
                game_state.pebbles_remaining -= turn;
                msg ::reply(PebblesEvent::CounterTurn(turn)).expect("Failed to send event");
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
        }
        PebblesAction::Restart { .. } => {
            // Reset the game state
            init();
        }
    }

    // Send the corresponding PebblesEvent
    let event = match game_state.winner {
        Some(Player::User) => PebblesEvent::Won(Player::User),
        Some(Player::Program) => PebblesEvent::Won(Player::Program),
        None => PebblesEvent::CounterTurn(game_state.pebbles_remaining),
    };
    msg::reply(event).expect("Failed to send event");
}

pub fn state() {
    msg::reply(game_state.clone()).expect("Failed to send game state");
}

