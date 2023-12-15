use crate::clean_lines;

pub fn run(input: &str) -> u64 {
    clean_lines(input).take(1).collect::<Vec<&str>>()[0]
        .split(',')
        .map(hash)
        .sum()
}

pub(crate) fn hash(input: &str) -> u64 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(run(input), 1320);
    }

    #[test]
    fn check_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
