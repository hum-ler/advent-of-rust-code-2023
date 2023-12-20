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
mod day_14_part_1;
mod day_14_part_2;
mod day_15_part_1;
mod day_15_part_2;
mod day_16_part_1;
mod day_16_part_2;
mod day_17_part_1;
mod day_17_part_2;
mod day_17_part_2_array;
mod day_18_part_1;
mod day_18_part_1_flood_exterior;
mod day_18_part_2;
mod day_19_part_1;
mod day_19_part_2;
mod day_20_part_1;
mod day_20_part_2;

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
pub use day_14_part_1::run as run_day_14_part_1;
pub use day_14_part_2::run as run_day_14_part_2;
pub use day_15_part_1::run as run_day_15_part_1;
pub use day_15_part_2::run as run_day_15_part_2;
pub use day_16_part_1::run as run_day_16_part_1;
pub use day_16_part_2::run as run_day_16_part_2;
pub use day_17_part_1::run as run_day_17_part_1;
pub use day_17_part_2::run as run_day_17_part_2;
pub use day_17_part_2_array::run as run_day_17_part_2_array;
pub use day_18_part_1_flood_exterior::run as run_day_18_part_1_flood_exterior;
pub use day_18_part_1::run as run_day_18_part_1;
pub use day_18_part_2::run as run_day_18_part_2;
pub use day_19_part_1::run as run_day_19_part_1;
pub use day_19_part_2::run as run_day_19_part_2;
pub use day_20_part_1::run as run_day_20_part_1;
pub use day_20_part_2::run as run_day_20_part_2;
