use crate::game::board_location::BoardLocation;

pub struct LayerGroup {
    layers: Vec<u64>,
    teams: u64,
}

impl LayerGroup {
    pub fn new(layers: Vec<u64>, teams: u64) -> Self {
        Self {
            layers: layers,
            teams: teams,
        }
    }

    pub fn make_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let teams_bytes: [u8; 8] = u64::to_ne_bytes(self.teams);
        bytes.append(&mut teams_bytes.to_vec());
        
        for layer in self.layers {
            let layer_bytes: [u8; 8] = u64::to_ne_bytes(layer);
            bytes.append(&mut layer_bytes.to_vec());
        }
        

        bytes
    }

    pub fn from_bytes(mut bytes: Vec<u8>) -> Self {
        let teams;
        if let Ok(bytes) = bytes[0..8].try_into() {
            teams = u64::from_ne_bytes(bytes);
        } else {
            teams = 0;
        }

        let mut next_bytes = bytes.split_off(8);
        let mut layers: Vec<u64> = Vec::new();
        while next_bytes.len() > 0 {
            if let Ok(bytes) = next_bytes[0..8].try_into() {
                let layer = u64::from_ne_bytes(bytes);
                layers.push(layer);
            }

            next_bytes = next_bytes.split_off(8);
        }

        Self {
            teams: teams,
            layers: layers,
        }
    }

}
    
pub fn print_board(layers: &LayerGroup, layer_types: &Vec<(String, String)>, selected: Option<&BoardLocation>) -> String {
    let mut stream: Vec<Vec<&str>> = Vec::new();
    let sel = match selected {
        Some (s) => s.transpose(),
        _ => None,
    };
    for y in 0..8 {
        let mut layer: Vec<&str> = Vec::new();
        for x in 0..8 {
            let pos = BoardLocation::new(x, y)
                .transpose()
                .expect("Board goes out of range while formatting. This should be imposible");
            let mut ch: &str = " ";
            let mut i = 0;
            for lay in &layers.layers {
                let stat = ((lay >> pos) & 0b1) as u8;
                let team = ((layers.teams >> pos) & 0b1) as u8;
                if stat == 1 {
                    if team == 1 {
                        ch = layer_types[i].0.trim();
                    } else {
                        ch = layer_types[i].1.trim();
                    }
                    break;
                }
                i += 1;
            }

            if let Some(sel) = sel && sel == pos {
                if (x + y % 2) % 2 == 0 {
                    layer.push("\x1b[46m");
                } else {
                    layer.push("\x1b[45m");
                }
            } else {
                if (x + y % 2) % 2 == 0 {
                    layer.push("\x1b[40m");
                } else {
                    layer.push("\x1b[47m");
                }
            }

            layer.push(ch);
            layer.push(" \x1b[0m");
        }
        stream.push(layer);
    }

    let mut full: String = String::new();

    for row in stream {
        for col in row {
            full += col;
        }
        full += "\n";
    }

    full
}