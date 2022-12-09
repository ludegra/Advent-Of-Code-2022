use std::time::Instant;
const DAY_NUMBER: u32 = 4;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let input = input.collect::<Vec<_>>();

    let sum = input.iter().fold(0, |acc, l| {
        let mut split = l.split(',');
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        let mut first_split = first.split('-');
        let mut second_split = second.split('-');

        let first1 = first_split.next().unwrap().parse::<u32>().unwrap();
        let first2 = first_split.next().unwrap().parse::<u32>().unwrap();
        let first_start = u32::min(first1, first2);
        let first_end = u32::max(first1, first2);

        let second1 = second_split.next().unwrap().parse::<u32>().unwrap();
        let second2 = second_split.next().unwrap().parse::<u32>().unwrap();
        let second_start = u32::min(second1, second2);
        let second_end = u32::max(second1, second2);

        if first_start <= second_start && second_end <= first_end  {
            acc + 1
        }
        else if second_start <= first_start && first_end <= second_end {
            acc + 1
        }
        else {
            acc
        }
    });

    println!("{}", sum);
    println!("{}\u{00B5}s", start.elapsed().as_micros());
    let start = Instant::now();

    let sum = input.iter().fold(0, |acc, l| {
        let mut split = l.split(',');
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        let mut first_split = first.split('-');
        let mut second_split = second.split('-');

        let first1 = first_split.next().unwrap().parse::<u32>().unwrap();
        let first2 = first_split.next().unwrap().parse::<u32>().unwrap();
        let first_start = u32::min(first1, first2);
        let first_end = u32::max(first1, first2);

        let second1 = second_split.next().unwrap().parse::<u32>().unwrap();
        let second2 = second_split.next().unwrap().parse::<u32>().unwrap();
        let second_start = u32::min(second1, second2);
        let second_end = u32::max(second1, second2);


        if first_start <= second_start && second_start <= first_end {
            acc + 1
        }
        else if second_start <= first_start && first_start <= second_end {
            acc + 1
        }
        else {
            acc
        }
    });

    println!("{}", sum);
    println!("{}\u{00B5}s", start.elapsed().as_micros());

}
