use rand::thread_rng;
use rand::Rng;
use std::ops::Range;
struct Table {
    table: [u8; 81],
}

impl Table {
    fn new(tab: &[u8]) -> Self {
        Self { table }
    }

    fn check_available_numbers_in_line(&self, index: usize) -> Vec<u8> {
        let range: Range<usize> = match index {
            0..=8 => Range { start: 0, end: 9 },
            9..=17 => Range { start: 9, end: 18 },
            18..=26 => Range { start: 18, end: 27 },
            27..=35 => Range { start: 27, end: 36 },
            36..=44 => Range { start: 36, end: 45 },
            45..=53 => Range { start: 45, end: 54 },
            54..=62 => Range { start: 54, end: 63 },
            63..=71 => Range { start: 63, end: 72 },
            72..=80 => Range { start: 72, end: 81 },
            _ => todo!(),
        };
        let mut missing_nums = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut nums = Vec::new();
        for idx in range {
            if self.table[idx] != 0 {
                nums.push(self.table[idx as usize]);
            }
        }

        missing_nums.retain(|x| !nums.contains(x));

        //println!("Miss nums: {:?}", missing_nums);
        missing_nums
    }

    fn check_available_numbers_in_column(&self, index: usize) -> Vec<u8> {
        let range: Vec<u8> = match index {
            0 | 9 | 18 | 27 | 36 | 45 | 54 | 63 | 72 => vec![0, 9, 18, 27, 36, 45, 54, 63, 72],
            1 | 10 | 19 | 28 | 37 | 46 | 55 | 64 | 73 => vec![1, 10, 19, 28, 37, 46, 55, 64, 73],
            2 | 11 | 20 | 29 | 38 | 47 | 56 | 65 | 74 => vec![2, 11, 20, 29, 38, 47, 56, 65, 74],
            3 | 12 | 21 | 30 | 39 | 48 | 57 | 66 | 75 => vec![3, 12, 21, 30, 39, 48, 57, 66, 75],
            4 | 13 | 22 | 31 | 40 | 49 | 58 | 67 | 76 => vec![4, 13, 22, 31, 40, 49, 58, 67, 76],
            5 | 14 | 23 | 32 | 41 | 50 | 59 | 68 | 77 => vec![5, 14, 23, 32, 41, 50, 59, 68, 77],
            6 | 15 | 24 | 33 | 42 | 51 | 60 | 69 | 78 => vec![6, 15, 24, 33, 42, 51, 60, 69, 78],
            7 | 16 | 25 | 34 | 43 | 52 | 61 | 70 | 79 => vec![7, 16, 25, 34, 43, 52, 61, 70, 79],
            8 | 17 | 26 | 35 | 44 | 53 | 62 | 71 | 80 => vec![8, 17, 26, 35, 44, 53, 62, 71, 80],
            _ => todo!(),
        };
        let mut missing_nums = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut nums = Vec::new();
        for idx in range.iter() {
            if self.table[*idx as usize] != 0 {
                nums.push(self.table[*idx as usize]);
            }
        }

        missing_nums.retain(|x| !nums.contains(x));

        //println!("Miss nums columns: {:?}", missing_nums);
        missing_nums
    }

    fn check_available_numbers_in_square(&self, index: usize) -> Vec<u8> {
        let range: Vec<u8> = match index {
            0 | 1 | 2 | 9 | 10 | 11 | 18 | 19 | 20 => vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
            3 | 4 | 5 | 12 | 13 | 14 | 21 | 22 | 23 => vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
            6 | 7 | 8 | 15 | 16 | 17 | 24 | 25 | 26 => vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
            27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            60 | 61 | 62 | 69 | 70 | 71 | 78 | 79 | 80 => vec![60, 61, 62, 69, 70, 71, 78, 79, 80],
            _ => todo!(),
        };
        let mut missing_nums = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut nums = Vec::new();
        for idx in range.iter() {
            if self.table[*idx as usize] != 0 {
                nums.push(self.table[*idx as usize]);
            }
        }

        missing_nums.retain(|x| !nums.contains(x));

        //println!("Miss nums columns: {:?}", missing_nums);
        missing_nums
    }
}

const table: [u8; 81] = [
    0, 0, 3, 0, 8, 0, 0, 0, 0, 7, 0, 4, 3, 0, 0, 0, 8, 0, 5, 8, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 7, 4, 0, 0, 0, 0, 0, 0, 5, 8, 0, 5, 0, 0, 0, 0, 0, 3, 6, 9, 0, 0, 5, 2, 8, 7, 0, 4, 0,
    0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 7, 6, 0, 0,
];

fn check_single_value_to_add(tab: &mut Table) {
    let mut stop_flag = true;

    while stop_flag {
        stop_flag = false;

        for idx in 0..81 {
            if tab.table[idx] != 0 {
                continue;
            }
            let miss_l = tab.check_available_numbers_in_line(idx);
            let miss_c = tab.check_available_numbers_in_column(idx);
            let miss_s = tab.check_available_numbers_in_square(idx);

            let mut possible = Vec::new();

            for n in miss_l.iter() {
                if miss_c.contains(n) && miss_s.contains(n) {
                    possible.push(n);
                }
            }

            if possible.len() == 1 {
                //println!("Updating value at cell: {idx}");
                stop_flag = true;
                tab.table[idx] = **possible.get(0).unwrap();
            }

            //println!("Possible nums for cell {:?}: {:?}", idx, possible);
        }
    }
}

fn main() {
    let mut tab = Table::new(&table);

    // Check immediately if we have single value to put into the table
    check_single_value_to_add(&mut tab);

    let mut solved = false;

    while !solved {
        solved = true;
        // Clone the table
        let mut n_tab = Table::new(&tab.table);

        for idx in 0..81 {
            if n_tab.table[idx] != 0 {
                continue;
            }

            let miss_l = n_tab.check_available_numbers_in_line(idx);
            let miss_c = n_tab.check_available_numbers_in_column(idx);
            let miss_s = n_tab.check_available_numbers_in_square(idx);
            let mut possible = Vec::new();

            for n in miss_l.iter() {
                if miss_c.contains(n) && miss_s.contains(n) {
                    possible.push(n);
                }
            }

            if possible.len() == 0 {
                solved = false;
                break;
            }
            let mut rng = thread_rng();
            println!("LEN: {}", possible.len());
            let first_num = rng.gen_range(0, possible.len());
            n_tab.table[idx] = **possible.get(first_num).unwrap();

            check_single_value_to_add(&mut n_tab);
        }
        //println!("The table: {:?}", n_tab.table);
        //std::thread::sleep(std::time::Duration::from_secs(1));
        if solved {
            println!("The table: {:?}", n_tab.table);
        }
    }
}
