pub enum Expression {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    LoopBegin,
    LoopEnd,
    Output,
    Input,
}

impl Expression {
    fn from_char(byte: u8) -> Option<Self> {
        match byte {
            b'+' => Some(Expression::Increment),
            b'-' => Some(Expression::Decrement),
            b'>' => Some(Expression::MoveRight),
            b'<' => Some(Expression::MoveLeft),
            b'[' => Some(Expression::LoopBegin),
            b']' => Some(Expression::LoopEnd),
            b'.' => Some(Expression::Output),
            b',' => Some(Expression::Input),
            _ => None,
        }
    }

    pub fn from_string(code: &str) -> Vec<Self> {
        code.bytes().filter_map(Expression::from_char).collect()
    }
}
