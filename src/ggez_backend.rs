/// This is an implementation of a force directed graph with abstract nodes
///
/// 

extern crate rand;
extern crate mursten;
extern crate mursten_ggez_backend;
extern crate nalgebra;
extern crate petgraph;
extern crate ggez;

use mursten::logic::{Update, ElapsedDelta};
use mursten::sequence::Sequence;
use mursten::random::Seed;
use mursten::input::JoystickProvider;
use petgraph::stable_graph::{StableGraph, NodeIndex};
use std::ops::{Index, IndexMut};
use nalgebra::*;


struct World {
    content: StableGraph<Node, Edge>,
    player_id: NodeIndex,
    spawn_cooldown: f32,
    seed: Seed,
    speed: f32,
    delta: f32,
}

const STEEP : f32 = 0.3;

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

impl ElapsedDelta for World {
    fn delta(&self) -> f32 {
        self.delta
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

trait CommonProperties {
    fn coordinates<'a>(&'a self) -> &'a Point2<f32>;
}

impl CommonProperties for Node {
    fn coordinates<'a>(&'a self) -> &'a Point2<f32> {
        match self {
            Node::Player(p) => &p.position,
            Node::Tree(t) => &t.position,
            Node::Monster(m) => &m.position,
        }
    }
}

#[derive(Clone)]
struct Tree {
    pub position: Point2<f32>,
    pub seed: Seed,
}

impl Tree {
    pub fn new(position: Point2<f32>) -> Self {
        Tree { position, seed: Seed::random() }
    }
}


#[derive(Clone)]
struct Monster {
    pub position: Point2<f32>,
}

impl Monster {
    pub fn new(position: Point2<f32>) -> Self {
        Monster { position }
    }
}


#[derive(Clone)]
struct Player {
    position: Point2<f32>,
    direction: Vector2<f32>,
}

impl Player {
    fn new() -> Self {
        Self {
            position: Point2::new(30.0, 100.0),
            direction: Rotation2::new(STEEP) * Vector2::x(),
        }
    }
}



mod update {
    use super::*;
    use mursten::logic::Update;

    
    impl World {
        fn clear_trees(&mut self) {
            let ids: Vec<NodeIndex> = self.content.node_indices().collect();
            for id in ids {
                let o = self.content[id].clone();
                if let Node::Tree(t) = o {
                    if t.position.x < -100.0 {
                        self.content.remove_node(id);
                    }
                }
            }
        }
        
        fn spawn_trees(&mut self) {
            const TREE_RATE : f32 = 2.0;
            self.spawn_cooldown -= self.delta;
            
            if self.spawn_cooldown < 0.0 {
                let mut rng = self.seed.rng();
                self.spawn_cooldown = rng.poisson(TREE_RATE) + 0.2;
                let direction = Rotation2::new(STEEP) * Vector2::x();
                let spawn_position = direction * 500.0;

                let offset = rng.triangular(20.0, 200.0, 110.0);
                self.content.add_node(Node::Tree(Tree::new(Point2::new(0.0, offset) + spawn_position)));

                self.seed = rng.seed();
            }
        }
    }
    
    impl<B> Update<B> for World
        where
            B: ElapsedDelta + JoystickProvider,  // Here we can specify backend capabilities
    {
        fn update(&mut self, backend: &mut B) {
            
            self.delta = backend.delta();
            let mut is_alive = true;

            let ids: Vec<NodeIndex> = self.content.node_indices().collect();
            for id in ids {
                let mut o = self.content[id].clone();
                o.update(self);
                self.content[id] = o;
            }
            
            self.clear_trees();
            
            self.spawn_trees();
            
            if let Some(jid) = backend.available_joysticks().first() {
                let joystick = backend.joystick(*jid);
                let direction = Rotation2::new(joystick.left_axis.y * 2.0) * Rotation2::new(STEEP) * Vector2::x();
                
                if let Node::Player(ref mut player) = self.content[self.player_id.clone()] {
                    player.direction = direction;
                }
            }
        }
    }

    impl Update<World> for Node {
        fn update(&mut self, w: &mut World) {
            match self {
                Node::Player(p) => p.update(w),
                Node::Tree(t) => {
                    let direction = Rotation2::new(STEEP) * Vector2::x();
                    t.position -= direction * w.delta() * w.speed;
                },
                _ => {},
            }
        }
    }
    
