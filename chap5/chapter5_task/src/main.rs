// Define a Hero Struct
#[derive(Debug)]
struct Hero {
    name: String,
    hitpoints: u32,
    damage: u32,
    position: Vec<Vec<u32>>,
}
// Let Hero spawn, attack and move
impl Hero {
    // fn spawn(...) -> ... {
    //
    // }
    // fn attack(...) {
    //     ...
    // }
    // fn move_hero(...) {
    //     ...
    // }
}

// Define a Monster Struct
#[derive(Debug)]
struct Monster {
    name: String,
    hitpoints: u32,
    damage: u32,
    position: (u32,u32),
}
// Let Monster spawn, attack and move
impl Monster {
    fn spawn(name:String, hitpoints:u32, damage:u32, position: (u32,u32)) -> Monster {
        Monster{
            name,
            hitpoints,
            damage,
            position,
        }
    }
    //
    // fn attack(...) {
    //     ...
    // }
    // fn move_monster(...) {
    //    ...
    // }
    // // Make monster cloneable
    // fn clone(...) {
    //     ...
    // }
}
#[derive(Debug)]
// #[derive(fmt::Display)]
// #[fmt::Display::fmt(fmt = "{}", grid)]

struct Gamefield {
    size: u32,
    grid: Vec<Vec<u32>>,
}

impl Gamefield {
    fn create(size: u32, grid: Vec<Vec<u32>>) -> Gamefield {
       Gamefield{
           size,
           grid,
       }
    }
    // fn display(...) {
    //     ...
    // }
}


fn main() {
    // Create Gamefield
    let mut gamefield = Gamefield::create(8, vec![vec![0;8]; 8]);
    println!("{:#?}", gamefield);
    // Spawn a hero
    // let mut hero = ...

    // Spawn a Monster
    let mut monster = Monster::spawn("Zombie".to_string(), 50, 15, (2,3));
    gamefield.grid[2][3] = 1;
    println!("{:#?}", gamefield);
    println!("{:#?}", monster);
    // Let them fight, and clone a monster
    // hero.attack(...);
    // monster_1.attack(...);
    // Move hero and mosnter
    // hero.move_hero(...);
    // monster_1.move_monster(...);
}