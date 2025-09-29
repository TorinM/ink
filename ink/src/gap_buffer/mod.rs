// Module to control the gap buffer

struct GapBuffer {
    size: u8,
    gap_left: u8,
    gap_right: u8,
    buffer: Vec<char>
}

impl GapBuffer {
    // Controls the initialization of the array
    fn from_size(size: u8) -> GapBuffer {
        let buffer = Vec::with_capacity(size.into());

        GapBuffer {
            size: size,
            gap_right: 0,
            gap_left: size - 1,
            buffer: buffer
        }
    }
    // Moves the gap left in the array
    fn left(&mut self) {
        
    }

    // Moves the gap right in the array
    fn right(&mut self) {
    }
    
    // Controls the growth of the size of the buffer
    fn grow(&mut self) {
    }
    
    // Controls the movement of the gap linked to the cursor insertion
    fn move_cursor(&mut self, pos: &u8) {
        if *pos < self.gap_right {
            self.right();
        } else {
            self.left()
        }
    }
    
    // Controls data insertion operations
    fn insert(&mut self, pos: u8, value: char) {
        
        if pos != self.gap_left {
            self.move_cursor(&pos);
        }

        if self.gap_left == self.gap_right {
            self.grow();
        }
        
        self.buffer[pos as usize] = value;
        self.gap_left += 1
         
    }
}
