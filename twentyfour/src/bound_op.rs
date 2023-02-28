use super::Op;
use std::{fmt, vec};

/// A recursive data structure representing a bound operation.
#[derive(Clone, Debug)]
pub enum BoundOp {
    Val(u8),
    BoundOp {
        op: Op,
        l: Box<BoundOp>,
        r: Box<BoundOp>,
    },
}

/// Communitative operations are considered equal.
/// This is not a perfect use of PartialEq, but it works for this use case.
impl PartialEq for BoundOp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BoundOp::Val(a), BoundOp::Val(b)) => a == b,
            (
                BoundOp::BoundOp {
                    op: aop,
                    l: al,
                    r: ar,
                },
                BoundOp::BoundOp {
                    op: bop,
                    l: bl,
                    r: br,
                },
            ) => {
                match (aop, bop) {
                    (Op::Add, Op::Add) | (Op::Mul, Op::Mul) => {
                        return (al == bl && ar == br) || (al == br && ar == bl)
                    }
                    _ => {}
                }
                aop == bop && al == bl && ar == br
            }
            _ => false,
        }
    }
}

/// All solutions to a set of cards
#[cfg_attr(wasm, wasm_bindgen)]
#[derive(Debug, PartialEq)]
pub struct Solutions(pub Vec<BoundOp>);

impl Solutions {
    pub fn new(solutions: Vec<BoundOp>) -> Self {
        Self(solutions)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Solutions {
    type Item = BoundOp;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl BoundOp {
    pub fn eval(&self) -> f64 {
        match self {
            BoundOp::Val(n) => *n as f64,
            BoundOp::BoundOp { op, l, r } => (op.f())(l.eval(), r.eval()),
        }
    }

    pub fn to_infix_notation(&self) -> String {
        match self {
            BoundOp::Val(n) => n.to_string(),
            BoundOp::BoundOp { op, l, r } => {
                format!("({}{}{})", l.to_infix_notation(), op, r.to_infix_notation(),)
            }
        }
    }

    pub fn explain(&self) -> (f64, Vec<String>) {
        let mut explanation = vec![];
        match self {
            BoundOp::Val(val) => (*val as f64, explanation),
            BoundOp::BoundOp { op, l, r } => {
                let (lv, le) = l.explain();
                explanation.extend(le);
                let (rv, re) = r.explain();
                explanation.extend(re);

                let flat_bop = BoundOp::BoundOp {
                    op: op.clone(),
                    l: Box::new(BoundOp::Val(lv as u8)),
                    r: Box::new(BoundOp::Val(rv as u8)),
                };
                let val = flat_bop.eval();
                explanation.push(format!("{} = {}", flat_bop, val));
                (val, explanation)
            }
        }
    }
}

impl fmt::Display for BoundOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}
