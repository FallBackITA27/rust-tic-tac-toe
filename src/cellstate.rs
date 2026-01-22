use std::fmt::Display;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum WinState {
    #[default]
    Unfinished,
    Tie,
    Win,
    Loss,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum CellState {
    #[default]
    Empty,
    Cross,
    Circle,
    Hover,
}

impl From<CellState> for u8 {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Empty => 0,
            CellState::Cross => 1,
            CellState::Circle => 2,
            CellState::Hover => 3,
        }
    }
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => CellState::Empty,
            1 => CellState::Cross,
            2 => CellState::Circle,
            3 => CellState::Hover,
            _ => panic!("Shouldn't pass number >3"),
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ' ',
                Self::Cross => 'X',
                Self::Circle => 'O',
                Self::Hover => '@',
            }
        )
    }
}

#[derive(Debug)]
pub struct GameMatrix<const N: usize>([[CellState; N]; N]);

impl<const N: usize> Default for GameMatrix<N> {
    fn default() -> Self {
        Self([[Default::default(); N]; N])
    }
}

impl<const N: usize> Display for GameMatrix<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..N {
            for _ in 0..N {
                write!(f, "+-")?;
            }
            writeln!(f, "+")?;

            for c in 0..N {
                write!(f, "|{}", self.0[r][c])?
            }
            writeln!(f, "|")?;

            if r == N - 1 {
                for _ in 0..N {
                    write!(f, "+-")?;
                }

                writeln!(f, "+")?;
            }
        }

        Ok(())
    }
}

impl<const N: usize> GameMatrix<N> {
    pub const fn size(&self) -> usize {
        N
    }

    pub fn first_free_cell(
        &self,
        r_in: usize,
        c_in: usize,
        direction: Direction,
    ) -> (usize, usize) {
        let (mut r, mut c) = (r_in, c_in);
        loop {
            let idx = match direction {
                Direction::Right | Direction::Left => r * N + c,
                Direction::Up | Direction::Down => c * N + r,
            };

            let idx = match direction {
                Direction::Right | Direction::Down => (idx + 1) % (N * N),
                Direction::Up | Direction::Left => (idx - 1 + N * N) % (N * N),
            };

            (r, c) = match direction {
                Direction::Right | Direction::Left => (idx / N, idx % N),
                Direction::Up | Direction::Down => (idx % N, idx / N),
            };

            if self.cell_free(r, c) {
                break;
            }
        }

        (r, c)
    }

    pub fn cell_free(&self, r: usize, c: usize) -> bool {
        c < N && r < N && (self.0[r][c] == CellState::Empty || self.0[r][c] == CellState::Hover)
    }

    pub fn write_to_cell(&mut self, r: usize, c: usize, cell_state: CellState) -> Option<()> {
        if c >= N || r >= N {
            return None;
        }
        self.0[r][c] = cell_state;
        Some(())
    }

    pub fn check_win(&self) -> WinState {
        fn counter_match(v: CellState, circle: &mut usize, cross: &mut usize) {
            match v {
                CellState::Circle => *circle += 1,
                CellState::Cross => *cross += 1,
                _ => (),
            }
        }

        let mut out = WinState::Tie;
        for i in 0..N {
            // Col
            let mut found_circle = 0;
            let mut found_cross = 0;

            for j in 0..N {
                counter_match(self.0[i][j], &mut found_circle, &mut found_cross);
                match (found_circle, found_cross) {
                    (_, n) if n == N => return WinState::Win,
                    (n, _) if n == N => return WinState::Loss,
                    (n1, n2) if n1 > 0 && n2 > 0 => break,
                    _ => (),
                }
                if j == N - 1 {
                    found_circle = 0;
                    found_cross = 0;
                }
            }
            match (found_circle, found_cross) {
                (0, _) | (_, 0) => out = WinState::Unfinished,
                _ => (),
            }

            // Row
            let mut found_circle = 0;
            let mut found_cross = 0;
            for j in 0..N {
                counter_match(self.0[j][i], &mut found_circle, &mut found_cross);
                match (found_circle, found_cross) {
                    (_, n) if n == N => return WinState::Win,
                    (n, _) if n == N => return WinState::Loss,
                    (n1, n2) if n1 > 0 && n2 > 0 => break,
                    _ => (),
                }
                if j == N - 1 {
                    found_circle = 0;
                    found_cross = 0;
                }
            }
            match (found_circle, found_cross) {
                (0, _) | (_, 0) => out = WinState::Unfinished,
                _ => (),
            }
        }

        // Diag 1
        let mut found_circle = 0;
        let mut found_cross = 0;
        for i in 0..N {
            counter_match(self.0[i][i], &mut found_circle, &mut found_cross);
            match (found_circle, found_cross) {
                (_, n) if n == N => return WinState::Win,
                (n, _) if n == N => return WinState::Loss,
                (n1, n2) if n1 > 0 && n2 > 0 => break,
                _ => (),
            }
        }
        match (found_circle, found_cross) {
            (0, _) | (_, 0) => out = WinState::Unfinished,
            _ => (),
        }

        // Diag 2
        let mut found_circle = 0;
        let mut found_cross = 0;
        for i in 0..N {
            match self.0[i][N - i - 1] {
                CellState::Circle => found_circle += 1,
                CellState::Cross => found_cross += 1,
                _ => (),
            }
            match (found_circle, found_cross) {
                (_, n) if n == N => return WinState::Win,
                (n, _) if n == N => return WinState::Loss,
                (n1, n2) if n1 > 0 && n2 > 0 => break,
                _ => (),
            }
        }
        match (found_circle, found_cross) {
            (0, _) | (_, 0) => out = WinState::Unfinished,
            _ => (),
        }

        out
    }
}
