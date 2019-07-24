/// help parse lib for todo app
use std::ops::Range;

/// emun holding returning value for parsing function
/// It return an usize integer or a range
#[derive(Debug, PartialEq)]
pub enum ReturnItem {
    IntNum(usize),
    IntRange(Range<usize>),
    None,
}

/// get item set - an usize integer or a range of item
pub fn get_item_set(s: &str) -> ReturnItem {
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
    if let Ok(r1) = first.parse() {
        if let Ok(r2) = second.parse() {
            if r1 < r2 {
                return ReturnItem::IntRange(r1..r2);
            } else {
                return ReturnItem::None;
            }
        } else {
            return ReturnItem::IntNum(r1);
        };
    } else {
        return ReturnItem::None;
    };
}

#[cfg(test)]
mod tests {
    use super::get_item_set;
    use super::ReturnItem;
    #[test]
    fn valid_range_tests() {
        let valid_data = vec!["5..10", "5x10", "5-10"];
        for test in valid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::IntRange(5..10),
                "we are testing {} as {:?}",
                test,
                ReturnItem::IntRange(5..10)
            );
        }
    }
    #[test]
    fn valid_single_tests() {
        let valid_data = vec!["5", "5xxx", "5..."];
        for test in valid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::IntNum(5),
                "we are testing {} as {:?}",
                test,
                ReturnItem::IntNum(5)
            );
        }
    }
    #[test]
    fn invalid_tests() {
        let invalid_data = vec!["61-6a", "aaa5", "61-6", "xxx", "61-6"];
        for test in invalid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::None,
                "we are testing {} as None",
                test
            );
        }
    }
}
