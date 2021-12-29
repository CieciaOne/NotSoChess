use std::fmt::{self};
use std::io::stdin;
use std::num::ParseIntError;
use std::ops::Sub;
use std::str::FromStr;

fn main() {
    // println!("Hello, world!");
    let pawn = Figure::new(
        "Pawn".to_string(),
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 1, 1, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        Position { x: 0, y: 0 },
        crate::FigureType::Normal,
    );

    let king = Figure::new(
        "King".to_string(),
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        Position { x: 1, y: 2 },
        crate::FigureType::Special,
    );

    let queen = Figure::new(
        "Queen".to_string(),
        [
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 0, 1, 1, 1, 1],
            [1, 1, 1, 0, 0, 0, 1, 1, 1],
            [1, 1, 1, 1, 0, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1],
        ],
        Position { x: 2, y: 1 },
        crate::FigureType::Special,
    );
    let bishop = Figure::new(
        "Bishop".to_string(),
        [
            [1, 0, 0, 0, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 1, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 1],
        ],
        Position { x: 3, y: 3 },
        crate::FigureType::Special,
    );

    let set = [
        queen,
        king,
        pawn.clone().starting_at(Position { x: 2, y: 2 }),
        pawn.clone().starting_at(Position { x: 1, y: 0 }),
        pawn.clone().starting_at(Position { x: 0, y: 1 }),
        pawn.clone().starting_at(Position { x: 0, y: 2 }),
        pawn.clone().starting_at(Position { x: 2, y: 0 }),
        pawn,
    ];

    let p1 = Player::new("Ame".to_string(), 1, set.clone());
    let p2 = Player::new("Gura".to_string(), 2, set);
    let mut session = Session::new([p1, p2]);

    session.show();
    session.calculate_round(1);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FigureType {
    Normal,
    Special,
}

impl fmt::Display for FigureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Figure {
    name: String,
    pattern: [[u8; 9]; 9],
    start_position: Position,
    fig_type: FigureType,
}

impl Figure {
    pub fn new(
        name: String,
        pattern: [[u8; 9]; 9],
        start_position: Position,
        fig_type: FigureType,
    ) -> Self {
        Self {
            name,
            pattern,
            start_position,
            fig_type,
        }
    }

    /// Get Figure with updated position
    pub fn starting_at(&mut self, position: Position) -> Self {
        self.start_position = position;
        self.to_owned()
    }

    /// Get a reference to the figure's fig_type.
    pub fn fig_type(&self) -> &FigureType {
        &self.fig_type
    }

    /// Get a reference to the figure's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the figure's start_position.
    pub fn start_position(&self) -> Position {
        self.start_position
    }

    /// Get a reference to the figure's pattern.
    pub fn pattern(&self) -> [[u8; 9]; 9] {
        self.pattern
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    x: u8,
    y: u8,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PositionError {
    ParseError(ParseIntError),
    IndexError,
}
impl From<ParseIntError> for PositionError {
    fn from(error: ParseIntError) -> Self {
        PositionError::ParseError(error)
    }
}

impl FromStr for Position {
    type Err = PositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')' || p == '\n')
            .trim()
            .split(' ')
            .collect();
        if coords.len() == 2 {
            let x_fromstr = coords[0].parse::<u8>()?;
            let y_fromstr = coords[1].parse::<u8>()?;

            Ok(Position {
                x: x_fromstr,
                y: y_fromstr,
            })
        } else {
            Err(PositionError::IndexError)
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Translate position to fit into opposing side of map.
    pub fn reverse(&self) -> Position {
        Position {
            x: 15 - self.x,
            y: 15 - self.y,
        }
    }

    fn is_valid(&self) -> bool {
        let range = 0..16;
        range.contains(&self.x) && range.contains(&self.y)
    }

    /// Get a reference to the position's x.
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Set the position's x.
    pub fn set_x(&mut self, x: u8) {
        self.x = x;
    }

    /// Get a reference to the position's y.
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Set the position's y.
    pub fn set_y(&mut self, y: u8) {
        self.y = y;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entity {
    id: u8,
    owner_id: u128,
    state: State,
    figure: Figure,
    position: Position,
    points: u8,
}
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}: {}\n* {}\n",
            self.figure.name, self.position, self.points
        )
    }
}

