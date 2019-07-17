pub struct State {
    pos: i32,            // Current position of the head on the tape
    pos_tape: Vec<bool>, // Positions 0, 1, ...
    neg_tape: Vec<bool>, // Positions -1, -2, ...
    input: Vec<u8>,      // Input stream
    input_bit: usize,    // Next bit in the input stream to read
    output: Vec<u8>,     // Output stream
    output_bit: usize,   // Next index in the output stream to write to
    code: Vec<char>,     // The code string
    code_index: usize,   // The program counter/index into the code string
}
impl State {
    pub fn new(code: Vec<char>, input: Vec<u8>) -> State {
        State {
            pos: 0,
            pos_tape: Vec::new(),
            neg_tape: Vec::new(),
            input,
            input_bit: 0,
            output: vec![0],
            output_bit: 0,
            code,
            code_index: 0,
        }
    }

    fn set_bit(&mut self, bit: bool) {
        let r;
        if self.pos >= 0 {
            r = State::get_or_extend_mut(&mut self.pos_tape, self.pos as usize)
        } else {
            r = State::get_or_extend_mut(&mut self.neg_tape, -self.pos as usize - 1)
        }
        *r = bit;
    }
    fn get_or_extend_mut(vec: &mut Vec<bool>, index: usize) -> &mut bool {
        if index >= vec.len() {
            vec.resize(index + 1, false);
        }
        vec.get_mut(index).expect("Failed to resize vector?")
    }
    fn get_bit(&self) -> bool {
        let cell;
        if self.pos >= 0 {
            cell = self.pos_tape.get(self.pos as usize)
        } else {
            cell = self.neg_tape.get(-self.pos as usize - 1)
        }
        cell.copied().unwrap_or(false)
    }

    fn get_input_bit(&mut self) -> Result<bool, String> {
        // Read bits in little-endian order
        match self.input.get(self.input_bit / 8) {
            Some(word) => {
                let bit_value = word & (1u8 << self.input_bit % 8);
                self.input_bit += 1; // Advance in the input stream
                Ok(bit_value != 0)
            }
            None => Err(format!("Index out of bound in input stream: {}", self.input_bit))
        }
    }
    fn push_output_bit(&mut self, bit: bool) {
        // Only need to adjust the value if we're writing a 1
        if bit {
            if self.output_bit / 8 + 1 > self.output.len() {
                self.output.push(0);
            }
            let r = self.output.get_mut(self.output_bit / 8).expect("Failed to push enough output u8s");
            *r |= 1 << (self.output_bit % 8);
        }
        self.output_bit += 1;
    }
    fn jump_to_matching_bracket(&mut self, init_char: char) -> Result<(), String> {
        let match_char: char;
        let direction: i32;
        let position_adjust: usize;
        if init_char == '[' {
            // Look for ] to the right
            match_char = ']';
            direction = 1;
            position_adjust = 1; // Jump to one past the [, in accordance with the spec
        } else if init_char == ']' {
            // Look for [ to the left
            match_char = '[';
            direction = -1;
            position_adjust = 0; // Jump to exactly on the ]
        } else {
            return Err(format!("Character passed is neither '[' nor ']': {}", init_char))
        }

        let mut code_index = self.code_index;
        // Count the number of brackets of the same type as init_char that we need to see before being able to accept
        // a match_char as being the *closing* bracket
        let mut mismatch_count: u32 = 0;
        loop {
            if code_index == 0 {
                return Err(format!("Reached start of code while looking for {}", match_char))
            } else if code_index + 1 == self.code.len() {
                return Err(format!("Reached end of code while looking for {}", match_char))
            }
            // Checks above ensure we don't over/underflow
            code_index = (code_index as i32 + direction) as usize;

            if self.code[code_index] == init_char {
                // Mismatching bracket, we need to see one more opposite bracket
                mismatch_count += 1;
            } else if self.code[code_index] == match_char {
                // Matching bracket, but is it the one for us?
                if mismatch_count > 0 {
                    mismatch_count -= 1;
                } else {
                    break; // Found the matching bracket
                }
            }
        }
        self.code_index = code_index + position_adjust;
        return Ok(());
    }

    fn step(&mut self) -> Result<bool, String> {
        // Return true if we need to keep stepping before terminating, false if we're done
        match self.code.get(self.code_index) {
            None => Ok(false),
            Some(&command) => {
                let result = match command {
                    '+' => Ok(self.set_bit(!self.get_bit())), // Flip the bit under the cursor
                    ',' => self.get_input_bit().map(|b| self.set_bit(b)), // Set the cursor bit from input
                    ';' => Ok(self.push_output_bit(self.get_bit())), // Output the bit under the cursor
                    '<' => Ok(self.pos -= 1), // Move the pointer one bit to the left
                    '>' => Ok(self.pos += 1), // Move the pointer one bit to the right
                    '[' if !self.get_bit() => self.jump_to_matching_bracket('['),
                    ']' if self.get_bit() => self.jump_to_matching_bracket(']'),
                    _ => Ok(()),
                };
                result.and_then(|_| {
                // If we did a jump operation, don't move to the next instruction, we're already at it
                if command != '[' && command != ']' {
                    self.code_index += 1;
                }
                Ok(true)
                })
            }
        }
    }
    pub fn run(&mut self) -> Result<Vec<u8>, String> {
        loop {
            match self.step() {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => return Err(e)
            }
        }
        return Ok(self.output.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_setting() {
        // Check we can set bits and move around on the tape
        let mut state = State::new(Vec::new(), Vec::new());
        state.set_bit(false);
        assert!(!state.get_bit());
        state.set_bit(true);
        assert!(state.get_bit());

        state.pos = 17;

        state.set_bit(true);
        assert!(state.get_bit());
        state.set_bit(false);
        assert!(!state.get_bit());

        state.pos = 0;
        assert!(state.get_bit());
    }
    #[test]
    fn test_negative_bit_setting() {
        // Check we can set bits at negative positions and move around on the tape
        let mut state = State::new(Vec::new(), Vec::new());
        state.pos = 1;
        state.set_bit(true);
        state.pos = -1;
        state.set_bit(true);
        state.pos = 1;
        assert!(state.get_bit());
        state.pos = -1;
        assert!(state.get_bit());
    }
    #[test]
    fn test_get_input_bit() {
        // Should read little-endian order
        let mut state = State::new(Vec::new(), vec![0b10100011]);
        assert_eq!(state.get_input_bit(), Ok(true));
        assert_eq!(state.get_input_bit(), Ok(true));
        assert_eq!(state.get_input_bit(), Ok(false));
        assert_eq!(state.get_input_bit(), Ok(false));
        assert_eq!(state.get_input_bit(), Ok(false));
        assert_eq!(state.get_input_bit(), Ok(true));
        assert_eq!(state.get_input_bit(), Ok(false));
        assert_eq!(state.get_input_bit(), Ok(true));
    }
}