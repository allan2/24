use serde::Serialize;
use twentyfour::{BoundOp, Cards};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Solution {
    solution: String,
    explanation: Vec<String>,
}

impl From<BoundOp> for Solution {
    fn from(bop: BoundOp) -> Self {
        Solution {
            solution: fmt_solution(bop.to_string()),
            // Remove leading and trailing parantheses
            explanation: bop
                .explain()
                .1
                .into_iter()
                .map(fmt_explanation_step)
                .collect(),
        }
    }
}

/// Outputs the solutions as a serialized string.
#[wasm_bindgen]
pub fn solve24(a: u8, b: u8, c: u8, d: u8) -> String {
    let cards = Cards::new(vec![a, b, c, d]);
    let strs = cards
        .solve()
        .into_iter()
        .map(Solution::from)
        .collect::<Vec<_>>();
    serde_json::to_string(&strs).unwrap()
}

/// Removes parantheses around the BoundOp.
///
/// Adds spaces around all operators except exponentiation.
/// Exponentiation is transformed to a superscript using JavaScript.
fn fmt_explanation_step<S>(s: S) -> String
where
    S: Into<String>,
{
    let mut s = s.into();
    s = s.replacen('(', "", 1);
    s = s.replacen(')', "", 1);

    for c in ['+', '-', '×', '÷'] {
        // only replace first match and then return
        if s.contains(c) {
            s = s.replacen(c, &format!(" {c} "), 1);
            return s;
        }
    }

    if s.contains('^') {
        // Replace 2^3 = 8 with 2<sup>3/sup> = 8

        let (l, r) = s.split_once(" =").unwrap();
        let (base, exp) = l.split_once('^').unwrap();
        s = format!("{base}<sup>{exp}</sup> ={r}");
    }
    s
}

/// Some formatting for the solution, but more can be done
fn fmt_solution<S>(s: S) -> String
where
    S: Into<String>,
{
    let s = s.into();

    // Remove opening and trailing parantheses
    let mut chars = s.chars();
    chars.next();
    chars.next_back();

    let mut s = chars.collect::<String>();

    for c in ['+', '-', '×', '÷'] {
        s = s.replace(c, &format!(" {c} "));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_fmt_explanation_step() {
        assert_eq!(fmt_explanation_step("(2+3) = 5"), "2 + 3 = 5".to_owned())
    }

    #[test]
    pub fn test_fmt_explanation_step_mul() {
        assert_eq!(fmt_explanation_step("(1×2) = 2"), "1 × 2 = 2".to_owned())
    }

    #[test]
    pub fn test_fmt_explanation_step_div() {
        assert_eq!(fmt_explanation_step("(2÷1) = 1"), "2 ÷ 1 = 1".to_owned())
    }

    #[test]
    pub fn test_fmt_explanation_step_pow() {
        assert_eq!(
            fmt_explanation_step("(2^3) = 8"),
            "2<sup>3</sup> = 8".to_owned()
        )
    }

    #[test]
    pub fn test_fmt_solution_1() {
        assert_eq!(
            fmt_solution("((1+(2+3))×4)"),
            "(1 + (2 + 3)) × 4".to_owned()
        )
    }

    #[test]
    pub fn test_fmt_solution_2() {
        assert_eq!(
            fmt_solution("((1×2)×(3×4))"),
            "(1 × 2) × (3 × 4)".to_owned()
        )
    }
}
