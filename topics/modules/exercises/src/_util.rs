pub mod log {
    pub fn debug(s: &str) {
        println!("DEBUG: {s}");
    }
}

pub mod vec {
    pub fn first(v: &[u32]) -> Option<u32> {
        let n = v.len();
        if n > 0 {
            Some(v[0])
        } else {
            None
        }
    }

    pub fn last(v: &[u32]) -> Option<u32> {
        let n = v.len();
        if n > 0 {
            Some(v[n - 1])
        } else {
            None
        }
    }
}
