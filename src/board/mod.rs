pub mod parse;

use super::global::ARROWS_PER_OCTI;
use std::collections::{BTreeMap, HashSet};
use std::default::Default;
use std::ops;

//
// Aliases
//

pub type Direction = Position;
pub type OctiID = u32;

//
// Enums
//

#[derive(Clone, PartialEq, Eq)]
pub enum OctiMove {
    Arrow(Position, Arrow),
    Move(Position, Vec<Arrow>),
}

#[derive(Clone)]
pub enum BoardEvent {
    NewArrow(Position, Arrow),
    NewOctiPosition(Position, Position),
    OctiEaten(Position),
    Div, // optional divider signifying end of intermidiary move
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Team {
    Red,
    Green,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ArrowStatus {
    Active,
    Inactive,
}

//
// Structs
//

#[derive(Clone)]
pub struct Board {
    turn: Team,
    bounds: BoardBounds,
    octis: BTreeMap<OctiID, Octi>,
    pos_indexer: BTreeMap<Position, OctiID>,
    arr_counts: BTreeMap<Team, u32>,
    next_id: OctiID,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Octi {
    id: OctiID,
    team: Team,
    pos: Position,
    arrs: [ArrowStatus; ARROWS_PER_OCTI],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arrow(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Position(i32, i32);

#[derive(Clone, Copy)]
pub struct BoardBounds(Position, Position);

impl Board {
    pub fn bounds(&self) -> BoardBounds {
        self.bounds
    }

    fn get_octi_by_pos_mut(&mut self, pos: &Position) -> Option<&mut Octi> {
        self.octis.get_mut(self.pos_indexer.get(pos)?)
    }

    fn get_octi_by_id_mut(&mut self, id: &OctiID) -> Option<&mut Octi> {
        self.octis.get_mut(id)
    }
}

impl Boardable for Board {
    fn get_octi_by_pos(&self, pos: &Position) -> Option<&Octi> {
        self.get_octi_by_id(self.pos_indexer.get(pos)?)
    }

    fn get_octi_by_id(&self, id: &OctiID) -> Option<&Octi> {
        self.octis.get(id)
    }

    fn get_arr_count(&self, team: &Team) -> Option<u32> {
        Some(*self.arr_counts.get(team)?)
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        self.bounds.in_bounds(pos)
    }

    fn turn(&self) -> Team {
        self.turn
    }

    fn set_turn(&mut self, turn: Team) {
        self.turn = turn;
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
                    *self.arr_counts.get_mut(&team).unwrap() -= 1;
                }
                BoardEvent::NewOctiPosition(pos, new_pos) => {
                    let id = self.pos_indexer.remove(pos).unwrap();
                    self.pos_indexer.insert(*new_pos, id);

                    let octi = self.get_octi_by_id_mut(&id).unwrap();
                    octi.set_pos(*new_pos);
                }
                BoardEvent::OctiEaten(pos) => {
                    let octi_id = self.pos_indexer.remove(pos).unwrap();
                    let octi = self.octis.remove(&octi_id).unwrap();

                    let other_team = match octi.team() {
                        Team::Red => Team::Green,
                        Team::Green => Team::Red,
                    };

                    *self.arr_counts.get_mut(&other_team).unwrap() += octi.arr_count();
                }
                BoardEvent::Div => {}
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let turn = Team::Red;
        let bounds = BoardBounds::default();
        let mut pos_indexer = BTreeMap::new();
        let octis = BTreeMap::from_iter((0..8).map(|id| {
            let (pos, team) = {
                if id < 4 {
                    let x = id + 1;
                    (Position::new(x, 5), Team::Red)
                } else {
                    let x = id - 3;
                    (Position::new(x, 1), Team::Green)
                }
            };

            let id = id as OctiID;
            pos_indexer.insert(pos, id);
            (
                id,
                Octi::new(id, team, pos, [ArrowStatus::Inactive; ARROWS_PER_OCTI]),
            )
        }));
        let mut arr_counts = BTreeMap::new();
        arr_counts.insert(Team::Red, 12);
        arr_counts.insert(Team::Green, 12);
        let next_id = 8;

        Board {
            turn,
            bounds,
            octis,
            pos_indexer,
            arr_counts,
            next_id,
        }
    }
}

impl Octi {
    pub fn new(
        id: OctiID,
        team: Team,
        pos: Position,
        arrs: [ArrowStatus; ARROWS_PER_OCTI],
    ) -> Octi {
        Octi {
            id,
            team,
            pos,
            arrs,
        }
    }

    // Getters

    pub fn id(&self) -> OctiID {
        self.id
    }

    pub fn team(&self) -> Team {
        self.team
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn has_arr(&self, arr: &Arrow) -> bool {
        self.arrs[arr.value()] == ArrowStatus::Active
    }

    pub fn arr_count(&self) -> u32 {
        self.arrs.iter().fold(0, |acc, x| match x {
            ArrowStatus::Active => acc + 1,
            _ => acc,
        })
    }

    pub fn arr_iter(&self) -> impl Iterator<Item = (usize, &ArrowStatus)> {
        self.arrs.iter().enumerate()
    }

    // Setters

    pub fn add_arr(&mut self, arr: Arrow) {
        if self.has_arr(&arr) {
            panic!("Octi already has said arrow")
        }

        self.arrs[arr.value()] = ArrowStatus::Active;
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }
}

impl Arrow {
    pub fn new(value: usize) -> Result<Arrow, String> {
        if value >= ARROWS_PER_OCTI {
            Err(format!("Invalid arrow value: {}", value))
        } else {
            Ok(Arrow(value))
        }
    }

    pub fn value(&self) -> usize {
        self.0
    }

    pub fn direction(&self) -> Direction {
        match self.0 {
            0 => Direction::new(1, 0),
            1 => Direction::new(1, -1),
            2 => Direction::new(0, -1),
            3 => Direction::new(-1, -1),
            4 => Direction::new(-1, 0),
            5 => Direction::new(-1, 1),
            6 => Direction::new(0, 1),
            7 => Direction::new(1, 1),
            _ => panic!("Something went terribly wrong, arrow value exceeds max"),
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position(x, y)
    }

    // Getters

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    // Operations

    pub fn abs(mut self) -> Self {
        self.0 = self.0.abs();
        self.1 = self.1.abs();
        self
    }
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl ops::Mul<i32> for Position {
    type Output = Position;

    fn mul(self, rhs: i32) -> Self::Output {
        Position::new(self.x() * rhs, self.y() * rhs)
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}

impl BoardBounds {
    pub fn new(lu: Position, rd: Position) -> BoardBounds {
        BoardBounds(lu, rd)
    }

    pub fn lu(&self) -> Position {
        self.0
    }

    pub fn rd(&self) -> Position {
        self.1
    }

    pub fn height(&self) -> i32 {
        self.1.y() - self.0.y() + 1
    }

    pub fn width(&self) -> i32 {
        self.1.x() - self.0.x() + 1
    }

    pub fn in_bounds(&self, pos: &Position) -> bool {
        self.0.x() <= pos.x()
            && pos.x() <= self.1.x()
            && self.0.y() <= pos.y()
            && pos.y() <= self.1.y()
    }
}

impl Default for BoardBounds {
    fn default() -> Self {
        BoardBounds::new(Position::new(0, 0), Position::new(5, 6))
    }
}

//
// Traits
//

pub trait Boardable {
    // Getters

    fn get_octi_by_pos(&self, pos: &Position) -> Option<&Octi>;
    fn get_octi_by_id(&self, id: &OctiID) -> Option<&Octi>;
    fn get_arr_count(&self, team: &Team) -> Option<u32>;
    fn in_bounds(&self, pos: &Position) -> bool;
    fn turn(&self) -> Team;
    fn set_turn(&mut self, turn: Team);
}

// BoardEventProcessor process_events isn't responsible for validating events and assumes them to
// be valid
pub trait BoardEventProcessor: Boardable {
    fn is_move_valid(&self, octi_move: &OctiMove) -> bool {
        self.move_events(octi_move).is_ok()
    }

    // supposed to be chronologically ordered
    fn move_events(&self, octi_move: &OctiMove) -> Result<Vec<BoardEvent>, String> {
        match octi_move {
            OctiMove::Arrow(pos, arr) => {
                let octi = self.get_octi_by_pos(pos);
                if octi.is_none() {
                    Err(format!("Octi at position {:?} does not exist", pos))?;
                }

                let octi = octi.unwrap();
                let team = octi.team();
                let arr_count = self.get_arr_count(&team);

                if arr_count.is_none() {
                    Err(format!("Team: {:?} does not implement arrows", team))?;
                }

                let arr_count = arr_count.unwrap();
                let turn = self.turn();

                if arr_count == 0 {
                    Err(format!("Team {:?} has 0 arrows", team))?;
                }
                if team != turn {
                    Err(format!("Expected team: {:?}, Got: {:?}", turn, team))?;
                }
                if octi.has_arr(arr) {
                    Err(format!("Arrow {:?} already exists on {:?}", arr, pos))?;
                }

                Ok(vec![BoardEvent::NewArrow(*pos, *arr)])
            }
            OctiMove::Move(pos, arrs) => {
                let pos = *pos;

                let octi = self.get_octi_by_pos(&pos);
                if octi.is_none() {
                    Err(format!("Octi at position {:?} does not exist", pos))?;
                }
                let octi = octi.unwrap();
                let team = octi.team();
                let turn = self.turn();

                if team != turn {
                    Err(format!("Expected team: {:?}, Got: {:?}", turn, team))?;
                }

                let mut next_pos = pos;

                if arrs.len() == 1 {
                    let arr = arrs[0];
                    if !octi.has_arr(&arr) {
                        Err(format!("Arrow {:?} does not exist on {:?}", arr, pos))?;
                    }

                    let direction = arrs[0].direction();
                    let consider_position = next_pos + direction;
                    let consider_position_octi = self.get_octi_by_pos(&consider_position);

                    if self.in_bounds(&consider_position) && consider_position_octi.is_none() {
                        return Ok(vec![BoardEvent::NewOctiPosition(pos, consider_position)]);
                    }
                }

                // *3 in case every jump is eat + for every div
                let mut board_events = Vec::with_capacity(arrs.len() * 3);
                let mut eaten_octis = HashSet::new();

                for arr in arrs {
                    if !octi.has_arr(arr) {
                        Err(format!("Arrow {:?} does not exist on {:?}", arr, pos))?;
                    }

                    let direction = arr.direction();

                    let previous_pos = next_pos;
                    let in_between_pos = next_pos + direction;
                    next_pos = next_pos + direction * 2;

                    if !self.in_bounds(&next_pos) {
                        Err(format!("Positions not in bounds: {:?}", next_pos))?;
                    }

                    if let Some(in_between_octi) = self.get_octi_by_pos(&in_between_pos) {
                        if in_between_octi.team() != team && !eaten_octis.contains(&in_between_pos)
                        {
                            board_events.push(BoardEvent::OctiEaten(in_between_pos));
                            eaten_octis.insert(in_between_pos);
                        }
                    } else {
                        Err(format!("No in-between octi at: {:?}", in_between_pos))?;
                    }

                    let next_pos_octi = self.get_octi_by_pos(&next_pos);
                    if next_pos_octi.is_some() && next_pos != pos {
                        Err(format!(
                            "Cannot jump because there is octi at: {:?}",
                            next_pos
                        ))?;
                    }

                    board_events.push(BoardEvent::NewOctiPosition(previous_pos, next_pos));
                    board_events.push(BoardEvent::Div);
                }

                Ok(board_events)
            }
        }
    }

    fn process_events(&mut self, board_events: &[BoardEvent]);

    fn make_move(&mut self, octi_move: &OctiMove) -> Result<(), String> {
        let board_events = self.move_events(octi_move)?;
        self.process_events(&board_events);
        self.set_turn(match self.turn() {
            Team::Red => Team::Green,
            Team::Green => Team::Red,
        });
        Ok(())
    }
}
