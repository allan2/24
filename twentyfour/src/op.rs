use std::{fmt, vec};

/// The type of arithmetic operation
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Op {
    pub fn f(&self) -> fn(f64, f64) -> f64 {
        match self {
            Op::Add => |a, b| a + b,
            Op::Sub => |a, b| a - b,
            Op::Mul => |a, b| a * b,
            Op::Div => |a, b| a / b,
            Op::Pow => |a, b| a.powf(b),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "×",
            Op::Div => "÷",
            Op::Pow => "^",
        };
        write!(f, "{s}")
    }
}

/// A newtype collection of operations
#[derive(Debug, Clone, PartialEq)]
pub struct Ops(Vec<Op>);

impl Ops {
    pub fn with_capacity(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }

    pub fn with_ops(ops: Vec<Op>) -> Self {
        Self(ops.to_vec())
    }

    pub fn into_inner(self) -> Vec<Op> {
        self.0
    }

    pub fn push(&mut self, op: Op) {
        self.0.push(op)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Iterator for Ops {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Default for Ops {
    fn default() -> Self {
        Self(vec![Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Pow])
    }
}
