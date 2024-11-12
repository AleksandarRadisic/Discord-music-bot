use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle_vec<T>(vec: &mut Vec<T>) {
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
}

pub fn shuffle_vec_excluding_first<T>(vec: &mut Vec<T>) {
    if vec.len() > 1 {
        let mut rng = thread_rng();
        vec[1..].shuffle(&mut rng);
    }
}

pub fn shuffle_vecdeque_excluding_first<T>(vec_deque: &mut VecDeque<T>) {
    if vec_deque.len() > 1 {
        let mut temp_vec: Vec<_> = vec_deque.drain(1..).collect();
        shuffle_vec(& mut temp_vec);
        vec_deque.extend(temp_vec);
    }
}