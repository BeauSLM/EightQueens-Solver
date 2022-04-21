fn main() {
    Board::default().solve(0);
}

#[derive(Default)]
struct Board([Option<i8>; 8]);

impl Board {
    fn check_square(&self, row: i8, col: i8) -> bool {
        !(0..row).any(|i| {
            let j = self.0[i as usize].expect("Every row before this one is a Some");
            j == col || j - i == col - row || j + i == col + row
        })
    }

    fn solve(&mut self, row: i8) {
        if row > 7 {
            println!("Solved:\n{:?}", self.0.iter().flatten().collect::<Vec<&i8>>());
            std::process::exit(0);
        }
        for col in 0..8 {
            if !self.check_square(row, col) { continue; }
            self.0[row as usize] = Some(col);
            self.solve(row + 1);
            self.0[row as usize] = None;
        }
    }
}
