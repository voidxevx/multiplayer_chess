use std::io::{self, ErrorKind};


type BoardLayer = u64;
static BOARD_LAYERS: usize = 3;

#[repr(C, packed)]
#[derive(Debug)]
pub struct BoardLocation {
    x: u32,
    y: u32,
}

impl BoardLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    pub fn transpose(&self) -> Option<u32> {
        if self.x > 8 || self.y > 8 {
            return None;
        }
        Some((self.y * 8) + self.x)
    }
}

pub struct Board {
    layers: Vec<BoardLayer>,
    layer_types: Vec<(String, String)>,
    teams: BoardLayer,
}

impl Board {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_types: Vec::new(),
            teams: 0,
        }
    }

    pub fn add_layer(&mut self, char_w: String, char_b: String) {
        self.layers.push(0);
        self.layer_types.push((char_w, char_b));
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

    pub fn print_board(&self) -> Vec<Vec<&str>> {
        let mut stream: Vec<Vec<&str>> = Vec::new();

        for y in 0..8 {
            let mut layer: Vec<&str> = Vec::new();
            for x in 0..8 {
                let pos = BoardLocation::new(x, y)
                    .transpose()
                    .expect("Board goes out of range while formatting. This should be imposible");
                let mut ch: &str = " ";
                let mut i = 0;
                for lay in &self.layers {
                    let stat = ((lay >> pos) & 0b1) as u8;
                    let team = ((self.teams >> pos) & 0b1) as u8;
                    if stat == 1 {
                        if team == 1 {
                            ch = self.layer_types[i].0.trim();
                        } else {
                            ch = self.layer_types[i].1.trim();
                        }
                        break;
                    }
                    i += 1;
                }

                if ((x + y % 2) % 2 == 0) {
                    layer.push("\x1b[40m");
                    layer.push(ch);
                } else {
                    layer.push("\x1b[47m");
                    layer.push(ch);
                }
                layer.push(" \x1b[0m");
            }
            stream.push(layer);
        }

        stream
    }
}