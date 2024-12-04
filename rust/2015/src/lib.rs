#[macro_export]
macro_rules! day_input {
    ($day:literal) => {
        include_str!(concat!("../../../inputs/2015/day_", $day, "/input.txt"))
    };
}

pub fn day_21_input() -> (&'static str, &'static str) {
    (
        include_str!("../../../inputs/2015/day_21/shop.txt"),
        include_str!("../../../inputs/2015/day_21/boss.txt"),
    )
}
