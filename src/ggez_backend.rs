
/// This is the real game implementation, 
///

extern crate nalgebra;
extern crate petgraph;
extern crate ggez;

use engine::logical::Update;
use engine::graphics::{Draw, Graphics, DrawPrimitives, DrawMode, Color};
use engine::sequence::Sequence;
use petgraph::graph::{Graph, NodeIndex};
use std::ops::{Index, IndexMut};
use nalgebra::*;

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
}

impl Update<World> for Node {
    fn update(&mut self, w: &mut World) {
        match self {
            Node::Player(p) => p.update(w),
            _ => {},
        }
    }
}

impl<S> Draw<S> for Node
where
    S: DrawPrimitives,
{
    fn draw(&self, surface: &mut S) {
        match self {
            Node::Player(p) => p.draw(surface),
            _ => {},
        }
    }
}


#[derive(Clone)]
struct Player {
    position: Point2<f32>,
    sequence: Sequence,
}

impl Player {
    fn new() -> Self {
        Self {
            position: Point2::origin(),
            sequence: Sequence::new(),
        }
    }
}

impl Update<World> for Player {
    fn update(&mut self, w: &mut World) {
        self.sequence.step(&mut (), w)
            .then(|_, _| { println!("Hi"); })
            .wait(500)
            .then(|_, _| { println!("Hi"); })
            .wait(1000)
            .then(|_, _| { println!("Hi"); });
    }
}

#[derive(Clone, Copy)]
enum Palette {
    White,
    Black,
}

impl Color for Palette {
    fn into_rgba(self) -> [f32; 4] {
        match self {
            Palette::White => [1.0, 1.0, 1.0, 1.0],
            Palette::Black => [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl<S> Draw<S> for Player
where 
    S: DrawPrimitives,
{
    fn draw(&self, surface: &mut S) {
        surface.set_color(Palette::White);
        surface.circle(DrawMode::Fill, Point::origin(), 100.0);
    }
}

impl World {
    fn new() -> Self {
        let mut content = Graph::new();
        let player_id = content.add_node(Node::Player(Player::new()));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Tree(Tree));
        content.add_node(Node::Monster(Monster));
        Self { content, player_id }
    }
}

impl<S> Draw<S> for World
where 
    S: DrawPrimitives,
{
    fn draw(&self, surface: &mut S) {
        let ids: Vec<NodeIndex> = self.content.node_indices().collect();
        surface.clear(Palette::Black);
        for id in ids {
            self.content[id].draw(surface);
        }
        surface.present();
    }
}

impl<B> Update<B> for World
where
    B: Sized,  // Here we can specify backend capabilities
{
    fn update(&mut self, backend: &mut B) {

        let mut is_alive = true;

        let ids: Vec<NodeIndex> = self.content.node_indices().collect();
        for id in ids {
            let mut o = self.content[id].clone();
            o.update(self);
            self.content[id] = o;
        }
    }
}

#[derive(Clone)]
struct Tree;

#[derive(Clone)]
struct Monster;



/////


/////



fn main() {
    let game = Game::new(320, 240).run(World::new());
}
