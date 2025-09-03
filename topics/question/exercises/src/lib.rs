#![allow(unused)]

fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(nums: &[&str]) -> Result<u32, String> {
    let mut sum = 0;
    for num in nums {
        sum += parse(num)?;
    }
    Ok(sum)
}
