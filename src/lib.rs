pub mod frame_manager;
pub  mod text;
#[derive(Debug)]
pub enum RotateDirect {
    North = 0,
    East,
    South,
    West,
}

impl RotateDirect {
    pub fn get_next(&mut self)-> RotateDirect {
        match self {
            RotateDirect::North => {
                return RotateDirect::East;
            }
            RotateDirect::East => {
                return RotateDirect::South;
            }
            RotateDirect::South => {
                return RotateDirect::West;
            }
            RotateDirect::West => {
                return RotateDirect::North;
            }
        }
    }
     pub fn get_prev(&mut self)-> RotateDirect {
        match self {
            RotateDirect::North => {
                return RotateDirect::West;
            }
            RotateDirect::West => {
                return RotateDirect::South;
            }
            RotateDirect::South => {
                return RotateDirect::East;
            }
            RotateDirect::East => {
                return RotateDirect::North;
            }
        }
    }
 }