use std::convert::TryInto;

use super::super::board::{
    self, BoardEvent, BoardEventProcessor, Boardable, Octi, OctiID, Position, Team,
};

use super::{team_index, BOARD_HEIGHT, BOARD_WIDTH, TEAMS};

// optimized board for calculations
// octis indexed by pos and not id because pos is used more often
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    turn: Team,
    octis: [Option<Octi>; BOARD_WIDTH * BOARD_HEIGHT],
    arr_counts: [u32; TEAMS],
}

impl Board {
    pub fn new(board: &board::Board) -> Board {
        let octis: [usize; BOARD_WIDTH * BOARD_HEIGHT] = (0..BOARD_WIDTH * BOARD_HEIGHT)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let octis: [Option<Octi>; BOARD_WIDTH * BOARD_HEIGHT] =
            octis.map(|x| board.get_octi_by_pos(&Self::index_to_pos(x)).cloned());

        let mut arr_counts = [0; TEAMS];
        arr_counts[team_index(Team::Red)] = board.get_arr_count(&Team::Red).unwrap();
        arr_counts[team_index(Team::Green)] = board.get_arr_count(&Team::Green).unwrap();

        Board {
            turn: board.turn(),
            octis,
            arr_counts,
        }
    }

    pub fn octis(&self) -> impl Iterator<Item = &Octi> {
        self.octis.iter().filter_map(|x| x.as_ref())
    }

    // mutability functions
    fn get_octi_by_pos_mut(&mut self, pos: &Position) -> Option<&mut Octi> {
        if let Some(mut_cell) = self.octis.get_mut(Self::pos_to_index(pos)) {
            mut_cell.as_mut()
        } else {
            None
        }
    }

    fn take_octi_by_pos(&mut self, pos: &Position) -> Option<Octi> {
        if let Some(mut_cell) = self.octis.get_mut(Self::pos_to_index(pos)) {
            mut_cell.take()
        } else {
            None
        }
    }

    // assumes pos is valid
    fn insert_octi_at_pos(&mut self, pos: &Position, octi: Octi) {
        *self.octis.get_mut(Self::pos_to_index(pos)).unwrap() = Some(octi);
    }

    // associated methods
    fn index_to_pos(index: usize) -> Position {
        Position::new((index % BOARD_WIDTH) as i32, (index / BOARD_WIDTH) as i32)
    }

    fn pos_to_index(pos: &Position) -> usize {
        pos.x() as usize + pos.y() as usize * BOARD_WIDTH
    }
}

impl Boardable for Board {
    fn get_octi_by_pos(&self, pos: &Position) -> Option<&Octi> {
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            self.octis.get(Self::pos_to_index(pos))?.as_ref()
        }
    }

    fn get_octi_by_id(&self, id: &OctiID) -> Option<&Octi> {
        let id = *id;
        for octi in self
            .octis
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
        {
            if octi.id() == id {
                return Some(octi);
            }
        }
        None
    }

    fn get_arr_count(&self, team: &Team) -> Option<u32> {
        let index = team_index(*team);
        if index < TEAMS {
            Some(self.arr_counts[index])
        } else {
            None
        }
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        0 <= pos.x()
            && pos.x() < BOARD_WIDTH as i32
            && 0 <= pos.y()
            && pos.y() < BOARD_HEIGHT as i32
    }

    fn turn(&self) -> Team {
        self.turn
    }

    fn set_turn(&mut self, turn: Team) {
        self.turn = turn
    }
}

impl BoardEventProcessor for Board {
    fn process_events(&mut self, board_events: &[BoardEvent]) {
        for event in board_events {
            match event {
                BoardEvent::NewArrow(pos, arr) => {
                    let team = {
                        let octi = self.get_octi_by_pos_mut(pos).unwrap();
                        octi.add_arr(*arr);
                        octi.team()
                    };
                    *self.arr_counts.get_mut(team_index(team)).unwrap() -= 1;
                }
                BoardEvent::NewOctiPosition(pos, new_pos) => {
                    let mut octi = self.take_octi_by_pos(pos).unwrap();
                    octi.set_pos(*new_pos);
                    self.insert_octi_at_pos(new_pos, octi);
                }
                BoardEvent::OctiEaten(pos) => {
                    let octi = self.take_octi_by_pos(pos).unwrap_or_else(|| {
                        panic!("{}", pos);
                    });

                    let other_team = match octi.team() {
                        Team::Red => Team::Green,
                        Team::Green => Team::Red,
                    };

                    *self.arr_counts.get_mut(team_index(other_team)).unwrap() += octi.arr_count();
                }
                BoardEvent::Div => {}
            }
        }
    }
}
