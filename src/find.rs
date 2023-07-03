use crate::cli::NnImpls;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use ordered_float::OrderedFloat;

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

pub fn k_nearest(input_vector: &Vec<f32>, search_space: &Vec<Vec<f32>>, k: &usize, nn: &NnImpls) -> Vec<Vec<f32>> {
     match nn {
        NnImpls::Heap  => {
            heap(input_vector, search_space, k)
        },
        _ => {
            todo!("This case is not yet implemented");
        }
    }

}
