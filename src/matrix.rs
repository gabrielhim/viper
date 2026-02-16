use std::cmp;
use std::{collections::HashMap, i32};

use crate::alignment::AlignmentMode;
use crate::constants::{GAP_EXTENSION_PENALTY, GAP_PENALTY, MATCH_SCORE, MISMATCH_PENALTY};

pub struct Matrix {
    pub primary: HashMap<(usize, usize), i32>,
    pub aux_ins: HashMap<(usize, usize), i32>,
    pub aux_del: HashMap<(usize, usize), i32>,
    pub start_key: (usize, usize),
    pub alignment_mode: AlignmentMode,
}

impl Matrix {
    pub fn create(seq1: &Vec<char>, seq2: &Vec<char>, mode: AlignmentMode) -> Self {
        match mode {
            AlignmentMode::Global => Self::create_global_matrix(seq1, seq2),
            AlignmentMode::Local => Self::create_local_matrix(seq1, seq2),
        }
    }

    fn create_global_matrix(seq1: &Vec<char>, seq2: &Vec<char>) -> Self {
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
        let mut start_key = (0, 0);

        for i in 1..=seq1.len() {
            for j in 1..=seq2.len() {
                let ins_gap = matrix.get(&(i, j - 1)).unwrap() - GAP_PENALTY;
                let ins_ext = insertions.get(&(i, j - 1)).unwrap() - GAP_EXTENSION_PENALTY;
                let max_ins_core = cmp::max(ins_gap, ins_ext);
                insertions.insert((i, j), max_ins_core);

                let del_gap = matrix.get(&(i - 1, j)).unwrap() - GAP_PENALTY;
                let del_ext = deletions.get(&(i - 1, j)).unwrap() - GAP_EXTENSION_PENALTY;
                let max_del_score = cmp::max(del_gap, del_ext);
                deletions.insert((i, j), max_del_score);

                let match_or_mismatch = if seq1[i - 1] == seq2[j - 1] {
                    matrix.get(&(i - 1, j - 1)).unwrap() + MATCH_SCORE
                } else {
                    matrix.get(&(i - 1, j - 1)).unwrap() - MISMATCH_PENALTY
                };

                let score = *[max_ins_core, max_del_score, match_or_mismatch]
                    .iter()
                    .max()
                    .unwrap();

                matrix.insert((i, j), score);

                if i == seq1.len() && j == seq2.len() {
                    start_key = (i, j);
                }
            }
        }

        Self {
            primary: matrix,
            aux_ins: insertions,
            aux_del: deletions,
            start_key: start_key,
            alignment_mode: AlignmentMode::Global,
        }
    }

    fn create_local_matrix(seq1: &Vec<char>, seq2: &Vec<char>) -> Self {
        let mut matrix: HashMap<(usize, usize), i32> = HashMap::new();
        let mut insertions: HashMap<(usize, usize), i32> = HashMap::new();
        let mut deletions: HashMap<(usize, usize), i32> = HashMap::new();

        // Initialize matrices
        matrix.insert((0, 0), 0);

        for i in 1..=seq1.len() {
            matrix.insert((i, 0), 0);
            insertions.insert((i, 0), 0);
        }
        for j in 1..=seq2.len() {
            matrix.insert((0, j), 0);
            deletions.insert((0, j), 0);
        }

        // Calculate scores
        let mut max_key = (0, 0);
        let mut max_value = 0;

        for i in 1..=seq1.len() {
            for j in 1..=seq2.len() {
                let ins_gap = matrix.get(&(i, j - 1)).unwrap() - GAP_PENALTY;
                let ins_ext = insertions.get(&(i, j - 1)).unwrap() - GAP_EXTENSION_PENALTY;
                let max_ins_score = *[0, ins_gap, ins_ext].iter().max().unwrap();
                insertions.insert((i, j), max_ins_score);

                let del_gap = matrix.get(&(i - 1, j)).unwrap() - GAP_PENALTY;
                let del_ext = deletions.get(&(i - 1, j)).unwrap() - GAP_EXTENSION_PENALTY;
                let max_del_score = *[0, del_gap, del_ext].iter().max().unwrap();
                deletions.insert((i, j), max_del_score);

                let match_or_mismatch = if seq1[i - 1] == seq2[j - 1] {
                    matrix.get(&(i - 1, j - 1)).unwrap() + MATCH_SCORE
                } else {
                    matrix.get(&(i - 1, j - 1)).unwrap() - MISMATCH_PENALTY
                };

                let score = *[0, max_ins_score, max_del_score, match_or_mismatch]
                    .iter()
                    .max()
                    .unwrap();

                matrix.insert((i, j), score);

                if score > max_value {
                    max_value = score;
                    max_key = (i, j);
                }
            }
        }

        Self {
            primary: matrix,
            aux_ins: insertions,
            aux_del: deletions,
            start_key: max_key,
            alignment_mode: AlignmentMode::Local,
        }
    }
}
