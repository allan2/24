use super::{BoundOp, Ops};
use crate::{abs_diff_eq, bound_op::Solutions};
use itertools::Itertools;
use std::collections::VecDeque;

pub const TARGET: f64 = 24.0;

/// A group of cards
#[derive(Clone)]
pub struct Cards {
    /// The numbers
    ///
    /// Array must not be empty
    cards: Vec<u8>,
    /// Permitted operations
    ops: Ops,
}

impl Cards {
    pub fn new(cards: Vec<u8>) -> Self {
        Self {
            cards,
            ops: Ops::default(),
        }
    }

    /// Specify a custom set of operations
    pub fn with_ops(cards: Vec<u8>, ops: Ops) -> Self {
        Self { cards, ops }
    }

    /// Finds all possible solutions
    pub fn solve(self) -> Solutions {
        let items = self
            .cards_ops()
            .into_iter()
            .flat_map(move |(cards_perm, ops)| {
                let q = vec![cards_perm.into_iter().map(BoundOp::Val).collect::<Vec<_>>()];
                let mut q = VecDeque::from(q);

                while let Some(bops) = q.pop_front() {
                    let new_bops_len = bops.len() - 1;
                    if new_bops_len == 0 {
                        break;
                    }

                    q.extend((0..new_bops_len).map(|i| {
                        let mut new_bops = bops.clone();

                        // Take two BoundOps from `bops` and combine them into one.
                        new_bops.splice(
                            i..i + 2,
                            Some(BoundOp::BoundOp {
                                l: Box::new(bops[i].clone()),
                                r: Box::new(bops[i + 1].clone()),
                                // going backwards here, but no big deal
                                op: ops.clone().into_inner()[new_bops_len - 1].clone(),
                            }),
                        );
                        new_bops
                    }));
                }
                q.into_iter().filter_map(move |bops| {
                    let bop = bops[0].clone();
                    if abs_diff_eq(bop.eval(), TARGET) {
                        return Some(bop);
                    }
                    None
                })
            })
            .dedup()
            .collect();
        Solutions::new(items)
    }

    /// Gets all possible operations for each permutation of the cards.
    ///
    /// Note: some operations are communitative. Duplicates are removed.
    fn cards_ops(self) -> Vec<(Vec<u8>, Ops)> {
        let cards = self.clone().permutations_dedup();

        let ops = self.op_product();
        CartesianProduct::with_sizes(&[cards.len(), ops.len()])
            .map(move |idxs| (cards[idxs[0]].clone(), ops[idxs[1]].clone()))
            .collect()
    }

    /// Gets the Cartesian product of the operations.
    ///
    /// This is unfiltered.
    fn op_product(self) -> Vec<Ops> {
        let mut op_prods = Vec::with_capacity(self.ops.len() * (self.cards.len() - 1));

        let cart_prod = CartesianProduct::with_sizes(&vec![self.ops.len(); self.cards.len() - 1]);

        for idxes in cart_prod {
            let ops = idxes
                .into_iter()
                .map(|i| self.ops.clone().into_inner()[i].clone())
                .collect::<Vec<_>>();
            op_prods.push(Ops::with_ops(ops));
        }
        op_prods
    }

    /// Returns all permutations of the cards, with duplicates removed.
    fn permutations_dedup(self) -> Vec<Vec<u8>> {
        self.cards
            .clone()
            .into_iter()
            .permutations(self.cards.len())
            .dedup()
            .collect()
    }
}

pub struct CartesianProduct {
    sizes: Vec<usize>,
    indices: Vec<usize>,
    done: bool,
}

impl CartesianProduct {
    pub fn with_sizes(sizes: &[usize]) -> Self {
        Self {
            sizes: sizes.to_vec(),
            indices: vec![0; sizes.len()],
            done: false,
        }
    }
}

