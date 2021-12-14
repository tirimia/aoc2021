#[cfg(test)]
mod tests {
    use crate::day12::{
        day12a,
        day12b,
    };

    const RAW_TEST_INPUT: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const RAW_INPUT: &str = r"rf-RL
rf-wz
wz-RL
AV-mh
end-wz
end-dm
wz-gy
wz-dm
cg-AV
rf-AV
rf-gy
end-mh
cg-gy
cg-RL
gy-RL
VI-gy
AV-gy
dm-rf
start-cg
start-RL
rf-mh
AV-start
qk-mh
wz-mh";

    #[test]
    fn day12a_example() { assert_eq!(day12a(RAW_TEST_INPUT), 10) }
    #[test]
    fn day12a_real() { assert_eq!(day12a(RAW_INPUT), 3421) }
    #[test]
    fn day12b_example() { assert_eq!(day12b(RAW_TEST_INPUT), 36) }
    #[test]
    fn day12b_real() { assert_eq!(day12b(RAW_INPUT), 84870) }
}
