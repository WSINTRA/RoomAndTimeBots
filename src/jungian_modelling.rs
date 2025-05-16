// Define the four psychological functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    Thinking,
    Feeling,
    Sensation,
    Intuition,
}

// Define the two attitudes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attitude {
    Introverted,
    Extraverted,
}

// A function with its type and attitude
#[derive(Debug, Clone, Copy)]
pub struct PsychologicalFunction {
    pub function: FunctionType,
    pub attitude: Attitude,
    pub weight: u8, // 0-100% strength of this function
}

// Personality with dominant, auxiliary, and inferior functions
#[derive(Debug, Clone)]
pub struct Personality {
    pub dominant: PsychologicalFunction,
    pub auxiliary: PsychologicalFunction,
    pub inferior: PsychologicalFunction,
}
