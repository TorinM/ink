// Module to control the gap buffer

use std::fmt;

#[derive(Debug)]
pub struct GapBuffer {
    cursor_pos: usize,
    gap_left_ptr: usize,
    gap_right_ptr: usize,
    data: Vec<u8>,
    gap_size: usize,
}
impl GapBuffer {
    pub fn new(size: usize) -> GapBuffer {
        GapBuffer {
            cursor_pos: 0,
            gap_left_ptr: 0,
            gap_right_ptr: size - 1,
            data: vec![0u8; size.try_into().unwrap()],
            gap_size: 10,
        }
    }

    pub fn get_diagnostics(&self) -> String {
        let s = format!("Cursor Position: {:?} | Gap Left Pointer: {:?} | Gap Right Pointer: {:?}", self.cursor_pos, self.gap_left_ptr, self.gap_right_ptr);
        s
    }

    #[allow(dead_code)] 
    pub fn move_cursor_by(&mut self, shift_val: isize) {
        if self.cursor_pos > 0 && self.cursor_pos < self.data.len()-1 {
            if shift_val < 0 {
                self.cursor_pos -= shift_val.wrapping_abs() as usize;
            }
            else {
                self.cursor_pos += shift_val as usize;
            }
            self.move_buffer();
        }
    }

    #[allow(dead_code)] 
    pub fn move_cursor_to(&mut self, new_pos: usize) {
        self.cursor_pos = new_pos;
        self.move_buffer();
    }

    pub fn move_buffer(&mut self) {
        if self.cursor_pos < self.gap_left_ptr {
            self.shift_left()
        } else {
            self.shift_right()
        }
    }
    
    //backspace handler
    pub fn delete_data(&mut self) {
         if self.cursor_pos != self.gap_left_ptr {
            self.move_buffer()
        }
        self.cursor_pos -= 1;
        self.gap_left_ptr -= 1;
        self.data[self.cursor_pos] = 0u8;
    }

    pub fn insert_data(&mut self, val: char) {
        if self.cursor_pos != self.gap_left_ptr {
            self.move_buffer()
        }

        if self.gap_left_ptr == self.gap_right_ptr {
            self.grow()
        }

        self.data[self.cursor_pos] = val as u8;
        self.cursor_pos += 1;
        self.gap_left_ptr += 1;
    }

    fn grow(&mut self) {
        let new_buffer = vec![0u8; self.gap_size.try_into().unwrap()];
    
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
impl fmt::Display for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();

        for d in self.data.clone() {
            if d == 0 {
                //s.push('_')
            }
            else {
                s.push(char::from(d))
            }
        }
    
        write!(f, "{}", s)
    }
}

