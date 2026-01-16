use std::io::{ErrorKind};
use chess_shared::game::{board_format::LayerGroup, board_location::BoardLocation};

type BoardLayer = u64;
static BOARD_LAYERS: usize = 3;

pub struct Board {
    layers: Vec<BoardLayer>,
    teams: BoardLayer,
}

impl Board {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            teams: 0,
        }
    }

    pub fn get_layer_group(&mut self) -> LayerGroup {
        LayerGroup::new(self.layers.clone(), self.teams.clone())
    }

    pub fn collapse_layers(&self) -> BoardLayer {
        let mut sum: BoardLayer = 0;
        for layer in &self.layers {
            sum |= layer;
        }
        sum
    }

    pub fn toggle_location_active(&mut self, layer: usize, location: &BoardLocation) {
        if let Some (pos) = location.transpose() {
            let shadow_board = 1 << pos;
            self.layers[layer] ^= shadow_board;
        }
    }

    pub fn set_location_team(&mut self, location: &BoardLocation, team: u8) {
        if let Some(pos) = location.transpose() {
            let shadow_board = (team as u64) << pos;
            self.teams ^= shadow_board;
        }
    }

    
}