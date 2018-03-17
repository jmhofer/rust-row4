use row4::Column;
use row4::COLUMNS;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MoveList {
    moves: [Option<Column>; 7],
    len: u8
}

impl MoveList {
    pub fn full() -> MoveList {
        MoveList::from(&COLUMNS)
    }

    pub fn from(moves: &[Column]) -> MoveList {
        let mut ml = [None; 7];
        for i in 0..7 {
            if i < moves.len() {
                ml[i] = Some(moves[i]);
            } else {
                ml[i] = None;
            }
        }
        MoveList { moves: ml, len: moves.len() as u8 }
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn at(&self, index: usize) -> u8 {
        self.moves[index].unwrap()
    }

    pub fn moves(&self) -> Vec<Column> {
        let mut moves = Vec::new();
        let mut index = 0;
        while let Some(column) = self.moves[index] {
            moves.push(column);
            index += 1;
            if index == 7 { break };
        }
        moves
    }
}

#[test]
fn test_full_move_list() {
    let full = MoveList::full();
    assert_eq!(full.moves.to_vec(), COLUMNS.iter().map(|&x| Some(x)).collect::<Vec<_>>());
    assert_eq!(full.moves(), COLUMNS.to_vec());
    assert_eq!(full.len(), 7);
}

#[test]
fn test_empty_move_list() {
    let empty = MoveList::from(&[]);
    assert_eq!(empty.moves, [None; 7]);
    assert_eq!(empty.moves(), vec!());
    assert_eq!(empty.len(), 0);
}

#[test]
fn test_partial_move_list() {
    let moves = MoveList::from(&[3, 1, 6]);
    assert_eq!(moves.moves, [Some(3), Some(1), Some(6), None, None, None, None]);
    assert_eq!(moves.moves(), vec!(3, 1, 6));
    assert_eq!(moves.len(), 3);
}