impl Entity {
    fn new(id: u8, owner_id: u128, figure: Figure, position: Position) -> Self {
        let base: u8 = 16;
        let points = match figure.fig_type() {
            FigureType::Normal => 1,
            FigureType::Special => 2,
        };
        Self {
            id,
            owner_id,
            state: State::Alive,
            figure,
            position,
            points: points * base,
        }
    }

    fn deal_dmg(&self, e: &mut Entity) {
        if e.points as i8 - self.points as i8 >= 0 {
            e.points -= self.points;
        } else {
            e.points = 0;
        }
    }

    pub fn get_symbol(&self) -> String {
        let s = self.figure.name().chars().next().unwrap().to_string();
        match self.figure.fig_type() {
            FigureType::Normal => s.to_lowercase(),
            FigureType::Special => s.to_uppercase(),
        }
    }

    pub fn can_reach(&self, target: Position) -> bool {
        let mvx: i8 = target.x as i8 - self.position.x as i8;
        let mvy: i8 = target.y as i8 - self.position.y as i8;
        let origin = Position { x: 4, y: 4 };
        let range = 0..9;
        if origin.x as i8 >= mvx
            && origin.y as i8 >= mvy
            && range.contains(&(origin.x as i8 - mvx))
            && range.contains(&(origin.y as i8 - mvy))
        {
            self.figure.pattern[(origin.x as i8 - mvx) as usize][(origin.y as i8 - mvy) as usize] // panicking if out of range, mv more than indexing??!?!
                == 1
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    // Consider changing it from 2d array to 1d
    positions: [[Option<Entity>; 16]; 16],
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = "\n==================================\n".to_string();
        let mut temp = self.clone();
        temp.rotate90();
        let mut rows = [
            "F", "E", "D", "C", "B", "A", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0",
        ]
        .iter();
        for row in temp.positions.iter() {
            let mut formatted_row: String = "".to_string();
            formatted_row.push_str(format!("{} ", rows.next().unwrap()).as_str());
            for index in row {
                let mut symbol = match index.to_owned() {
                    None => "_".to_string(),
                    Some(e) => e.get_symbol().to_string(),
                };
                symbol.push(' ');
                formatted_row.push_str(symbol.as_str());
            }
            formatted_row.push('\n');
            map.push_str(formatted_row.as_str());
        }
        map.push_str("  0 1 2 3 4 5 6 7 8 9 A B C D E F\n=================================");
        //let out = "a";
        write!(f, "{}", map)
    }
}

impl Map {
    pub fn empty() -> Self {
        const _INIT0: Option<Entity> = None;
        const _INIT1: [Option<Entity>; 16] = [_INIT0; 16];
        Self {
            positions: [_INIT1; 16],
        }
    }

    fn is_taken(&self, position: Position) -> bool {
        self.positions[position.x() as usize][position.y() as usize] != None
    }

    fn transpose(&mut self) {
        for m in 0..16 - 2 {
            for n in m + 1..16 - 1 {
                let temp = self.positions[n][m].clone();
                self.positions[n][m] = self.positions[m][n].clone();
                self.positions[m][n] = temp;
            }
        }
    }

    fn reverse_rows(&mut self) {
        self.positions.reverse();
    }

    fn rotate90(&mut self) {
        self.transpose();
        self.reverse_rows();
    }
    // TODO please rewrite this monster
    fn update_points(&mut self) {
        for i in 0..16 {
            for j in 0..16 {
                let e = match self.positions[i][j].to_owned() {
                    None => continue,
                    Some(e) => e,
                };
                let pattern = e.figure.pattern;
                let origin = Position { x: 4, y: 4 };
                for m in -4..5 {
                    for n in -4..5 {
                        let pattern_x = origin.x as i32 + m;
                        let pattern_y = origin.y as i32 + n;
                        if pattern[pattern_x as usize][pattern_y as usize] == 1 {
                            let absolute_target = Position {
                                x: (i as i32 + (pattern_x - origin.x as i32)) as u8,
                                y: { j as i32 + (pattern_y - origin.y as i32) } as u8,
                            };
                            if absolute_target.is_valid()
                                && e.can_reach(absolute_target)
                                && self.is_taken(absolute_target)
                            {
                                let mut target = self.positions[absolute_target.x as usize]
                                    [absolute_target.y as usize]
                                    .to_owned()
                                    .unwrap();
                                // ADD CONDITION TO OMIT TEAM ENTITIES!!!
                                e.deal_dmg(&mut target)
                            }
                        }
                    }
                }
            }
        }
    }

