const MEMORY_SIZE: usize = 10000;
pub type MemoryPointer = usize;

#[derive(Debug)]
pub struct Machine {
    data: [u8; MEMORY_SIZE],
    pointer: MemoryPointer,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            data: [0; MEMORY_SIZE],
            pointer: MEMORY_SIZE / 2,
        }
    }

    pub fn move_right(&mut self) -> Result<(), MemoryError> {
        if self.pointer < MEMORY_SIZE - 1 {
            self.pointer += 1;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    pub fn move_left(&mut self) -> Result<(), MemoryError> {
        if self.pointer > 0 {
            self.pointer -= 1;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    pub fn increment(&mut self) -> Result<(), MemoryError> {
        if self.pointer < MEMORY_SIZE {
            self.data[self.pointer] += 1;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    pub fn decrement(&mut self) -> Result<(), MemoryError> {
        if self.pointer < MEMORY_SIZE {
            self.data[self.pointer] -= 1;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    pub fn read(&self) -> Result<u8, MemoryError> {
        if self.pointer < MEMORY_SIZE {
            Ok(self.data[self.pointer])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }

    pub fn write(&mut self, value: u8) -> Result<(), MemoryError> {
        if self.pointer < MEMORY_SIZE {
            self.data[self.pointer] = value;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
