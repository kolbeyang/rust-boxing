use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
pub enum MoveX {
    Left,
    None,
    Right,
}

impl MoveX {
    pub fn to_num(&self) -> i32 {
        match self {
            MoveX::Left => -1,
            MoveX::Right => 1,
            MoveX::None => 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveY {
    Back,
    None,
}

impl MoveY {
    pub fn to_num(&self) -> i32 {
        match self {
            MoveY::None => 0,
            MoveY::Back => -1,
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Control {
    pub move_x: MoveX,
    pub move_y: MoveY,
    pub left_punch: bool,
    pub right_punch: bool,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            move_x: MoveX::None,
            move_y: MoveY::None,
            left_punch: false,
            right_punch: false,
        }
    }
}

impl Control {
    pub fn to_int(&self) -> usize {
        match self {
            // move_x: None, move_y: None
            Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            } => 0,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            } => 1,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            } => 2,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            } => 3,

            // move_x: None, move_y: Back
            Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            } => 4,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            } => 5,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            } => 6,
            Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            } => 7,

            // move_x: Left, move_y: None
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            } => 8,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            } => 9,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            } => 10,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            } => 11,

            // move_x: Left, move_y: Back
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            } => 12,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            } => 13,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            } => 14,
            Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            } => 15,

            // move_x: Right, move_y: None
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            } => 16,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            } => 17,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            } => 18,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            } => 19,

            // move_x: Right, move_y: Back
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            } => 20,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            } => 21,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            } => 22,
            Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            } => 23,
        }
    }

    pub fn from_int(action: usize) -> Self {
        match action {
            0 => Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            },
            1 => Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            },
            2 => Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            },
            3 => Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            },
            4 => Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            },
            5 => Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            },
            6 => Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            },
            7 => Self {
                move_x: MoveX::None,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            },
            8 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            },
            9 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            },
            10 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            },
            11 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            },
            12 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            },
            13 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            },
            14 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            },
            15 => Self {
                move_x: MoveX::Left,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            },
            16 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            },
            17 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: false,
            },
            18 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: true,
            },
            19 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::None,
                left_punch: true,
                right_punch: true,
            },
            20 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: false,
            },
            21 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: false,
            },
            22 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: false,
                right_punch: true,
            },
            23 => Self {
                move_x: MoveX::Right,
                move_y: MoveY::Back,
                left_punch: true,
                right_punch: true,
            },
            _ => Self {
                move_x: MoveX::None,
                move_y: MoveY::None,
                left_punch: false,
                right_punch: false,
            }, // Default to no action
        }
    }
}
