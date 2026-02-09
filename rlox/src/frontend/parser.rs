use crate::frontend::token::Token;

// TOOD: again. get rid of the lifetime. We are draggin &str subslices around beter to something like struct Span { start: u32, len: u32 } and then slice in to the source code if I actually need the source code which I don't think i do at the moment... Rip it out?
struct Parser {
    tokens: Vec<Token>,
    current: i32,
}
