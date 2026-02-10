use std::cmp;
use std::{collections::HashMap, i32};

use crate::constants::{GAP_EXTENSION_PENALTY, GAP_PENALTY, MATCH_SCORE, MISMATCH_PENALTY};

pub struct Matrix {
    pub primary: HashMap<(usize, usize), i32>,
    pub aux_ins: HashMap<(usize, usize), i32>,
    pub aux_del: HashMap<(usize, usize), i32>,
}

impl Matrix {
    pub fn create(seq1: &Vec<char>, seq2: &Vec<char>) -> Self {
        let mut matrix: HashMap<(usize, usize), i32> = HashMap::new();
        let mut insertions: HashMap<(usize, usize), i32> = HashMap::new();
        let mut deletions: HashMap<(usize, usize), i32> = HashMap::new();

        // Initialize matrices
        matrix.insert((0, 0), 0);
        for i in 0..=seq1.len() {
            insertions.insert((i, 0), -i32::MAX);
        }
        for j in 0..=seq2.len() {
            deletions.insert((0, j), -i32::MAX);
        }

        for i in 1..=seq1.len() {
            let penalty = -(GAP_PENALTY + (i as i32 - 1) * GAP_EXTENSION_PENALTY);
            deletions.insert((i, 0), penalty);
            matrix.insert((i, 0), *deletions.get(&(i, 0)).unwrap());
        }
        for j in 1..=seq2.len() {
            let penalty = -(GAP_PENALTY + (j as i32 - 1) * GAP_EXTENSION_PENALTY);
            insertions.insert((0, j), penalty);
            matrix.insert((0, j), *insertions.get(&(0, j)).unwrap());
        }

        // Calculate scores
        for i in 1..=seq1.len() {
            for j in 1..=seq2.len() {
                let ins_gap = matrix.get(&(i, j - 1)).unwrap() - GAP_PENALTY;
                let ins_ext = insertions.get(&(i, j - 1)).unwrap() - GAP_EXTENSION_PENALTY;
                insertions.insert((i, j), cmp::max(ins_gap, ins_ext));

                let del_gap = matrix.get(&(i - 1, j)).unwrap() - GAP_PENALTY;
                let del_ext = deletions.get(&(i - 1, j)).unwrap() - GAP_EXTENSION_PENALTY;
                deletions.insert((i, j), cmp::max(del_gap, del_ext));

                let ins_or_del = *cmp::max(
                    insertions.get(&(i, j)).unwrap(),
                    deletions.get(&(i, j)).unwrap(),
                );

                let match_or_mismatch = if seq1[i - 1] == seq2[j - 1] {
                    matrix.get(&(i - 1, j - 1)).unwrap() + MATCH_SCORE
                } else {
                    matrix.get(&(i - 1, j - 1)).unwrap() - MISMATCH_PENALTY
                };

                matrix.insert((i, j), cmp::max(ins_or_del, match_or_mismatch));
            }
        }

        Self {
            primary: matrix,
            aux_ins: insertions,
            aux_del: deletions,
        }
    }
}
