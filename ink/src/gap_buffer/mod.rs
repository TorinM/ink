// Module to control the gap buffer

// TODO: Replace the char usage with u8s byte representation

#[derive(Debug)]
pub struct GapBuffer {
    cursor_pos: usize,
    gap_left_ptr: usize,
    gap_right_ptr: usize,
    data: Vec<char>,
    gap_size: usize,
}
impl GapBuffer {
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {
            cursor_pos: 0,
            gap_left_ptr: 0,
            gap_right_ptr: size - 1,
            data: vec!['_'; size.try_into().unwrap()],
            gap_size: 10,
        }
    }

    pub fn move_cursor(&mut self, new_pos: usize) {
        self.cursor_pos = new_pos;
    }

    pub fn move_buffer(&mut self) {
        if self.cursor_pos < self.gap_left_ptr {
            self.shift_left()
        } else {
            self.shift_right()
        }
    }

    pub fn insert_data(&mut self, val: char) {
        if self.cursor_pos != self.gap_left_ptr {
            self.move_buffer()
        }

        if self.gap_left_ptr == self.gap_right_ptr {
            self.grow()
        }

        self.data[self.cursor_pos] = val;

        self.cursor_pos += 1;
        self.gap_left_ptr += 1;
    }

    fn grow(&mut self) {
        let new_buffer = vec!['_'; self.gap_size.try_into().unwrap()];
    
        let index = self.cursor_pos;

        self.data.splice(index..index, new_buffer.iter().cloned());

        self.gap_right_ptr = self.data.len() - 1;
        self.gap_size = self.data.len();
    }

    fn shift_left(&mut self) {
        while self.cursor_pos < self.gap_left_ptr {
            self.gap_left_ptr -= 1;

            self.data.swap(self.gap_right_ptr, self.gap_left_ptr);

            self.gap_right_ptr -= 1;
        }
    }

    fn shift_right(&mut self) {
        while self.cursor_pos > self.gap_left_ptr && self.gap_right_ptr < self.gap_size - 1 {
            self.gap_right_ptr += 1;

            self.data.swap(self.gap_left_ptr, self.gap_right_ptr);

            self.gap_left_ptr += 1;
       }
    }
}