    impl Update<World> for Player {
        fn update(&mut self, w: &mut World) {
            eprintln!("Position: {:?}", self.position);
            eprintln!("Delta: {:?}", w.delta());
            self.position += Matrix2::new(0.0, 0.0, 0.0, 1.0) * (Rotation2::new(-STEEP) * self.direction * w.delta() * w.speed);
        }
    }
}

mod graphics {
    use nalgebra::*;
    use mursten::graphics::{Draw, Graphics, DrawPrimitives, DrawMode, Color};
    use super::*;

    #[derive(Clone, Copy)]
    enum Palette {
        White,
        Green,
        Brown,
        Black,
    }

    impl Color for Palette {
        fn into_rgba(self) -> [f32; 4] {
            match self {
                Palette::White => [1.00, 1.00, 1.00, 1.0],
                Palette::Green => [0.10, 1.00, 0.10, 1.0],
                Palette::Brown => [0.40, 0.26, 0.13, 1.0],
                Palette::Black => [0.00, 0.00, 0.00, 1.0],
            }
        }
    }

    impl<S> Draw<S> for World
    where
        S: DrawPrimitives,
    {
        fn draw(&self, surface: &mut S) {
            let mut ids: Vec<NodeIndex> = self.content.node_indices().collect();
            
            ids.sort_by(|a, b| {
                let a = self.content[a.clone()].coordinates().y;
                let b = self.content[b.clone()].coordinates().y;
                a.partial_cmp(&b).unwrap()
            });
            surface.clear(Palette::Black);
            for id in ids {
                self.content[id].draw(surface);
            }
            surface.present();
        }
    }

    impl<S> Draw<S> for Node
    where
        S: DrawPrimitives,
    {
        fn draw(&self, surface: &mut S) {
            match self {
                Node::Player(p) => p.draw(surface),
                Node::Tree(t) => t.draw(surface),
                _ => {},
            }
        }
    }

    impl<S> Draw<S> for Player
    where
        S: DrawPrimitives,
    {
        fn draw(&self, surface: &mut S) {
            
            let project = |ps: Vec<Point3<f32>>| -> Vec<Point2<f32>> {
                ps.iter().map(|p| {
                    let p = Rotation3::look_at_lh(
                        &Vector3::new(self.direction.y * -10.0, -2.0, 10.0),
                        &Vector3::y()
                    ) * p;
                    let s = 4.0;
                    self.position + Matrix2::new(s, 0.0, 0.0, s) * Vector2::new(p.x, -p.y)
                }).collect()
            };
            
            let draw = |surface: &mut S, color, ps: Vec<Point3<f32>>| {
                surface.set_color(color);
                let ps2 = ps.iter().map(|p| { Matrix3::new(-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) * p }).collect();
                surface.polygon(DrawMode::Fill, &project(ps));
                surface.polygon(DrawMode::Fill, &project(ps2));
            };

            // Board
            draw(surface, Palette::White, vec![
                Point3::new(0.0, 0.0, -1.0),
                Point3::new(5.0, 0.0, -1.5),
                Point3::new(6.0, 0.0, -1.0),
                Point3::new(6.0, 0.0, 1.0),
                Point3::new(5.0, 0.0, 1.5),
                Point3::new(0.0, 0.0, 1.0)
            ]);

            // Shoe
            draw(surface, Palette::Green, vec![
                Point3::new(3.0, 0.0, -0.8),
                Point3::new(3.5, 0.0, 0.8),
                Point3::new(2.5, 0.0, 0.8)
            ]);

            // Lower leg
            draw(surface, Palette::Brown, vec![
                Point3::new(3.5, 0.0, 0.8),
                Point3::new(2.5, 0.0, 0.8),
                Point3::new(1.0, 2.0, -2.0),
                Point3::new(2.0, 2.0, -2.0),
            ]);
            
            // Upper leg
            draw(surface, Palette::Brown, vec![
                Point3::new(1.0, 2.0, -2.0),
                Point3::new(2.0, 2.0, -2.0),
                Point3::new(1.0, 4.0, 0.5),
                Point3::new(0.0, 4.0, 0.5),
            ]);
            
            // Torso
            draw(surface, Palette::White, vec![
                Point3::new(1.0, 4.0, 0.5),
                Point3::new(0.0, 4.0, 0.5),
                Point3::new(0.0, 7.0, 0.0),
                Point3::new(1.0, 7.0, 0.0),
            ]);
            
            // Upper arm
            draw(surface, Palette::White, vec![
                Point3::new(0.5, 7.0, 0.0),
                Point3::new(1.0, 7.0, 0.0),
                Point3::new(2.5, 6.0, 2.0),
                Point3::new(2.0, 6.0, 2.0),
            ]);
            
            // Lower arm
            draw(surface, Palette::White, vec![
                Point3::new(2.5, 6.0, 2.0),
                Point3::new(2.0, 6.0, 2.0),
                Point3::new(3.0, 5.0, 0.0),
                Point3::new(3.5, 5.0, 0.0),
            ]);
            
            // Head
            draw(surface, Palette::Brown, vec![
                Point3::new(0.0, 7.0, 0.0),
                Point3::new(1.0, 7.0, 0.0),
                Point3::new(1.0, 9.0, 0.0),
                Point3::new(0.0, 9.0, 0.0),
            ]);
        }
    }
    
