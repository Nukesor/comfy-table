pub struct Column {
    // Left padding for each cell of this column in spaces
    padding_left: u32,
    // Right padding for each cell of this column in spaces
    padding_right: u32,
}

impl Column {
    pub fn new() -> Self {
        Column {
            padding_left: 1,
            padding_right: 1,
        }
    }
}
