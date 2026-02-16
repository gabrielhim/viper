use itertools::Itertools;
use std::cmp;
use std::collections::VecDeque;

use crate::constants::OUTPUT_ROW_SIZE;

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

        let mut out_aligned_seq1: Vec<char> = Vec::new();
        let mut out_aligned_seq2: Vec<char> = Vec::new();

        // Do this for local alignment
        for _ in 0..(cmp::max(self.start_idx_seq1, self.start_idx_seq2)) {
            bridges.push(' ');
        }

        if self.start_idx_seq1 > self.start_idx_seq2 {
            for _ in 0..(self.start_idx_seq1 - self.start_idx_seq2) {
                out_aligned_seq2.push(' ');
            }
        } else if self.start_idx_seq2 > self.start_idx_seq1 {
            for _ in 0..(self.start_idx_seq2 - self.start_idx_seq1) {
                out_aligned_seq1.push(' ');
            }
        }

        if self.start_idx_seq1 > 0 {
            for idx in 0..self.start_idx_seq1 {
                out_aligned_seq1.push(self.seq1[idx]);
            }
        }
        if self.start_idx_seq2 > 0 {
            for idx in 0..self.start_idx_seq2 {
                out_aligned_seq2.push(self.seq2[idx]);
            }
        }

        // Common logic
        for idx in 0..self.aligned_seq1.len() {
            if self.aligned_seq1[idx] == self.aligned_seq2[idx] {
                bridges.push('|');
            } else {
                bridges.push(' ');
            }

            out_aligned_seq1.push(self.aligned_seq1[idx]);
            out_aligned_seq2.push(self.aligned_seq2[idx]);
        }

        // Local alignment
        out_aligned_seq1.extend(&self.seq1[self.end_idx_seq1..]);
        out_aligned_seq2.extend(&self.seq2[self.end_idx_seq2..]);

        // Print output to stdout
        let mut start = 0;

        while start + OUTPUT_ROW_SIZE < out_aligned_seq1.len() {
            let end = start + OUTPUT_ROW_SIZE;
            println!("{}", out_aligned_seq1[start..end].iter().join(""));
            println!("{}", bridges[start..end].iter().join(""));
            println!("{}", out_aligned_seq2[start..end].iter().join(""));
            println!("");
            start += OUTPUT_ROW_SIZE;
        }

        println!("{}", out_aligned_seq1[start..].iter().join(""));
        println!("{}", bridges[start..].iter().join(""));
        println!("{}", out_aligned_seq2[start..].iter().join(""));
    }
}
