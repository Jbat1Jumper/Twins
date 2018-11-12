
/// This is the real game implementation, 
///

extern crate nalgebra;
extern crate petgraph;

use engine::logical::{Logical, LogicUpdater};
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

mod engine;


struct World {
    content: Graph<Node, Edge>,
    player_id: NodeIndex,
}

impl Index<NodeIndex> for World {
    type Output = Node;

    fn index(&self, id: NodeIndex) -> &Node {
        &self.content[id]
    }
}

impl IndexMut<NodeIndex> for World {
    fn index_mut(&mut self, id: NodeIndex) -> &mut Node {
        &mut self.content[id]
    }
}

#[derive(Clone)]
struct Edge;

#[derive(Clone)]
enum Node {
    Player(Player),
    Tree(Tree),
    Monster(Monster),
    Dead,
}

impl Logical<World> for Node {
    fn update(&mut self, w: &mut World) {
        match self {
            Node::Player(p) => p.update(w),
            _ => {},
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
        let mut content = Graph::new();
        let player_id = content.add_node(Node::Player(Player { lives: 3, }));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Monster(Monster));
        Self { content, player_id }
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
        let ids: Vec<NodeIndex> = w.content.node_indices().collect();
        for id in ids {
            LogicUpdater::update(id, &mut w);
        }
        if let Node::Player(ref p) = w[w.player_id] {
            is_alive = p.lives > 0;
        }
    }
}
