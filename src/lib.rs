use itertools::Itertools;
use std::collections::HashMap;

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

fn create_matrix(seq1: &Vec<char>, seq2: &Vec<char>) -> HashMap<(usize, usize), i32> {
    let mut matrix: HashMap<(usize, usize), i32> = HashMap::new();

    matrix.insert((0, 0), 0);

    for i in 1..=seq1.len() {
        matrix.insert((i, 0), i as i32);
    }
    for j in 1..=seq2.len() {
        matrix.insert((0, j), j as i32);
    }
    for i in 1..=seq1.len() {
        for j in 1..=seq2.len() {
            let insertion = matrix.get(&(i, j - 1)).unwrap() + 1;
            let deletion = matrix.get(&(i - 1, j)).unwrap() + 1;
            let match_or_mismatch = if seq1[i - 1] == seq2[j - 1] {
                *matrix.get(&(i - 1, j - 1)).unwrap()
            } else {
                matrix.get(&(i - 1, j - 1)).unwrap() + 1
            };
            let score = *[insertion, deletion, match_or_mismatch]
                .iter()
                .min()
                .unwrap();
            matrix.insert((i, j), score);
        }
    }

    matrix
}

fn retrieve_alignment(
    seq1: &Vec<char>,
    seq2: &Vec<char>,
    matrix: &HashMap<(usize, usize), i32>,
) -> Alignment {
    let mut alignment = Alignment {
        aligned_seq1: vec![],
        aligned_seq2: vec![],
    };
    let mut i = seq1.len();
    let mut j = seq2.len();

    while i > 0 || j > 0 {
        if i > 0 && *matrix.get(&(i - 1, j)).unwrap() == matrix.get(&(i, j)).unwrap() - 1 {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push('-');
            i -= 1;
        } else if j > 0 && *matrix.get(&(i, j - 1)).unwrap() == matrix.get(&(i, j)).unwrap() - 1 {
            alignment.aligned_seq1.push('-');
            alignment.aligned_seq2.push(seq2[j - 1]);
            j -= 1;
        } else {
            alignment.aligned_seq1.push(seq1[i - 1]);
            alignment.aligned_seq2.push(seq2[j - 1]);
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

    let matrix = create_matrix(&seq1_chars, &seq2_chars);
    retrieve_alignment(&seq1_chars, &seq2_chars, &matrix)
}
