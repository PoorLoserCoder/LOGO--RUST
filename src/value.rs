use std::fmt;
use crate::vm::ExeState;
use crate::coordinate;
use plotters::prelude::DrawingArea;
use plotters::prelude::BitMapBackend;
use plotters::coord::Shift;

// ANCHOR: value
#[derive(Clone)]
pub enum Value {
    Nil,
    Integer(i64),
    String(String),
    Function(fn (&mut ExeState, &mut coordinate, &mut DrawingArea<BitMapBackend, Shift>) -> i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}
// ANCHOR_END: value
impl Value {
    fn add(self, other: Value) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Integer(i1 + i2)),
            _ => Err("Both values must be integers"),
        }
    }

    fn sub(self, other: Value) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Integer(i1 - i2)),
            _ => Err("Both values must be integers"),
        }
    }

    fn mul(self, other: Value) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Integer(i1 * i2)),
            _ => Err("Both values must be integers"),
        }
    }

    fn div(self, other: Value) -> Result<Value, &'static str> {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => {
                if i2 != 0 {
                    Ok(Value::Integer(i1 / i2))
                } else {
                    Err("Cannot divide by zero")
                }
            }
            _ => Err("Both values must be integers"),
        }
    }
}

// ANCHOR: peq
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        // TODO compare Integer vs Float
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
            (Value::String(s1), Value::String(s2)) => *s1 == *s2,
            (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
            (_, _) => false,
        }
    }
}
// ANCHOR_END: peq