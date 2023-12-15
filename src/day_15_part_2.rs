use crate::{clean_lines, day_15_part_1::hash};

pub fn run(input: &str) -> u64 {
    let mut boxes = Vec::<Vec<Lens>>::with_capacity(256);
    (0..256).for_each(|_| boxes.push(Vec::<Lens>::new()));

    clean_lines(input).take(1).collect::<Vec<&str>>()[0]
        .split(',')
        .for_each(|instruction| execute_instruction(instruction, &mut boxes));

    total_focusing_power(&boxes)
}

fn execute_instruction(instruction: &str, boxes: &mut [Vec<Lens>]) {
    if instruction.ends_with('-') {
        execute_remove(instruction.strip_suffix('-').unwrap(), boxes);
    } else {
        let (label, number) = instruction.split_once('=').unwrap();
        execute_upsert(label, number.parse::<u8>().unwrap(), boxes);
    }
}

fn execute_remove(label: &str, boxes: &mut [Vec<Lens>]) {
    let b = &mut boxes[hash(label) as usize];

    if let Some(index) = b.iter().position(|lens| label == lens.label.as_str()) {
        b.remove(index);
    }
}

fn execute_upsert(label: &str, focal_length: u8, boxes: &mut [Vec<Lens>]) {
    let lens = Lens {
        label: String::from(label),
        focal_length,
    };

    let b = &mut boxes[hash(label) as usize];
    let existing_index = b.iter().position(|lens| label == lens.label.as_str());

    b.push(lens);

    if let Some(index) = existing_index {
        b.swap_remove(index);
    }
}

fn total_focusing_power(boxes: &[Vec<Lens>]) -> u64 {
    boxes
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(j, lens)| lens_focusing_power(lens, i, j))
                .sum::<u64>()
        })
        .sum()
}

fn lens_focusing_power(lens: &Lens, box_no: usize, slot_no: usize) -> u64 {
    (box_no as u64 + 1) * (slot_no as u64 + 1) * lens.focal_length as u64
}

struct Lens {
    label: String,
    focal_length: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(run(input), 145);
    }
}
