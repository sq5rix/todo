use std::ops::Range;

pub fn get_range(s: &str) -> Option<Range<usize>> {
    let mut first = String::new();
    let mut second = String::new();
    let mut first_filled = false;

    for c in s.chars() {
        // let ch = s.chars().next();
        if c.is_digit(10) {
            if !first_filled {
                first.push(c);
            }
        } else {
            first_filled = true;
        }
        if first_filled {
            if c.is_digit(10) {
                second.push(c);
            }
        }
    }
    // println!("Got: r1: {} r2: {}", first, second);
    let r1 = first.parse().expect("first number parsed bad");
    let r2 = second.parse().expect("second number parsed bad");
    if r1 < r2 {
        return Some(r1..r2);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::get_range;
    #[test]
    fn valid_tests() {
        let valid_data = vec!["5..10", "5x10", "5-10"];
        for test in valid_data {
            assert_eq!(
                get_range(&test),
                Some(5..10),
                "we are testing {} as {:?}",
                test,
                Some(5..10)
            );
        }
    }
    // #[test]
    // fn invalid_tests() {
    //     let invalid_data = vec!["61-6a", "51-10"];
    //     for test in invalid_data {
    //         assert_eq!(get_range(&test), None, "we are testing {} as None", test);
    //     }
    // }
}
