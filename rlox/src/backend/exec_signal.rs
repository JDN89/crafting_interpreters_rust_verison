use crate::backend::value::LoxValue;


pub enum ExecSignal {
    Normal,
    Return(LoxValue)
}
