//! NumPy 2.2.6 scalar double argsort (BSD-3-Clause).
//! Copyright (c) 2005-2024, NumPy Developers. All rights reserved.
//! Retained license and disclaimer: reference/numpy/LICENSE.txt.
//! Source: npysort/quicksort.cpp::aquicksort_ and npysort_heapsort.h::aheapsort_.
//! This is the ARM64 oracle's path, not NumPy's x86 SIMD dispatch.

fn less(a: f64, b: f64) -> bool {
    a < b || (b.is_nan() && !a.is_nan())
}

/// Ascending index order, preserving NumPy's unstable tie ordering.
pub fn argsort(values: &[f64]) -> Vec<usize> {
    let mut order: Vec<_> = (0..values.len()).collect();
    if order.len() < 2 {
        return order;
    }
    let depth = 2 * (usize::BITS - 1 - order.len().leading_zeros()) as i32;
    let mut stack = vec![(0, order.len() - 1, depth)];
    while let Some((mut lo, mut hi, mut depth)) = stack.pop() {
        if depth < 0 {
            heapsort(values, &mut order[lo..=hi]);
            continue;
        }
        while hi - lo > 15 {
            let mid = lo + ((hi - lo) >> 1);
            if less(values[order[mid]], values[order[lo]]) {
                order.swap(mid, lo);
            }
            if less(values[order[hi]], values[order[mid]]) {
                order.swap(hi, mid);
            }
            if less(values[order[mid]], values[order[lo]]) {
                order.swap(mid, lo);
            }
            let pivot = values[order[mid]];
            let (mut i, mut j) = (lo, hi - 1);
            order.swap(mid, j);
            loop {
                i += 1;
                while less(values[order[i]], pivot) {
                    i += 1;
                }
                j -= 1;
                while less(pivot, values[order[j]]) {
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                order.swap(i, j);
            }
            order.swap(i, hi - 1);
            depth -= 1;
            if i - lo < hi - i {
                stack.push((i + 1, hi, depth));
                hi = i - 1;
            } else {
                stack.push((lo, i - 1, depth));
                lo = i + 1;
            }
        }
        for i in lo + 1..=hi {
            let v = order[i];
            let mut j = i;
            while j > lo && less(values[v], values[order[j - 1]]) {
                order[j] = order[j - 1];
                j -= 1;
            }
            order[j] = v;
        }
    }
    order
}

fn heapsort(values: &[f64], order: &mut [usize]) {
    let mut n = order.len();
    for l in (1..=n / 2).rev() {
        let v = order[l - 1];
        sift(values, order, l, n, v);
    }
    while n > 1 {
        let v = order[n - 1];
        order[n - 1] = order[0];
        n -= 1;
        sift(values, order, 1, n, v);
    }
}

fn sift(values: &[f64], order: &mut [usize], mut i: usize, n: usize, v: usize) {
    let mut j = i * 2;
    while j <= n {
        if j < n && less(values[order[j - 1]], values[order[j]]) {
            j += 1;
        }
        if !less(values[v], values[order[j - 1]]) {
            break;
        }
        order[i - 1] = order[j - 1];
        i = j;
        j *= 2;
    }
    order[i - 1] = v;
}
