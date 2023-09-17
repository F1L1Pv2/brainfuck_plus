pub const MEM_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub enum TokenType{
    Ident, // names 
    MacroDecl, // #define
    PointerLeft, // <
    PointerRight, // >
    PointerReset, // &
    Add, // +
    Sub, // -
    ReadByte, // ,
    WriteByte, // .
    Clear, // '
    BaseMemAddr, // %
    MemAddr, // $
    ZeroJump, // [
    NonZeroJump, // ]
    Syscall, // ?
    NewLine, // \n
}

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub value: String
}

#[derive(PartialEq, Debug)]
pub struct Condition {
    pub addr: usize,
}

#[derive(PartialEq, Debug)]
pub struct Forward {
    pub back_addr: usize,
}

#[derive(PartialEq, Debug)]
pub enum Jumps {
    Condition(Condition),
    Forward(Forward),
}