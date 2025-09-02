pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for num in nums {
        sum += num;
    }
    sum
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..n {
        vec.push(i);
    }
    vec
}
