pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Bank {
    pub batteries: Vec<u8>
}

impl Bank {
    pub fn from_str(input: &str) -> Self {
        let batteries = input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        Bank { batteries }
    }

    pub fn max_voltage(&self) -> u32 {
        let mut index = 0;
        let mut max_voltage: u32 = 0;

        let mut temp = 0;

        for i in 0..self.batteries.len() - 1 {
            if self.batteries[i] > temp {
                temp = self.batteries[i];
                index = i;
            }
        }

        max_voltage = temp as u32 * 10u32;
        index += 1;
        temp = 0;

        for j in index..self.batteries.len() {
            if self.batteries[j] > temp {
                temp = self.batteries[j];
            }
        }

        max_voltage += temp as u32;

        max_voltage
    }

    pub fn max_voltage_part2(&self) -> u64 {
        let mut indexes = vec![];
        let mut index = 0;

        for j in 0..12usize {
            index = self.highest_index(index, j);
            indexes.push(index);
            index += 1;
        }

        let mut max_voltage = 0u64;
        let mut multiplier = 1u64;

        for i in (0..self.batteries.len()).rev() {
            let item = self.batteries[i];
            if indexes.contains(&i) {
                max_voltage += item as u64 * multiplier;
                multiplier *= 10u64;
            }
        }

        max_voltage
    }

    pub fn highest_index(&self, start: usize, limit: usize) -> usize {
        let mut index = 0;
        let mut temp = 0;
        let offset = self.batteries.len() - 11 + limit;

        for i in start..offset {
            if self.batteries[i] > temp {
                temp = self.batteries[i];
                index = i;
            }
        }

        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_voltage() {
        assert_eq!(98, Bank::from_str("987654321111111").max_voltage());
        assert_eq!(89, Bank::from_str("811111111111119").max_voltage());
        assert_eq!(78, Bank::from_str("234234234234278").max_voltage());
        assert_eq!(92, Bank::from_str("818181911112111").max_voltage());
    }

    #[test]
    fn test_max_voltage_part2() {
        assert_eq!(987654321111, Bank::from_str("987654321111111").max_voltage_part2());
        assert_eq!(811111111119, Bank::from_str("811111111111119").max_voltage_part2());
        assert_eq!(434234234278, Bank::from_str("234234234234278").max_voltage_part2());
        assert_eq!(888911112111, Bank::from_str("818181911112111").max_voltage_part2());
    }
}
