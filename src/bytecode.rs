#[derive(Debug)]
pub enum ByteCode {
    GetGlobal(u8, u8),
    SetGlobal(u8, u16),
    LoadConst(u8, u16),
    Move(u8, u8),
    Call(u8, u8),
    Operate(u8),
}