    impl<S> Draw<S> for Tree
    where 
        S: DrawPrimitives,
    {
        fn draw(&self, surface: &mut S) {
            let o = self.position.clone();
            let mut rng = self.seed.rng();
            
            let w = 20.0;
            let bw = rng.triangular(0.16 * w, 0.25 * w, 0.20 * w);
            let w1 = w;
            let w2 = rng.triangular(0.5 * w1, 0.9 * w1, 0.75 * w1);
            let w3 = rng.triangular(0.3 * w2, 0.8 * w2, 0.50 * w2);
            let h = rng.triangular(2.0 * w, 4.0 * w, 3.0 * w) / 4.0;
            
            let p = 0.7;

            let ps = vec![
                Vector2::new(-bw, 0.0),
                Vector2::new(bw, 0.0),
                Vector2::new(bw, -h),
                Vector2::new(-bw, -h),
            ];
            let ps : Vec<Point2<f32>> = ps.iter().map(|p| { o + p }).collect();
            
            surface.set_color(Palette::Brown);
            surface.polygon(DrawMode::Fill, &ps);
            
            let ps = vec![
                Vector2::new(0.0, -h),
                Vector2::new(w1, -h),
                Vector2::new(w2 * p, -h * 2.0),
                Vector2::new(w2, -h * 2.0),
                Vector2::new(w3 * p, -h * 3.0),
                Vector2::new(w3, -h * 3.0),
                Vector2::new(0.0, -h * 4.0)
            ];
            let ps : Vec<Vector2<f32>> = ps.iter().cloned().chain(ps.iter().rev().map(|p| { &Matrix2::new(-1.0, 0.0, 0.0, 1.0) * p })).collect();
            let ps : Vec<Point2<f32>> = ps.iter().map(|p| { o + p }).collect();

            surface.set_color(Palette::Green);
            surface.polygon(DrawMode::Fill, &ps);
        }
    }
}



impl World {
    fn new(seed: Seed) -> Self {
        let mut content = StableGraph::new();

        content.add_node(Node::Tree(Tree::new(Point2::new(100.0, 100.0))));
        content.add_node(Node::Tree(Tree::new(Point2::new(200.0, 100.0))));
        content.add_node(Node::Tree(Tree::new(Point2::new(100.0, 200.0))));
        content.add_node(Node::Tree(Tree::new(Point2::new(200.0, 180.0))));
        content.add_node(Node::Monster(Monster::new(Point2::new(200.0, 200.0))));
        let player_id = content.add_node(Node::Player(Player::new()));

        Self { seed, content, player_id, speed: 100.0, delta: 0.0, spawn_cooldown: 3.0 }
    }
}

use mursten::Scene;

impl Scene for World {}

use mursten::Game;
use mursten_ggez_backend::GgezBackend;

fn main() {
    Game::new(GgezBackend::new(320, 240))
        .run(World::new(Seed::new(1)));
}