impl Iterator for CartesianProduct {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.sizes.iter().product::<usize>() == 0 {
            return None;
        }
        let res = self.indices.clone();
        self.done = true;
        for (i, idx) in self.indices.iter_mut().enumerate().rev() {
            *idx += 1;
            if *idx >= self.sizes[i] {
                *idx = 0;
            } else {
                self.done = false;
                break;
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;
    use crate::Op;

    #[test]
    fn test_permutations_2() {
        let cards = Cards::new(vec![1, 2]);
        assert_eq!(cards.permutations_dedup(), vec![vec![1, 2], vec![2, 1]])
    }

    #[test]
    fn test_permutations_2_dup() {
        let cards = Cards::new(vec![1, 1]);
        assert_eq!(cards.permutations_dedup(), vec![vec![1, 1]])
    }

    #[test]
    fn test_permutations_3() {
        let cards = Cards::new(vec![1, 2, 3]);
        assert_eq!(
            cards.permutations_dedup(),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        )
    }

    #[test]
    fn test_cards_ops_2_dup() {
        let cards = Cards::new(vec![1, 1]);
        let card_ops = cards.cards_ops();
        assert_eq!(
            card_ops,
            vec![
                (vec![1, 1], Ops::with_ops(vec![Op::Add])),
                (vec![1, 1], Ops::with_ops(vec![Op::Sub])),
                (vec![1, 1], Ops::with_ops(vec![Op::Mul])),
                (vec![1, 1], Ops::with_ops(vec![Op::Div])),
                (vec![1, 1], Ops::with_ops(vec![Op::Pow])),
            ]
        )
    }

    #[test]
    fn test_cards_ops() {
        let cards = Cards::new(vec![1, 2]);
        let card_ops = cards.cards_ops();
        assert_eq!(
            card_ops,
            vec![
                (vec![1, 2], Ops::with_ops(vec![Op::Add])),
                (vec![1, 2], Ops::with_ops(vec![Op::Sub])),
                (vec![1, 2], Ops::with_ops(vec![Op::Mul])),
                (vec![1, 2], Ops::with_ops(vec![Op::Div])),
                (vec![1, 2], Ops::with_ops(vec![Op::Pow])),
                (vec![2, 1], Ops::with_ops(vec![Op::Add])),
                (vec![2, 1], Ops::with_ops(vec![Op::Sub])),
                (vec![2, 1], Ops::with_ops(vec![Op::Mul])),
                (vec![2, 1], Ops::with_ops(vec![Op::Div])),
                (vec![2, 1], Ops::with_ops(vec![Op::Pow])),
            ]
        )
    }

    #[test]
    fn test_solve_no_sol() {
        let cards = Cards::new(vec![1, 1, 1, 1]);
        assert_eq!(cards.solve(), Solutions::new(Vec::new()));
    }

    #[test]
    fn test_solve_3s() {
        let cards = Cards::new(vec![3, 3, 3, 3]);

        let unique_sol = BoundOp::BoundOp {
            op: Op::Sub,
            l: Box::new(BoundOp::BoundOp {
                op: Op::Mul,
                l: Box::new(BoundOp::Val(3)),
                r: Box::new(BoundOp::BoundOp {
                    op: Op::Mul,
                    l: Box::new(BoundOp::Val(3)),
                    r: Box::new(BoundOp::Val(3)),
                }),
            }),
            r: Box::new(BoundOp::Val(3)),
        };
        assert_eq!(cards.solve(), Solutions::new(vec![unique_sol]));
    }

    #[test]
    fn test_solve_difficult() {
        let cards = Cards::new(vec![1, 4, 7, 9]);

        let sols = vec![
            BoundOp::BoundOp {
                op: Op::Mul,
                l: Box::new(BoundOp::BoundOp {
                    op: Op::Sub,
                    l: Box::new(BoundOp::Val(1)),
                    r: Box::new(BoundOp::Val(9)),
                }),
                r: Box::new(BoundOp::BoundOp {
                    op: Op::Sub,
                    l: Box::new(BoundOp::Val(4)),
                    r: Box::new(BoundOp::Val(7)),
                }),
            },
            BoundOp::BoundOp {
                op: Op::Mul,
                l: Box::new(BoundOp::Val(4)),
                r: Box::new(BoundOp::BoundOp {
                    op: Op::Sub,
                    l: Box::new(BoundOp::Val(7)),
                    r: Box::new(BoundOp::BoundOp {
                        op: Op::Pow,
                        l: Box::new(BoundOp::Val(1)),
                        r: Box::new(BoundOp::Val(9)),
                    }),
                }),
            },
            // negative version of solution 1
            BoundOp::BoundOp {
                op: Op::Mul,
                l: Box::new(BoundOp::BoundOp {
                    op: Op::Sub,
                    l: Box::new(BoundOp::Val(7)),
                    r: Box::new(BoundOp::Val(4)),
                }),
                r: Box::new(BoundOp::BoundOp {
                    op: Op::Sub,
                    l: Box::new(BoundOp::Val(9)),
                    r: Box::new(BoundOp::Val(1)),
                }),
            },
        ];
        assert_eq!(cards.solve(), Solutions::new(sols));
    }

    #[test]
    fn test_permutations_3_dup() {
        let cards = Cards::new(vec![1, 1, 1]);
        assert_eq!(cards.permutations_dedup(), vec![vec![1, 1, 1]])
    }

    #[test]
    fn test_op_product_2() {
        let cards = Cards::new(vec![1, 2]);
        assert_eq!(
            cards.op_product(),
            vec![
                Ops::with_ops(vec![Op::Add]),
                Ops::with_ops(vec![Op::Sub]),
                Ops::with_ops(vec![Op::Mul]),
                Ops::with_ops(vec![Op::Div]),
                Ops::with_ops(vec![Op::Pow]),
            ]
        )
    }

    #[test]
    fn test_op_product_3() {
        let cards = Cards::new(vec![1, 2, 3]);
        assert_eq!(
            cards.op_product(),
            vec![
                Ops::with_ops(vec![Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Pow]),
            ]
        )
    }

    #[test]
    fn test_op_product_4() {
        let cards = Cards::new(vec![1, 2, 3, 4]);
        assert_eq!(
            cards.op_product(),
            vec![
                Ops::with_ops(vec![Op::Add, Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Add, Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Add, Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Add, Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Add, Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Add, Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Add, Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Add, Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Add, Op::Pow, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Sub, Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Sub, Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Sub, Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Sub, Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Sub, Op::Pow, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Mul, Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Mul, Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Mul, Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Mul, Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Mul, Op::Pow, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Div, Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Div, Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Div, Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Div, Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Div, Op::Pow, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Add, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Add, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Add, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Add, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Add, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Sub, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Sub, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Sub, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Sub, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Sub, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Mul, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Mul, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Mul, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Mul, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Mul, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Div, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Div, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Div, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Div, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Div, Op::Pow]),
                Ops::with_ops(vec![Op::Pow, Op::Pow, Op::Add]),
                Ops::with_ops(vec![Op::Pow, Op::Pow, Op::Sub]),
                Ops::with_ops(vec![Op::Pow, Op::Pow, Op::Mul]),
                Ops::with_ops(vec![Op::Pow, Op::Pow, Op::Div]),
                Ops::with_ops(vec![Op::Pow, Op::Pow, Op::Pow]),
            ]
        )
    }
}
