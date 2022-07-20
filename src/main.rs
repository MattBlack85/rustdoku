use rand::thread_rng;
use rand::Rng;
use std::ops::Range;

struct Table {
    table: [u8; 81],
}

impl Table {
    fn new(tab: [u8; 81]) -> Self {
        Self { table: tab }
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
        missing_nums
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let row_top = "
   +═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+";

        let row_1 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[0],
            self.table[1],
            self.table[2],
            self.table[3],
            self.table[4],
            self.table[5],
            self.table[6],
            self.table[7],
            self.table[8]
        );

        let row_2 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[9],
            self.table[10],
            self.table[11],
            self.table[12],
            self.table[13],
            self.table[14],
            self.table[15],
            self.table[16],
            self.table[17]
        );

        let row_3 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   ╠═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+",
            self.table[18],
            self.table[19],
            self.table[20],
            self.table[21],
            self.table[22],
            self.table[23],
            self.table[24],
            self.table[25],
            self.table[26]
        );

        let row_4 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[27],
            self.table[28],
            self.table[29],
            self.table[30],
            self.table[31],
            self.table[32],
            self.table[33],
            self.table[34],
            self.table[35]
        );

        let row_5 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[36],
            self.table[37],
            self.table[38],
            self.table[39],
            self.table[40],
            self.table[41],
            self.table[42],
            self.table[43],
            self.table[44]
        );

        let row_6 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   ╠═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+",
            self.table[45],
            self.table[46],
            self.table[47],
            self.table[48],
            self.table[49],
            self.table[50],
            self.table[51],
            self.table[52],
            self.table[53]
        );

        let row_7 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[54],
            self.table[55],
            self.table[56],
            self.table[57],
            self.table[58],
            self.table[59],
            self.table[60],
            self.table[61],
            self.table[62]
        );

        let row_8 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +-------+-------+-------+-------+-------+-------+-------+-------+-------+",
            self.table[63],
            self.table[64],
            self.table[65],
            self.table[66],
            self.table[67],
            self.table[68],
            self.table[69],
            self.table[70],
            self.table[71]
        );

        let row_9 = format!(
            "
   ║       |       |       ║       |       |       ║       |       |       ║
   ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║  {}    |  {}    |  {}    ║
   ║       |       |       ║       |       |       ║       |       |       ║
   +═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+═══════+",
            self.table[72],
            self.table[73],
            self.table[74],
            self.table[75],
            self.table[76],
            self.table[77],
            self.table[78],
            self.table[79],
            self.table[80]
        );

        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}",
            row_top, row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8, row_9
        )
    }
}

const _TABLE: [u8; 81] = [
    0, 0, 3, 0, 8, 0, 0, 0, 0, 7, 0, 4, 3, 0, 0, 0, 8, 0, 5, 8, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 7, 4, 0, 0, 0, 0, 0, 0, 5, 8, 0, 5, 0, 0, 0, 0, 0, 3, 6, 9, 0, 0, 5, 2, 8, 7, 0, 4, 0,
    0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 7, 6, 0, 0,
];

const _EVIL_TABLE_1: [u8; 81] = [
    0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 7, 0, 0, 7, 0, 4, 6, 0, 0, 0, 0, 1, 3, 0, 9, 0, 8,
    0, 1, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4,
    0, 1, 0, 9, 0, 3, 0, 0, 0, 0, 0, 8, 0, 0, 0, 9, 0,
];

const EVIL_TABLE_2: [u8; 81] = [
    0, 0, 0, 0, 0, 4, 8, 0, 0, 0, 0, 3, 2, 8, 0, 0, 0, 5, 0, 2, 0, 0, 0, 6, 0, 0, 0, 0, 0, 5, 0, 0,
    0, 0, 7, 0, 0, 3, 0, 9, 1, 0, 6, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 9, 0, 8, 3, 0, 1, 0, 0, 1,
    0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 4, 0, 0, 0, 0,
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
                stop_flag = true;
                tab.table[idx] = **possible.get(0).unwrap();
            }
        }
    }
}

fn main() {
    let mut tab = Table::new(EVIL_TABLE_2.clone());

    // Check immediately if we have single value to put into the table
    check_single_value_to_add(&mut tab);

    let mut solved = false;

    while !solved {
        solved = true;
        // Clone the table
        let mut n_tab = Table::new(tab.table);

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
            let first_num = rng.gen_range(0, possible.len());
            n_tab.table[idx] = **possible.get(first_num).unwrap();

            check_single_value_to_add(&mut n_tab);
        }

        if solved {
            println!("The table: {}", n_tab);
        }
    }
}
