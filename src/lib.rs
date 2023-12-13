mod day_1_part_1;
mod day_1_part_2;
mod day_2_part_1;
mod day_2_part_2;
mod day_3_part_1;
mod day_3_part_2;
mod day_4_part_1;
mod day_4_part_2;
mod day_5_part_1;
mod day_5_part_2;
mod day_6_part_1;
mod day_6_part_2;
mod day_7_part_1;
mod day_7_part_2;
mod day_8_part_1;
mod day_8_part_2;
mod day_8_part_2_nodes;
mod day_8_part_2_slices;
mod day_9_part_1;
mod day_9_part_2;
mod day_10_part_1;
mod day_10_part_2;
mod day_11_part_1;
mod day_11_part_2;
mod day_13_part_1;
mod day_13_part_2;

/// Splits input into lines.
///
/// Also trims, and removes empty lines.
pub(crate) fn clean_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

pub use day_1_part_1::run as run_day_1_part_1;
pub use day_1_part_2::run as run_day_1_part_2;
pub use day_2_part_1::run as run_day_2_part_1;
pub use day_2_part_2::run as run_day_2_part_2;
pub use day_3_part_1::run as run_day_3_part_1;
pub use day_3_part_2::run as run_day_3_part_2;
pub use day_4_part_1::run as run_day_4_part_1;
pub use day_4_part_2::run as run_day_4_part_2;
pub use day_5_part_1::run as run_day_5_part_1;
pub use day_5_part_2::run as run_day_5_part_2;
pub use day_6_part_1::run as run_day_6_part_1;
pub use day_6_part_2::run as run_day_6_part_2;
pub use day_7_part_1::run as run_day_7_part_1;
pub use day_7_part_2::run as run_day_7_part_2;
pub use day_8_part_1::run as run_day_8_part_1;
pub use day_8_part_2::run as run_day_8_part_2;
pub use day_8_part_2_nodes::run as run_day_8_part_2_nodes;
pub use day_8_part_2_slices::run as run_day_8_part_2_slices;
pub use day_9_part_1::run as run_day_9_part_1;
pub use day_9_part_2::run as run_day_9_part_2;
pub use day_10_part_1::run as run_day_10_part_1;
pub use day_10_part_2::run as run_day_10_part_2;
pub use day_11_part_1::run as run_day_11_part_1;
pub use day_11_part_2::run as run_day_11_part_2;
pub use day_13_part_1::run as run_day_13_part_1;
pub use day_13_part_2::run as run_day_13_part_2;
