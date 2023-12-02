use crate::{Distance, Zero};
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Index, Mul};

pub trait Point<T, U>:
    Debug
    + Copy
    + Distance<Output = f32>
    + Index<usize, Output = U>
    + AddAssign
    + Div<U, Output = T>
    + Sum
    + Zero
{
}
impl<T, U> Point<T, U> for T
where
    T: Debug
        + Copy
        + Distance<Output = f32>
        + Index<usize, Output = U>
        + AddAssign
        + Div<U, Output = T>
        + Sum
        + Zero,
    U: PointInner,
{
}

pub trait PointInner:
    Copy + PartialOrd + Add<Output = Self> + Mul<Output = Self> + From<u16>
{
}
impl<T> PointInner for T where
    T: Copy + PartialOrd + Add<Output = Self> + Mul<Output = Self> + From<u16>
{
}

pub fn fit<T, U>(points: &[T], k: usize, max_iter: usize) -> Vec<T>
where
    T: Point<T, U>,
    U: PointInner,
{
    let mut centroids: Vec<T> = naive_sharding(points, k);

    // Update centroids
    for _ in 0..max_iter {
        let old_centroids = centroids.clone();
        centroids = points
            .iter()
            .fold(vec![(0u32, T::zero()); k], |mut acc, p| {
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
            .map(|(count, sum)| {
                // Convert u32 to U, via u16 (f32 doesn't implement From<u32>)
                let high: U = ((count >> 16) as u16).into();
                let low: U = ((count & 0xFFFF) as u16).into();
                let count = high * 0x100.into() * 0x100.into() + low;
                sum / count
            })
            .collect();

        let max_change = (0..k).fold(0f32, |acc, i| {
            acc.max(centroids[i].distance(&old_centroids[i]))
        });
        if max_change < 0.0001 {
            break;
        }
    }

    centroids
}

fn naive_sharding<T, U>(points: &[T], k: usize) -> Vec<T>
where
    T: Point<T, U>,
    U: PointInner,
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
        .map(|shard| shard.iter().map(|&(i, _)| points[i]).sum::<T>() / (shard.len() as u16).into())
        .collect::<Vec<_>>()
}