    /// Initializes a map with players' entities
    pub fn init(p0: Player, p1: Player) -> Self {
        let mut arena = Map::empty();
        for figure in p0.clone().set().iter() {
            if arena.is_taken(figure.start_position()) {
                println!("Taken");
            } else {
                //println!("Spawning");
                arena.spawn(&p0, figure.to_owned(), false);
            }
        }
        for figure in p1.clone().set().iter() {
            let temp_pos = figure.start_position.reverse();
            if arena.is_taken(temp_pos) {
                // println!("Taken");
                // mechanism to deal with attempts of overwrites
                continue;
            } else {
                // println!("Spawning");
                arena.spawn(&p1, figure.to_owned(), true);
            }
        }
        arena
    }

    /// Assign Figure to the map's position.
    fn spawn(&mut self, player: &Player, figure: Figure, reverse: bool) {
        if reverse {
            self.positions[figure.start_position().reverse().x() as usize]
                [figure.start_position().reverse().y() as usize] = Some(Entity::new(
                0,
                player.id,
                figure.clone(),
                figure.start_position().reverse(),
            ));
        } else {
            self.positions[figure.start_position().x() as usize]
                [figure.start_position().y() as usize] = Some(Entity::new(
                0,
                player.id,
                figure.clone(),
                figure.start_position(),
            ));
        }
    }
    // todo return result of OP
    pub fn move_entity(&mut self, current: Position, target: Position) {
        let mut entity = self.positions[current.x as usize][current.y as usize]
            .clone()
            .unwrap();
        if self.positions[target.x as usize][target.y as usize] == None
            && entity.can_reach(target)
            && target.is_valid()
        {
            self.positions[entity.position.x as usize][entity.position.y as usize] = None;
            entity.position = target;
            self.positions[target.x as usize][target.y as usize] = Some(entity);
        }
    }
}
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    id: u128,
    set: [Figure; 8],
}

impl Player {
    fn new(name: String, id: u128, set: [Figure; 8]) -> Self {
        Self { name, id, set }
    }

    fn fig(&self, index: usize) -> &Figure {
        &self.set[index]
    }

    /// Get a reference to the player's set.
    fn set(&self) -> &[Figure; 8] {
        &self.set
    }

    /// Set the player's set.
    fn set_set(&mut self, set: [Figure; 8]) {
        self.set = set;
    }

    /// Get a reference to the player's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
#[derive(Debug, Clone)]
pub enum SessionResult {
    Finished(Box<Player>),
    Active,
    Suspended,
}
#[derive(Debug, Clone)]
pub struct Session {
    id: u128,
    players: [Player; 2],
    record: Vec<Map>,
    result: SessionResult,
}

impl Session {
    pub fn new(players: [Player; 2]) -> Self {
        Self {
            id: 1, // thats a temp dont worry
            players: players.clone(),
            record: vec![Map::init(players[0].clone(), players[1].clone())],
            result: SessionResult::Active,
        }
    }
    // Not working properly rn, logic is wrong
    fn calculate_round(&mut self, mov_num: usize) {
        let p1 = self.players[0].clone();
        let p2 = self.players[1].clone();
        let m1 = self.record.last().unwrap().clone();
        let m2 = self.record.last().unwrap().clone();
        let mut maps = [m1, m2];
        for (i, player) in [p1, p2].iter().enumerate() {
            // let mut moves: Vec<Option<(Entity, Position)>> = vec![];

            let mut moves: [Option<(Entity, Position)>; 3] = [None, None, None];

            let mut position_buffer = String::new();
            let mut target_buffer = String::new();

            for i in 0..mov_num {
                println!("P: {}; Move #{}", player.name(), &i + 1);

                println!("Position: ");
                stdin().read_line(&mut position_buffer).unwrap();

                println!("Move: {}", &position_buffer);
                println!("Target: ");

                stdin().read_line(&mut target_buffer).unwrap();
                println!("To: {}", &target_buffer);

                position_buffer.pop();
                let position = Position::from_str(&position_buffer);
                let target = Position::from_str(&target_buffer);

                let mov = match (position, target) {
                    (Ok(p), Ok(t)) => match self.get_entity_by_pos(p) {
                        Some(e) => Some((e, t)),
                        None => {
                            println!("Illegal command, you are losing this move {:?}, {:?}", p, t);
                            None
                        }
                    },
                    _ => {
                        println!("Illegal command, you are losing this move");
                        None
                    }
                };
                //clear buffers before using for next move
                position_buffer.clear();
                target_buffer.clear();
                moves[i] = mov;
                println!("--------");
            }
            //println!("{:?}", &moves);

            maps[i] = self.submit_moves(moves);
        }
        let new_map = self.resolve_maps(&maps[0], &maps[1]);
        self.record.push(new_map);
        self.show()
    }

