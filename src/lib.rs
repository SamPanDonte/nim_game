/// NimGame is a game where two players take turns placing stones on a field.
/// Stone placement is limited to the following rules:
/// - A player can place 1 stone on their turn.
/// - Stone cannot be placed on the same field as another stone.
/// - Stone cannot be placed on a field that is adjacent to another stone.
///
/// The game ends when a player cannot place a stone on the field anymore.
///
/// This struct holds nim value for games with sizes up to current inner vector length.
///
/// It can be iterated over to calculate nim values for games of larger sizes.
pub struct NimGame(Vec<u32>);

impl NimGame {
    /// Creates a new NimGame with nim value for game of size 0.
    pub fn new() -> Self {
        Self(vec![0])
    }

    /// Creates a new NimGame with nim value for game of size 0 and capacity for games up to
    /// `capacity` size.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nims = Vec::with_capacity(capacity);
        nims.push(0);
        Self(nims)
    }

    /// Solves first `max_game_size` nim games and returns the nim value for last game.
    /// If `max_game_size` is smaller than current inner vector length, returns nim value for
    /// game of size `max_game_size` and does not calculate nim values for larger games.
    pub fn solve(&mut self, max_game_size: usize) -> u32 {
        if max_game_size < self.0.len() {
            return self.0[max_game_size];
        }

        for game_size in self.0.len()..=max_game_size {
            let mut current_nims = vec![false; game_size + 1];

            for field in 0..game_size {
                let first = if field < 1 { 0 } else { field - 1 };
                let second = if field + 2 > game_size {
                    0
                } else {
                    game_size - field - 2
                };

                current_nims[(self.0[first] ^ self.0[second]) as usize] = true;
            }

            for (nim, present) in current_nims.into_iter().enumerate() {
                if !present {
                    self.0.push(nim as u32);
                    break;
                }
            }
        }

        self.0[max_game_size]
    }
}

impl std::default::Default for NimGame {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Index<usize> for NimGame {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::iter::Iterator for NimGame {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.solve(self.0.len());
        Some(next)
    }
}
