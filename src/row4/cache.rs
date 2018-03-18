use std::collections::HashMap;

use row4::board::Board;

pub struct BoardCache {
    // the key is the red/blue board encoding
    cache: HashMap<(u64, u64), f64>
}

impl BoardCache {
    pub fn new() -> BoardCache {
        BoardCache {
            cache: HashMap::new()
        }
    }

    pub fn store(&mut self, board: &Board, eval: f64) {
        self.cache.insert((board.red, board.blue), eval);
        self.cache.insert((board.blue, board.red), 1.0 - eval); // switched colors, pessimistic eval approximation (due to draws)

        let mirrored_red = BoardCache::mirror(board.red);
        let mirrored_blue = BoardCache::mirror(board.blue);
        self.cache.insert((mirrored_red, mirrored_blue), eval);
        self.cache.insert((mirrored_blue, mirrored_red), 1.0 - eval);
    }

    pub fn get(&self, board: &Board) -> Option<&f64> {
        self.cache.get(&(board.red, board.blue))
    }

    /// mirror the board bit representation
    fn mirror(src: u64) -> u64 {
        let mut src_mask = 1u64 << (6 + 8 * 5);
        let mut target_mask = 1 << (8 * 5);
        let mut target = 0;

        for _row in 0..6 {
            for _column in 0..7 {
                if src & src_mask != 0 {
                    target |= target_mask;
                }
                src_mask >>= 1;
                target_mask <<= 1;
            }
            src_mask >>= 1;
            target_mask >>= 15;
        }
        target
    }
}

#[test]
fn test_mirror() {
    assert_eq!(BoardCache::mirror(0), 0);
    assert_eq!(BoardCache::mirror(1), 1 << 6);
    assert_eq!(BoardCache::mirror(1 | 1 << 1 | 1 << 6), 1 | 1 << 5 | 1 << 6);
    assert_eq!(BoardCache::mirror(525324), 528408);
    assert_eq!(BoardCache::mirror(134219792), 134219780);
}