    pub fn show(&self) {
        let map = self.record.last().unwrap();
        println!("{}", map);
    }

    pub fn resolve_maps(&self, m1: &Map, m2: &Map) -> Map {
        // m1.positions;
        let map1 = m1;
        let m0 = self.record.last().unwrap();

        let mut map2 = m2;
        // map2.rotate90();
        // map2.rotate90();
        let mut resolved_map = m0.clone();
        for i in 0..16 {
            for j in 0..16 {
                let (e0, e1, e2) = (
                    m0.positions[i][j].clone(),
                    map1.positions[i][j].clone(),
                    map2.positions[i][j].clone(),
                );

                if e0 == None && e1 == None && e2 == None {
                    resolved_map.positions[i][j] = None;
                } else if e0 != None && e1 == None && e2 != None {
                    resolved_map.positions[i][j] = e1;
                } else if e0 != None && e1 != None && e2 == None {
                    resolved_map.positions[i][j] = e2;
                } else if e0 == None && e1 == None && e2 != None {
                    resolved_map.positions[i][j] = e2;
                } else if e0 == None && e1 != None && e2 == None {
                    resolved_map.positions[i][j] = e1;
                } else if e1 != None && e2 != None && e1 != e2 {
                    let mut e1 = e1.unwrap();
                    // println!("{:#?},{:#?}", &e1, &e2);
                    let mut e2 = e2.unwrap();
                    println!("Bitka pany! {} {}", &e1, &e2);
                    if e1.points == e2.points {
                        println!("Points are equal. Backing up.")
                    } else if e1.points > e2.points {
                        e2.deal_dmg(&mut e1);
                        e2.state = State::Dead;
                        resolved_map.positions[i][j] = Some(e1);
                    } else if e1.points < e2.points {
                        e1.deal_dmg(&mut e2);
                        e1.state = State::Dead;
                        resolved_map.positions[i][j] = Some(e2);
                    }
                } else {
                    continue;
                }
            }
        }
        resolved_map
    }
    fn get_entity_by_id(&self, id: u8) -> Option<Entity> {
        let mut res: Option<Entity> = None;
        for row in self.get_last_map().positions {
            for item in row {
                if item != None && item.clone().unwrap().id == id {
                    res = item
                }
            }
        }
        res
    }
    fn get_entity_by_pos(&self, position: Position) -> Option<Entity> {
        if position.is_valid() {
            match self.get_last_map().positions[position.x as usize][position.y as usize].to_owned()
            {
                Some(e) => Some(e),
                None => {
                    print!("ree");
                    None
                }
            }
        } else {
            print!("ee");
            None
        }
    }
    fn get_last_map(&self) -> Map {
        self.record.last().unwrap().to_owned()
    }
    pub fn submit_moves(&mut self, moves: [Option<(Entity, Position)>; 3]) -> Map {
        let mut map = self.record.last().unwrap().to_owned();
        for mov in moves {
            match mov {
                Some((e, t)) => {
                    map.move_entity(e.position, t);
                }
                None => {
                    continue;
                }
            };
        }
        map
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_fr_st_pos() {
        assert_eq!(Position { x: 1, y: 6 }, Position::from_str("1 6").unwrap());
    }
    #[test]
    fn test_fr_st_pos1() {
        assert_eq!(Position { x: 1, y: 6 }, Position::from_str("1 6 ").unwrap());
    }
    #[test]
    fn test_fr_st_pos2() {
        assert_eq!(Position { x: 1, y: 6 }, Position::from_str(" 1 6").unwrap());
    }
    #[test]
    fn test_fr_st_pos3() {
        assert_eq!(
            Position { x: 1, y: 6 },
            Position::from_str(" 1 6 ").unwrap()
        );
    }
    #[test]
    fn test_fr_st_pos4() {
        assert_eq!(
            Position { x: 1, y: 6 },
            Position::from_str(" 1 6\n").unwrap()
        );
    }
    #[test]
    fn test_fr_st_pos5() {
        assert!(Position::from_str("16").is_err());
    }
}
