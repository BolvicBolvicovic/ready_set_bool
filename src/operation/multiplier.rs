use super::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    if b == 0 || a == 0 {
        0
    } else {
        fn multiplier(acc: u32, a: u32, b: u32) -> u32 {
            if b <= 1 {
                acc
            } else {
                multiplier(adder(acc, a), a, b - 1)
            }
        }
        multiplier(adder(a, a), a, b - 1)
    }
}