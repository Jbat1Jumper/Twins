


/// Generic context implementation, here are the abstractions upon which the blocks are implemented

trait Context {
    type Id: Clone + Copy;
    type Object;
}

trait GetObject
where
    Self: Context,
{
    fn get(&self, Self::Id) -> Option<Self::Object>;
}

trait SetObject
where
    Self: Context,
{
    fn set(&mut self, Self::Id, Self::Object);
}



/// This is a block for a 'logic' updater which defines how an entity should be to run some logic,
/// and also what is needed from the context to update it

trait Logical<C> {
    fn update(&mut self, &mut C);
}

trait AsLogical<C> {
    fn as_logical_mut<'a>(&'a mut self) -> Option<&'a mut Logical<C>>;
}

struct LogicUpdater;

impl LogicUpdater {
    fn update<C>(id: C::Id, ctx: &mut C)
    where
        C: Context + GetObject + SetObject,
        C::Object: AsLogical<C>,
    {
        if let Some(mut o) = ctx.get(id) {
            if let Some(l) = o.as_logical_mut() {
                l.update(ctx);
            }
            ctx.set(id, o);
        }
    }
}



/// This is the real game implementation, 

use std::collections::HashMap;

type World = HashMap<Id, Object>;
type Id = u32;

#[derive(Clone)]
enum Object {
    Player(Player),
    Tree(Tree),
    Monster(Monster),
}

#[derive(Clone)]
struct Player {
    lives: u32,
}
#[derive(Clone)]
struct Tree;
#[derive(Clone)]
struct Monster;



impl Context for World {
    type Id = Id;
    type Object = Object;
}

impl GetObject for World {
    fn get(&self, id: Self::Id) -> Option<Self::Object> {
        self.get(&id).map(|o| (*o).clone())
    }
}

impl SetObject for World {
    fn set(&mut self, id: Self::Id, object: Self::Object) {
        self.insert(id, object);
    }
}



impl AsLogical<World> for Object {
    fn as_logical_mut<'a>(&'a mut self) -> Option<&'a mut Logical<World>> {
        match self {
            Object::Player(p) => Some(p),
            _ => None,
        }
    }
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



fn main() {
    let mut w = World::new();
    w.insert(0, Object::Player(Player { lives: 3, }));
    w.insert(1, Object::Tree(Tree));
    w.insert(2, Object::Tree(Tree));
    w.insert(3, Object::Tree(Tree));
    w.insert(4, Object::Tree(Tree));
    w.insert(5, Object::Monster(Monster));

    while true {
        let ids: Vec<Id> = w.keys().cloned().collect();
        for id in ids {
            LogicUpdater::update(id, &mut w);
        }
    }
}
