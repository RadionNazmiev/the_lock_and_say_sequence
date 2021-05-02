use itertools::*;

fn get_lines(n: usize) -> String {
    let mut result = String::new();
    match n {
        0 => (),
        _ => (1..=n).for_each(|x| {
            if x == 1 {
                result.push('1');
            }
            else {
                let index;
                match result.as_bytes().iter().rposition(|&c| c == b',') {
                    Some(i) => index = i+1,
                    None => index = 0,
                };
                let mut sequence = String::new();
                for (key, group) in &result[index..].chars().group_by(|&c| c) {
                    sequence.push_str(&group.count().to_string());
                    sequence.push(key);
                }
                result.push(',');
                result.push_str(&sequence);
            }

        }),
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_lines(2), "1,11");
        assert_eq!(get_lines(3), "1,11,21");
        assert_eq!(get_lines(5), "1,11,21,1211,111221");
    }
}
