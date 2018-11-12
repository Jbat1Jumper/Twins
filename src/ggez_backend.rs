
/// This is the real game implementation, 
///

extern crate nalgebra;

use engine::logical::{Logical, AsLogical, LogicUpdater};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

mod engine;


struct World {
    content: HashMap<Id, Object>,
}

type Id = u32;

impl Index<Id> for World {
    type Output = Object;

    fn index(&self, id: Id) -> &Object {
        self.content.get(&id).unwrap()
    }
}

impl IndexMut<Id> for World {
    fn index_mut(&mut self, id: Id) -> &mut Object {
        self.content.get_mut(&id).unwrap()
    }
}



#[derive(Clone)]
enum Object {
    Player(Player),
    Tree(Tree),
    Monster(Monster),
    Dead,
}

impl AsLogical<World> for Object {
    fn as_logical_mut<'a>(&'a mut self) -> Option<&'a mut Logical<World>> {
        match self {
            Object::Player(p) => Some(p),
            _ => None,
        }
    }
}


#[derive(Clone)]
struct Player {
    lives: u32,
}

impl Logical<World> for Player {
    fn update(&mut self, w: &mut World) {
        self.lives -= 1;
        if self.lives > 0 {
            println!("I'm the player and I can think for myself!");
        } else {
            println!("Ouch, not anymore!");
        }
    }
}

impl World {
    fn new() -> Self {
        let mut content = HashMap::new();
        content.insert(0, Object::Player(Player { lives: 3, }));
        content.insert(1, Object::Tree(Tree));
        content.insert(2, Object::Tree(Tree));
        content.insert(3, Object::Tree(Tree));
        content.insert(4, Object::Tree(Tree));
        content.insert(5, Object::Monster(Monster));
        Self { content, }
    }
}

#[derive(Clone)]
struct Tree;

#[derive(Clone)]
struct Monster;



fn main() {
    let mut w = World::new();

    let mut is_alive = true;
    while is_alive {
        let ids: Vec<Id> = w.content.keys().cloned().collect();
        for id in ids {
            LogicUpdater::update(id, &mut w);
        }
        if let Object::Player(ref p) = w[0] {
            is_alive = p.lives > 0;
        }
    }
}
