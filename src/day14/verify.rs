#[cfg(test)]
mod tests {
    use crate::day14::{
        day14a,
        day14b,
    };

    const RAW_TEST_INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    const RAW_INPUT: &str = r"VCOPVNKPFOOVPVSBKCOF

NO -> K
PO -> B
HS -> B
FP -> V
KN -> S
HV -> S
KC -> S
CS -> B
KB -> V
OB -> V
HN -> S
OK -> N
PC -> H
OO -> P
HF -> S
CB -> C
SB -> V
FN -> B
PH -> K
KH -> P
NB -> F
KF -> P
FK -> N
FB -> P
FO -> H
CV -> V
CN -> P
BN -> N
SC -> N
PB -> K
VS -> N
BP -> P
CK -> O
PS -> N
PF -> H
HB -> S
VN -> V
OS -> V
OC -> O
BB -> F
SK -> S
NF -> F
FS -> S
SN -> N
FC -> S
BH -> N
HP -> C
VK -> F
CC -> N
SV -> H
SO -> F
HH -> C
PK -> P
NV -> B
KS -> H
NP -> H
VO -> C
BK -> V
VV -> P
HK -> B
CF -> B
BF -> O
OV -> B
OH -> C
PP -> S
SP -> S
CH -> B
OF -> F
NK -> F
FV -> F
KP -> O
OP -> O
SS -> P
CP -> H
BO -> O
KK -> F
HC -> N
KO -> V
CO -> F
NC -> P
ON -> P
KV -> C
BV -> K
HO -> F
PV -> H
VC -> O
NH -> B
PN -> H
VP -> O
NS -> N
NN -> S
BS -> H
SH -> P
VB -> V
VH -> O
FH -> K
FF -> H
SF -> N
BC -> H
VF -> P";

    #[test]
    fn day14a_example() { assert_eq!(day14a(RAW_TEST_INPUT), 1588) }
    #[test]
    fn day14a_real() { assert_eq!(day14a(RAW_INPUT), 2851) }
    #[test]
    fn day14b_example() { assert_eq!(day14b(RAW_TEST_INPUT), 2188189693529) }
    #[test]
    fn day14b_real() { assert_eq!(day14b(RAW_INPUT), 10002813279337) }
}
