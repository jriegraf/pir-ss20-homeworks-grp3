// Define a Hero Struct
#[derive(Debug)]
struct Hero {
    name: String,
    hitpoints: u32,
    damage: u32,
    position: (usize,usize),
}
// Let Hero spawn, attack and move
impl Hero {
    fn spawn(name:String, hitpoints:u32, damage:u32, position: (usize,usize)) -> Hero {
        Hero{
            name,
            hitpoints,
            damage,
            position,
        }
    }
    fn attack(&mut self, mut monster:Monster) -> Monster {
        if monster.hitpoints - self.damage == 0 {
            println!("great, u've beaten the monster");
            monster.hitpoints = 0;
        } else {
            monster.hitpoints = monster.hitpoints - self.damage;
            println!("you've hit the monster for {}. It now has {} hitpoints left", self.damage, monster.hitpoints);
        }
        monster
    }
    fn move_hero(&mut self, position: (usize, usize), mut current_field: Gamefield) -> Gamefield{
        if self.position == position {
            println!("You already stand here");
            current_field
        } else {
            current_field = Gamefield::update(current_field, (self.position.0, self.position.1), 0);
            self.position = position;
            Gamefield::update(current_field, (position.0,position.1), 1) 
        }
    }
}

// Define a Monster Struct
#[derive(Debug)]
struct Monster {
    name: String,
    hitpoints: u32,
    damage: u32,
    position: (usize,usize),
}
// Let Monster spawn, attack and move
impl Monster {
    fn spawn(name:String, hitpoints:u32, damage:u32, position: (usize,usize)) -> Monster {
        Monster{
            name,
            hitpoints,
            damage,
            position,
        }
    }
    //
    fn attack(&mut self, mut hero:Hero)-> Hero { 
        if hero.hitpoints - self.damage == 0 {
            hero.hitpoints = 0;
            println!("great, u've beaten the hero");
        } else {
            hero.hitpoints = hero.hitpoints - self.damage;
            println!("{} hit the hero for {}. It now has {} hitpoints left", self.name, self.damage, hero.hitpoints);
        }
        hero
    }
    fn move_monster (&mut self, position: (usize, usize), mut current_field: Gamefield) -> Gamefield{
        if self.position == position {
            println!("You already stand here");
            current_field
        } else {
            current_field = Gamefield::update(current_field, (self.position.0, self.position.1), 0);
            self.position = position;
            Gamefield::update(current_field, (position.0,position.1), 2) 
        }
    }
    // // Make monster cloneable
    fn clone() {
        
    }
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
    fn update(mut gamefield:Gamefield, (x,y):(usize, usize), mon_or_hero:u32) -> Gamefield{
        match (gamefield.grid[x][y], mon_or_hero) {
            (0, 1) | (0,2) => gamefield.grid[x][y] = mon_or_hero,
            (1, 0) => gamefield.grid[x][y] = mon_or_hero, //hero moved
            (2, 0) => gamefield.grid[x][y] = mon_or_hero, //mon moved
            (1, 1) => println!("field is already taken by a hero!"),
            (1, 2) => println!("field is already taken by a hero!"),
            (2, 1) => println!("field is already taken by a monster!"),
            (2, 2) => println!("field is already taken by a monster!"),
            _ => println!("invalid input. 1 is for heroes to move, 2 for monsters!"),
        };
        gamefield
    }
    // fn display(...) {
    //     ...
    // }
}


fn main() {
    // Create Gamefield
    let mut gamefield = Gamefield::create(8, vec![vec![0; 8]; 8]);
    println!("{:?}", gamefield);
    println!("gamefield[x][y]: {:?}", gamefield.grid[2][3]);
    // Spawn a hero
    let mut hero = Hero::spawn("Arthur".to_string(), 100, 15, (4,3));
    println!("{:#?}", hero);
    gamefield = Gamefield::update(gamefield, (4,3), 1);
    // set monster to x,y on grid -> fn set in grid
    // Spawn a Monster
    let mut monster = Monster::spawn("Zombie".to_string(), 50, 15, (2,3));
    gamefield = Gamefield::update(gamefield, (2,3), 2);
    println!("{:#?}", monster);
    println!("gamefield with monster and hero: {:?}", gamefield);


    // Let them fight, and clone a monster
    let hp_mon = monster.hitpoints/2;
    monster = hero.attack(monster);
    if monster.hitpoints <= hp_mon{
        let clone_mon = Monster::spawn("clone".to_string(), 25, 10, (8,8));
    }; 
    hero = monster.attack(hero);
    // Move hero and mosnter
    gamefield = hero.move_hero((3,3), gamefield);
    gamefield = monster.move_monster((2,2), gamefield);
    println!("gamefield[x][y]: {:?}", gamefield.grid);
    // monster_1.move_monster(...);
}