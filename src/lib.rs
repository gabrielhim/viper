mod constants;
mod matrix;

use constants::{GAP_EXTENSION_PENALTY, GAP_PENALTY};
use itertools::Itertools;
use matrix::{AlignmentMode, Matrix};

enum Pointer {
    PRIMARY,
    INS,
    DEL,
}

pub struct Alignment {
    pub aligned_seq1: Vec<char>,
    pub aligned_seq2: Vec<char>,
}

impl Alignment {
    pub fn print_alignment(&self) {
        let mut bridges: Vec<char> = Vec::new();
        for idx in 0..self.aligned_seq1.len() {
            if self.aligned_seq1[idx] == self.aligned_seq2[idx] {
                bridges.push('|');
            } else {
                bridges.push(' ');
            }
        }
        println!("{}", self.aligned_seq1.iter().join(""));
        println!("{}", bridges.iter().join(""));
        println!("{}", self.aligned_seq2.iter().join(""));
    }
}

fn retrieve_alignment(seq1: &Vec<char>, seq2: &Vec<char>, matrix: &Matrix) -> Alignment {
    let mut alignment = Alignment {
        aligned_seq1: vec![],
        aligned_seq2: vec![],
    };
    let (mut i, mut j) = matrix.start_key;
    let mut pointer = Pointer::PRIMARY;

    while i > 0 || j > 0 {
        let curr_primary = matrix.primary.get(&(i, j)).unwrap();

        if matches!(matrix.alignment_mode, AlignmentMode::Local) && *curr_primary == 0 {
            break;
        }

        let curr_ins = matrix.aux_ins.get(&(i, j)).unwrap();
        let curr_del = matrix.aux_del.get(&(i, j)).unwrap();

        if matches!(pointer, Pointer::PRIMARY) && curr_primary == curr_del {
            pointer = Pointer::DEL;
        } else if matches!(pointer, Pointer::PRIMARY) && curr_primary == curr_ins {
            pointer = Pointer::INS;
        } else if matches!(pointer, Pointer::DEL)
            && i > 0
            && *matrix.aux_del.get(&(i - 1, j)).unwrap() == curr_del + GAP_EXTENSION_PENALTY
        {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push('-');
            i -= 1;
        } else if matches!(pointer, Pointer::DEL)
            && i > 0
            && *matrix.primary.get(&(i - 1, j)).unwrap() == curr_del + GAP_PENALTY
        {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push('-');
            pointer = Pointer::PRIMARY;
            i -= 1;
        } else if matches!(pointer, Pointer::INS)
            && j > 0
            && *matrix.aux_ins.get(&(i, j - 1)).unwrap() == curr_ins + GAP_EXTENSION_PENALTY
        {
            alignment.aligned_seq1.push('-');
            alignment.aligned_seq2.push(seq2[j - 1]);
            j -= 1;
        } else if matches!(pointer, Pointer::INS)
            && j > 0
            && *matrix.primary.get(&(i, j - 1)).unwrap() == curr_ins + GAP_PENALTY
        {
            alignment.aligned_seq1.push('-');
            alignment.aligned_seq2.push(seq2[j - 1]);
            pointer = Pointer::PRIMARY;
            j -= 1;
        } else {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push(seq2[j - 1]);
            pointer = Pointer::PRIMARY;
            i -= 1;
            j -= 1;
        }
    }
    alignment.aligned_seq1.reverse();
    alignment.aligned_seq2.reverse();

    alignment
}

fn split_chars(sequence: &str) -> Vec<char> {
    sequence.trim().chars().collect()
}

pub fn align_sequences(sequence1: &str, sequence2: &str, local: bool) -> Alignment {
    let seq1_chars = split_chars(sequence1);
    let seq2_chars = split_chars(sequence2);

    let mode = if local {
        AlignmentMode::Local
    } else {
        AlignmentMode::Global
    };

    let matrix = Matrix::create(&seq1_chars, &seq2_chars, mode);
    retrieve_alignment(&seq1_chars, &seq2_chars, &matrix)
}
