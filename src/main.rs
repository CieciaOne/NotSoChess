use std::fmt;

fn main() {
    println!("Hello, world!");
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

#[derive(Debug, PartialEq, Eq)]
enum State {
    Alive,
    Dead,
    Moved,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    /// Translate position to fit into opposing side of map.
    pub fn reverse(&self) -> Position {
        Position {
            x: 15 - self.x,
            y: 15 - self.x,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub struct Entity {
    id: u8,
    state: State,
    figure: Figure,
    position: Position,
    points: u8,
}

impl Entity {
    fn new(id: u8, figure: Figure, position: Position) -> Self {
        let base: u8 = 16;
        let points = match figure.fig_type() {
            FigureType::Normal => 1,
            FigureType::Special => 2,
        };
        Self {
            id,
            state: State::Alive,
            figure,
            position,
            points: points * base,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    positions: [[Option<Entity>; 16]; 16],
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
        println!(
            "{:?},{},{}",
            self.positions[position.x() as usize][position.y() as usize],
            position.x(),
            position.y()
        );
        if self.positions[position.x() as usize][position.y() as usize] != None {
            true
        } else {
            false
        }
    }

    /// Initializes a map with players' antities
    pub fn init(p0: Player, p1: Player) {
        let mut arena = Map::empty();
        let l = arena.positions().len();

        for figure in p0.set().iter() {
            if arena.is_taken(figure.start_position()) {
                println!("Taken");
            } else {
                println!("Spawning");
                arena.spawn(figure.to_owned(), false);
            }
        }

        for figure in p1.set().iter() {
            let temp_pos = figure.start_position.reverse();
            if arena.is_taken(temp_pos) {
                println!("Taken");
            } else {
                println!("Spawning");
                arena.spawn(figure.to_owned(), true);
            }
        }
        println!("{:#?}",arena);
    }

    /// Assign Figure to the map's position.
    fn spawn(&mut self, figure: Figure, reverse: bool) {
        if reverse {
            //println!("REVERSE: {:?}",figure.start_position().reverse());
            self.positions[figure.start_position().reverse().x() as usize]
                [figure.start_position().reverse().y() as usize] = Some(Entity::new(
                0,
                figure.clone(),
                figure.start_position().reverse(),
            ));
        } else {
            self.positions[figure.start_position().x() as usize]
                [figure.start_position().y() as usize] =
                Some(Entity::new(0, figure.clone(), figure.start_position()));
        }
    }

    /// Get a reference to the map's positions.
    pub fn positions(&self) -> &[[Option<Entity>; 16]; 16] {
        &self.positions
    }
}
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    set: [Figure; 8],
}

impl Player {
    fn new(name: String, set: [Figure; 8]) -> Self {
        Self { name, set }
    }

    pub fn fig(&self, index: usize) -> &Figure {
        &self.set[index]
    }

    /// Get a reference to the player's set.
    pub fn set(&self) -> &[Figure; 8] {
        &self.set
    }

    /// Set the player's set.
    pub fn set_set(&mut self, set: [Figure; 8]) {
        self.set = set;
    }

    /// Get a reference to the player's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_map() {
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
        let set = [
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
            pawn.clone(),
        ];
        let p1 = Player::new("name".to_string(), set);
        let mut arena = Map::init(p1.clone(), p1);
        let a = Map::empty();
        // println!("{} {:?}",a.is_taken(Position { x: 0, y:0 }),a);



        // println!("{:#?}", arena);
        //println!("{}", map);
    }
}
