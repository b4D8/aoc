#[derive(Debug, thiserror::Error)]
pub enum DayOneError {
    #[error(transparent)]
    Read(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] core::num::ParseIntError),
    // You can also get overflow
}

struct DayOne {
    calories: Vec<u32>,
}

impl DayOne {
    fn from_file(path: &str) -> Result<Self, DayOneError> {
        let path = std::path::PathBuf::from(path);
        let mut file = std::fs::File::open(path)?;
        let mut data = String::new();
        let _ = std::io::Read::read_to_string(&mut file, &mut data)?;

        let mut calories = Vec::new();
        let mut init = 0;

        for line in data.split('\n') {
            if line.is_empty() {
                calories.push(init);
                init = 0;
            } else {
                init += line.parse::<u32>()?;
            }
        }

        Ok(Self { calories })
    }

    fn sum_first_n(&self, len: usize) -> u32 {
        let mut cal = self.calories.clone();
        cal.sort_by(|a, b| b.cmp(a));
        cal.truncate(len);
        cal.iter().sum()
    }
}

fn main() -> Result<(), DayOneError> {
    let day_one = DayOne::from_file("../input")?;

    let part_one = day_one.sum_first_n(1);
    println!("Part One answer is: {}.", part_one);
    assert_eq!(part_one, 72070);

    let part_two = day_one.sum_first_n(3);
    println!("Part Two answer is: {}.", part_two);
    assert_eq!(part_two, 211805);

    Ok(())
}
