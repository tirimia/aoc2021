#[cfg(test)]
mod tests {
    use crate::day11::{
        day11a,
        day11b,
        input_to_octopi,
    };

    const RAW_TEST_INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const RAW_INPUT: &str = r"4658137637
3277874355
4525611183
3128125888
8734832838
4175463257
8321423552
4832145253
8286834851
4885323138";

    #[test]
    fn day11a_example() {
        assert_eq!(day11a(input_to_octopi(RAW_TEST_INPUT), 100), 1656)
    }
    #[test]
    fn day11b_example() {
        assert_eq!(day11b(input_to_octopi(RAW_TEST_INPUT)), 195)
    }
    #[test]
    fn day11a_real() {
        assert_eq!(day11a(input_to_octopi(RAW_INPUT), 100), 1686)
    }
    #[test]
    fn day11b_real() { assert_eq!(day11b(input_to_octopi(RAW_INPUT)), 360) }
}
