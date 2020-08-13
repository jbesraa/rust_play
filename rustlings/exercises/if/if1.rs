// if1.rs

pub fn bigger(a: i32, b: i32) -> i32 {
    let result: i32;

    if a > b {
        result = a;
    } else {
        result = b;
    }

    return result;
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
