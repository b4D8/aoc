struct DayOne {
    cal: Vec<u32>,
}

impl DayOne {
    fn from_file(path: &str) -> Self {
        let data = {
            let mut file = std::fs::File::open(path).unwrap();
            let mut data = String::new();
            let _ = std::io::Read::read_to_string(&mut file, &mut data).unwrap();
            data
        };

        let cal: Vec<u32> = data
            .split("\n\n")
            .map(|elf| {
                elf.lines().fold(0, |acc, cal| {
                    cal.parse::<u32>().unwrap().checked_add(acc).unwrap()
                })
            })
            .collect();

        Self { cal }
    }

    fn desc_cum_sum(&self, len: usize) -> u32 {
        {
            let mut cal: Vec<u32> = self.cal.clone();
            cal.sort_by(|a, b| b.cmp(a));
            cal.truncate(len);
            cal
        }
        .iter()
        .sum()
    }
}

fn main() {
    let day_one = DayOne::from_file("../input");

    let part_one = day_one.desc_cum_sum(1);
    println!("Part One answer is: {} calories.", part_one);
    assert_eq!(part_one, 72070);

    let part_two = day_one.desc_cum_sum(3);
    println!("Part Two answer is: {} calories.", part_two);
    assert_eq!(part_two, 211805);
}
