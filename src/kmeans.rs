use crate::{Distance, Zero};
use std::iter::Sum;
use std::ops::{AddAssign, Div, Index};

pub trait Point<T>:
    Copy
    + Distance<Output = f32>
    + Index<usize, Output = f32>
    + AddAssign
    + Div<f32, Output = T>
    + Sum
    + Zero
{
}
impl<T> Point<T> for T where
    T: Copy
        + Distance<Output = f32>
        + Index<usize, Output = f32>
        + AddAssign
        + Div<f32, Output = T>
        + Sum
        + Zero
{
}

pub fn fit<T>(points: &[T], k: usize, tresh: f32, max_iter: usize) -> Vec<T>
where
    T: Point<T>,
{
    let mut centroids: Vec<T> = naive_sharding(points, k);

    // Update centroids
    let mut max_change = 0.0;
    let mut iters = max_iter;
    for i in 0..max_iter {
        let old_centroids = centroids.clone();
        centroids = points
            .iter()
            .fold(vec![(0usize, T::zero()); k], |mut acc, p| {
                let mut min_dist = centroids[0].distance2(p);
                let mut min_idx = 0;
                // change to fluent api
                for (i, c) in centroids[1..].iter().enumerate() {
                    let dist = c.distance2(p);
                    if dist < min_dist {
                        min_dist = dist;
                        min_idx = i + 1;
                    }
                }
                acc[min_idx].0 += 1;
                acc[min_idx].1 += *p;
                acc
            })
            .into_iter()
            .map(|(count, sum)| sum / count as f32)
            .collect();

        max_change = (0..k).fold(0f32, |acc, i| {
            acc.max(centroids[i].distance(&old_centroids[i]))
        });
        if max_change <= tresh {
            iters = i;
            break;
        }
    }
    println!("Max change was {} after {} iterations", max_change, iters);

    centroids
}

fn naive_sharding<T>(points: &[T], k: usize) -> Vec<T>
where
    T: Point<T>,
{
    let mut composites: Vec<_> = points
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p[0] + p[1] + p[2]))
        .collect();
    composites.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let shard_size = composites.len().div_ceil(k);
    composites
        .chunks(shard_size)
        .map(|shard| shard.iter().map(|&(i, _)| points[i]).sum::<T>() / shard.len() as f32)
        .collect::<Vec<_>>()
}
