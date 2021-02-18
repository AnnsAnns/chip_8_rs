pub enum ProgramCounter {
    Unknown,
    Next,
    Skip,
    Jump(u16)
}

impl ProgramCounter {
    pub fn skip_when(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    pub fn resolve(&self, pc: u16) -> u16 {
        match self {
            ProgramCounter::Next => pc + 2,
            ProgramCounter::Skip => pc + 4,
            ProgramCounter::Jump(line) => {
                *line as u16
            }
            ProgramCounter::Unknown => panic!("Something went wrong and it appears like the ProgramCounter was never changed!")
        }
    }
}