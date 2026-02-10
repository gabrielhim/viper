mod constants;
mod matrix;

use constants::{GAP_EXTENSION_PENALTY, GAP_PENALTY};
use itertools::Itertools;
use matrix::Matrix;

enum VariantType {
    INS,
    DEL,
    MIS,
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
    let mut i = seq1.len();
    let mut j = seq2.len();
    let mut var = VariantType::MIS;

    while i > 0 || j > 0 {
        if matches!(var, VariantType::MIS)
            && matrix.primary.get(&(i, j)).unwrap() == matrix.aux_del.get(&(i, j)).unwrap()
        {
            var = VariantType::DEL;
        } else if matches!(var, VariantType::MIS)
            && matrix.primary.get(&(i, j)).unwrap() == matrix.aux_ins.get(&(i, j)).unwrap()
        {
            var = VariantType::INS;
        } else if matches!(var, VariantType::DEL)
            && i > 0
            && *matrix.aux_del.get(&(i - 1, j)).unwrap()
                == matrix.aux_del.get(&(i, j)).unwrap() + GAP_EXTENSION_PENALTY
        {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push('-');
            i -= 1;
        } else if matches!(var, VariantType::DEL)
            && i > 0
            && *matrix.primary.get(&(i - 1, j)).unwrap()
                == matrix.aux_del.get(&(i, j)).unwrap() + GAP_PENALTY
        {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push('-');
            var = VariantType::MIS;
            i -= 1;
        } else if matches!(var, VariantType::INS)
            && j > 0
            && *matrix.aux_ins.get(&(i, j - 1)).unwrap()
                == matrix.aux_ins.get(&(i, j)).unwrap() + GAP_EXTENSION_PENALTY
        {
            alignment.aligned_seq1.push('-');
            alignment.aligned_seq2.push(seq2[j - 1]);
            j -= 1;
        } else if matches!(var, VariantType::INS)
            && j > 0
            && *matrix.primary.get(&(i, j - 1)).unwrap()
                == matrix.aux_ins.get(&(i, j)).unwrap() + GAP_PENALTY
        {
            alignment.aligned_seq1.push('-');
            alignment.aligned_seq2.push(seq2[j - 1]);
            var = VariantType::MIS;
            j -= 1;
        } else {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push(seq2[j - 1]);
            var = VariantType::MIS;
            i -= 1;
            j -= 1;
        }
    }
    alignment.aligned_seq1.reverse();
    alignment.aligned_seq2.reverse();

    alignment
}

pub fn align_sequences(sequence1: &str, sequence2: &str) -> Alignment {
    let seq1_chars = sequence1.chars().collect();
    let seq2_chars = sequence2.chars().collect();

    let matrix = Matrix::create(&seq1_chars, &seq2_chars);
    retrieve_alignment(&seq1_chars, &seq2_chars, &matrix)
}
