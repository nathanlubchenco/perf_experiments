use crate::cli::NnImpls;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::cell::RefCell;
use std::thread;

thread_local!(static HAS_PRINTED: RefCell<bool> = RefCell::new(false));


fn distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn heap(input_vector: &Vec<f32>, search_space: &Vec<Vec<f32>>, k: &usize) -> Vec<Vec<f32>> {
    let mut heap = BinaryHeap::with_capacity(*k);

    search_space
        .iter()
        .enumerate()
        .for_each ( |(index, vec)| {
        let dist = distance(&input_vector, vec);
        // start by filling the heap to size
        if heap.len() < *k {
            // reversing here to make this a min-heap
            // also keeping track of the index rather than putting the entire vec on the heap
            heap.push((Reverse(OrderedFloat(dist)), index));
        } else if dist < *heap.peek().unwrap().0.0 {
            heap.pop();
            heap.push((Reverse(OrderedFloat(dist)), index))
        }

    });

    heap.into_iter()
        .map(|(_, index)| search_space[index].clone()) //can we avoid the clone here, if k is small probably doesn't matter much
        .collect()

}

fn parallel_sort(input_vector: &Vec<f32>, search_space: &Vec<Vec<f32>>, k: &usize) -> Vec<Vec<f32>> {
    let mut distances: Vec<(usize, OrderedFloat<f32>)> = search_space
        .par_iter()
        .enumerate()
        .map(|(index, vec)| {
            HAS_PRINTED.with(|has_printed| {
                if !*has_printed.borrow() {
                    println!("Number of threads = {}", rayon::current_num_threads());
                    *has_printed.borrow_mut() = true;
                }
            });
            (index, OrderedFloat(distance(input_vector, vec)))
        })
        .collect();

    distances.sort_by( |a,b| a.1.total_cmp(&b.1));

    distances
        .iter()
        .take(*k)
        .map(|(index, _)| search_space[*index].clone())
        .collect()

}

pub fn k_nearest(input_vector: &Vec<f32>, search_space: &Vec<Vec<f32>>, k: &usize, nn: &NnImpls) -> Vec<Vec<f32>> {
     match nn {
        NnImpls::Heap  => {
            heap(input_vector, search_space, k)
        },
         NnImpls::ParallelSort  => {
             parallel_sort(input_vector, search_space, k)
         },
        _ => {
            todo!("This case is not yet implemented");
        }
    }

}
