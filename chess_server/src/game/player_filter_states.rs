use std::{collections::HashMap, hash::Hash, io::{Error, ErrorKind}, net::SocketAddr};

use crate::game::board::Board;

pub struct PlayerID
{
    player_user_name: String,
    address: SocketAddr,
}

pub trait GameState {
    fn move_player(&mut self, id: u64) -> Option<PlayerID>;
    fn add_player(&mut self, id: u64, player: PlayerID);
}

pub enum GameType {
    Multiplayer(MultiplayerGame),
    WaitingLobby(PlayerWaitingLobby),
}


pub struct MultiplayerGame {
    players: HashMap<u64, PlayerID>,
    game_board: Option<Board>,
    /* Other game settings... */
}

impl MultiplayerGame {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            game_board: Some(Board::new()),
        }
    }

    // gets a reference to the board
    pub fn get_board(&self) -> &Option<Board> {
        &self.game_board
    }

    // start the game initializing the board
    pub fn start_game(&mut self) -> std::io::Result<()> {
        match &self.game_board {
            Some(_) => Err(Error::new(ErrorKind::Other, "Game already created")),
            None => {
                self.game_board = Some(Board::new());
                Ok(())
            }
        }
    }
}

impl GameState for MultiplayerGame {
    fn add_player(&mut self, id: u64, player: PlayerID) {
        self.players.insert(id, player);
    }

    fn move_player(&mut self, id: u64) -> Option<PlayerID> {
        return self.players.remove(&id);
    }
}




/* Holds players sin lobby waiting for a game join */
pub struct PlayerWaitingLobby {
    players: HashMap<u64, PlayerID>,
}

impl PlayerWaitingLobby {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}

impl GameState for PlayerWaitingLobby {
    fn add_player(&mut self, id: u64, player: PlayerID) {
        self.players.insert(id, player);
    }

    fn move_player(&mut self, id: u64) -> Option<PlayerID> {
        return self.players.remove(&id);
    }
}