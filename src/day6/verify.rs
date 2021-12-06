#[cfg(test)]
mod tests {
    use crate::day6::{
        day6a,
        day6b,
    };
    const TEST_INPUT: [usize; 5] = [3, 4, 3, 1, 2];

    const INPUT: [usize; 300] = [
        5, 1, 2, 1, 5, 3, 1, 1, 1, 1, 1, 2, 5, 4, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1,
        1, 1, 2, 1, 5, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 1, 4, 3, 1, 1, 4, 1,
        1, 1, 1, 2, 1, 1, 1, 5, 1, 1, 5, 1, 1, 1, 4, 4, 2, 5, 1, 1, 5, 1, 1, 2,
        2, 1, 2, 1, 1, 5, 3, 1, 2, 1, 1, 3, 1, 4, 3, 3, 1, 1, 3, 1, 5, 1, 1, 3,
        1, 1, 4, 4, 1, 1, 1, 5, 1, 1, 1, 4, 4, 1, 3, 1, 4, 1, 1, 4, 5, 1, 1, 1,
        4, 3, 1, 4, 1, 1, 4, 4, 3, 5, 1, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 4,
        1, 1, 3, 1, 1, 2, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 5, 5, 1, 1,
        1, 5, 1, 1, 1, 1, 5, 1, 3, 2, 1, 1, 5, 2, 3, 1, 2, 2, 2, 5, 1, 1, 3, 1,
        1, 1, 5, 1, 4, 1, 1, 1, 3, 2, 1, 3, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 2, 3,
        1, 5, 1, 4, 1, 3, 5, 1, 1, 1, 2, 2, 1, 1, 1, 1, 5, 4, 1, 1, 3, 1, 2, 4,
        2, 1, 1, 3, 5, 1, 1, 1, 3, 1, 1, 1, 5, 1, 1, 1, 1, 1, 3, 1, 1, 1, 4, 1,
        1, 1, 1, 2, 2, 1, 1, 1, 1, 5, 3, 1, 2, 3, 4, 1, 1, 5, 1, 2, 4, 2, 1, 1,
        1, 2, 1, 1, 1, 1, 1, 1, 1, 4, 1, 5,
    ];

    #[test]
    fn day6a_naive_example() {
        fn naive_day6a(state: Vec<usize>) -> usize {
            (0..80)
                .into_iter()
                .fold(state, |s, _| {
                    s.into_iter()
                        .flat_map(|fish| {
                            if fish >= 1 {
                                [fish - 1].to_vec()
                            } else {
                                [6, 8].to_vec()
                            }
                        })
                        .collect()
                })
                .len()
        }
        assert_eq!(naive_day6a(TEST_INPUT.to_vec()), 5934)
    }
    #[test]
    fn day6a_example() { assert_eq!(day6a(TEST_INPUT.to_vec()), 5934) }
    #[test]
    fn day6a_real() { assert_eq!(day6a(INPUT.to_vec()), 383160) }
    #[test]
    fn day6b_example() { assert_eq!(day6b(TEST_INPUT.to_vec()), 26984457539) }
    #[test]
    fn day6b_real() { assert_eq!(day6b(INPUT.to_vec()), 1721148811504) }
}
