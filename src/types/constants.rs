pub const UNDERFLOW_ERROR: &str = "Stack underflow";
pub const INVALID_HEAP_POINTER_ERROR: &str = "Invalid heap pointer";

// Garbage Collection Configuration
pub const GC_CHECK_INTERVAL: usize = 12;
pub const GC_THRESHOLD: usize = 4000;
pub const GC_HISTORY_BUFFER_SIZE: usize = 10;

// Heap Scoring Weights (for GC heuristics)
pub const HEAP_SCORE_ARRAY_BASE: usize = 16;
pub const HEAP_SCORE_ARRAY_PER_ELEMENT: usize = 8;
pub const HEAP_SCORE_STRING_BASE: usize = 24;
pub const HEAP_SCORE_MAP_BASE: usize = 32;
pub const HEAP_SCORE_MAP_PER_ELEMENT: usize = 16;
pub const HEAP_SCORE_OTHER_OBJECT: usize = 32;

// String Processing
pub const MAX_STRING_LENGTH: usize = 1024;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Pipeline = 1,
    Comparison = 2,
    Term = 3,   // Addition/Subtraction
    Factor = 4, // Multiplication/Division
    Unary = 5,  // Unary operators and parentheses
}

impl Precedence {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
