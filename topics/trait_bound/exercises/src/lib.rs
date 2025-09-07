use std::cmp::PartialOrd;

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

pub fn zip<A, B>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)>
where
    A: Copy,
    B: Copy,
{
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
