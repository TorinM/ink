// Module to control the gap buffer

struct GapBuffer {
    size: u8,
    gap_left: u8,
    gap_right: u8,
    buffer: [u8]
}

impl GapBuffer {
    // Moves the gap left in the array
    fn left(&self) {
    }

    // Moves the gap right in the array
    fn right(&self) {
    }
    
    // Controls the growth of the size of the buffer
    fn grow(&self) {
    }
    
    // Controls the movement of the gap linked to the cursor insertion
    fn move_cursor(&self) {
    }
    
    // Controls data insertion operations
    fn insert(&self) {
    }
}
