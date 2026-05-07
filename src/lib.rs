mod alignment;
mod matrix;

use alignment::{Alignment, AlignmentMode, AlignmentScores};
use matrix::Matrix;
use std::collections::VecDeque;

enum Pointer {
    PRIMARY,
    INS,
    DEL,
}

fn retrieve_alignment(
    seq1: Vec<char>,
    seq2: Vec<char>,
    scores: AlignmentScores,
    matrix: Matrix,
) -> Alignment {
    let mut aligned_seq1: VecDeque<char> = VecDeque::new();
    let mut aligned_seq2: VecDeque<char> = VecDeque::new();

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
            && *matrix.aux_del.get(&(i - 1, j)).unwrap() == curr_del + scores.gap_extension_penalty
        {
            aligned_seq1.push_front(seq1[i - 1]);
            aligned_seq2.push_front('-');
            i -= 1;
        } else if matches!(pointer, Pointer::DEL)
            && i > 0
            && *matrix.primary.get(&(i - 1, j)).unwrap() == curr_del + scores.gap_penalty
        {
            aligned_seq1.push_front(seq1[i - 1]);
            aligned_seq2.push_front('-');
            pointer = Pointer::PRIMARY;
            i -= 1;
        } else if matches!(pointer, Pointer::INS)
            && j > 0
            && *matrix.aux_ins.get(&(i, j - 1)).unwrap() == curr_ins + scores.gap_extension_penalty
        {
            aligned_seq1.push_front('-');
            aligned_seq2.push_front(seq2[j - 1]);
            j -= 1;
        } else if matches!(pointer, Pointer::INS)
            && j > 0
            && *matrix.primary.get(&(i, j - 1)).unwrap() == curr_ins + scores.gap_penalty
        {
            aligned_seq1.push_front('-');
            aligned_seq2.push_front(seq2[j - 1]);
            pointer = Pointer::PRIMARY;
            j -= 1;
        } else {
            aligned_seq1.push_front(seq1[i - 1]);
            aligned_seq2.push_front(seq2[j - 1]);
            pointer = Pointer::PRIMARY;
            i -= 1;
            j -= 1;
        }
    }

    Alignment {
        seq1,
        seq2,
        aligned_seq1,
        aligned_seq2,
        start_idx_seq1: i,
        start_idx_seq2: j,
        end_idx_seq1: matrix.start_key.0,
        end_idx_seq2: matrix.start_key.1,
    }
}

pub fn align_sequences(
    seq1_chars: Vec<char>,
    seq2_chars: Vec<char>,
    match_score: i32,
    mismatch_penalty: i32,
    gap_penalty: i32,
    gap_extension_penalty: i32,
    local: bool,
) -> Alignment {
    let mode = if local {
        AlignmentMode::Local
    } else {
        AlignmentMode::Global
    };

    let scores = AlignmentScores {
        match_score,
        mismatch_penalty,
        gap_penalty,
        gap_extension_penalty,
    };

    let matrix = Matrix::create(&seq1_chars, &seq2_chars, mode, &scores);
    retrieve_alignment(seq1_chars, seq2_chars, scores, matrix)
}
