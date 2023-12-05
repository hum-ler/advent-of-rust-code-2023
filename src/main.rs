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

fn main() {
    {
        let day_1_input = input("input/day-1.txt");
        println!("-- Day 1 --");
        println!(" Part 1: {}", day_1_part_1::run(&day_1_input));
        println!(" Part 2: {}", day_1_part_2::run(&day_1_input));
    }
    {
        let day_2_input = input("input/day-2.txt");
        println!("-- Day 2 --");
        println!(" Part 1: {}", day_2_part_1::run(&day_2_input));
        println!(" Part 2: {}", day_2_part_2::run(&day_2_input));
    }
    {
        let day_3_input = input("input/day-3.txt");
        println!("-- Day 3 --");
        println!(" Part 1: {}", day_3_part_1::run(&day_3_input));
        println!(" Part 2: {}", day_3_part_2::run(&day_3_input));
    }
    {
        let day_4_input = input("input/day-4.txt");
        println!("-- Day 4 --");
        println!(" Part 1: {}", day_4_part_1::run(&day_4_input));
        println!(" Part 1: {}", day_4_part_2::run(&day_4_input));
    }
    {
        let day_5_input = input("input/day-5.txt");
        println!("-- Day 5 --");
        println!(" Part 1: {}", day_5_part_1::run(&day_5_input));
        println!(" Part 2: {}", day_5_part_2::run(&day_5_input));
    }
}

pub fn input<P>(path: P) -> String
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(path).unwrap()
}
