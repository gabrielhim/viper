use itertools::Itertools;
use std::cmp;
use std::collections::VecDeque;

pub enum AlignmentMode {
    Global,
    Local,
}

pub struct Alignment {
    pub seq1: Vec<char>,
    pub seq2: Vec<char>,
    pub aligned_seq1: VecDeque<char>,
    pub aligned_seq2: VecDeque<char>,
    pub start_idx_seq1: usize,
    pub start_idx_seq2: usize,
    pub end_idx_seq1: usize,
    pub end_idx_seq2: usize,
}

impl Alignment {
    pub fn print_alignment(&self) {
        let mut bridges: Vec<char> = Vec::new();

        // Do this for local alignment
        for _ in 0..(cmp::max(self.start_idx_seq1, self.start_idx_seq2)) {
            bridges.push(' ');
        }

        let mut pre_seq1: Vec<char> = Vec::new();
        let mut pre_seq2: Vec<char> = Vec::new();

        if self.start_idx_seq1 > self.start_idx_seq2 {
            for _ in 0..(self.start_idx_seq1 - self.start_idx_seq2) {
                pre_seq2.push(' ');
            }
        } else if self.start_idx_seq2 > self.start_idx_seq1 {
            for _ in 0..(self.start_idx_seq2 - self.start_idx_seq1) {
                pre_seq1.push(' ');
            }
        }

        if self.start_idx_seq1 > 0 {
            for idx in 0..self.start_idx_seq1 {
                pre_seq1.push(self.seq1[idx]);
            }
        }
        if self.start_idx_seq2 > 0 {
            for idx in 0..self.start_idx_seq2 {
                pre_seq2.push(self.seq2[idx]);
            }
        }

        let post_seq1 = &self.seq1[self.end_idx_seq1..];
        let post_seq2 = &self.seq2[self.end_idx_seq2..];

        // Common logic
        for idx in 0..self.aligned_seq1.len() {
            if self.aligned_seq1[idx] == self.aligned_seq2[idx] {
                bridges.push('|');
            } else {
                bridges.push(' ');
            }
        }

        println!(
            "{}{}{}",
            pre_seq1.iter().join(""),
            self.aligned_seq1.iter().join(""),
            post_seq1.iter().join(""),
        );
        println!("{}", bridges.iter().join(""));
        println!(
            "{}{}{}",
            pre_seq2.iter().join(""),
            self.aligned_seq2.iter().join(""),
            post_seq2.iter().join(""),
        );
    }
}
