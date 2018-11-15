
/// This is the real game implementation, 
///

extern crate nalgebra;
extern crate petgraph;
extern crate ggez;

use engine::logical::Update;
use engine::graphics::{Draw, Graphics, DrawPrimitives, DrawMode, Color};
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
    Dead,
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
    lives: u32,
}

impl Update<World> for Player {
    fn update(&mut self, w: &mut World) {
        //self.lives -= 1;
        if self.lives > 0 {
            println!("I'm the player and I can think for myself!");
        } else {
            println!("Ouch, not anymore!");
        }
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
        let player_id = content.add_node(Node::Player(Player { lives: 3, }));
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

        if let Node::Player(ref p) = self[self.player_id] {
            is_alive = p.lives > 0;
        }
    }
}

#[derive(Clone)]
struct Tree;

#[derive(Clone)]
struct Monster;


pub struct Game {
    context: ggez::Context,
}

impl Game {
    fn new(w: u32, h: u32) -> Self {
        let mut c = ggez::conf::Conf::new();
        c.window_setup.title = "t".to_string();
        c.window_mode.width = w;
        c.window_mode.height = h;

        Self {
            context: ggez::Context::load_from_conf("_", "_", c).unwrap(),
        }
    }

    fn run<W>(&mut self, world: W)
    where
        for<'a> W: Update<ggez::Context> + Draw<Screen<'a>>
    {
        ggez::event::run(&mut self.context, &mut Main::new(world)).unwrap();
    }
}

pub struct Screen<'a> {
    ctx: &'a mut ggez::Context,
    precision: f32,
}

impl<'a> Graphics for Screen<'a> {
    fn clear<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_background_color(&mut self.ctx, color);
        ggez::graphics::clear(&mut self.ctx);
    }
    fn present(&mut self) {
        ggez::graphics::present(&mut self.ctx);
    }
}

fn convert_draw_mode(dm: DrawMode) -> ggez::graphics::DrawMode {
    match dm {
        DrawMode::Line(w) => ggez::graphics::DrawMode::Line(w),
        DrawMode::Fill => ggez::graphics::DrawMode::Fill,
    }
}

impl<'a> DrawPrimitives for Screen<'a> {
    fn set_color<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_color(&mut self.ctx, color);
    }
    fn circle(&mut self, dm: DrawMode, origin: Point2<f32>, radius: f32) {
        ggez::graphics::circle(
            &mut self.ctx,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            radius,
            self.precision,
        ).unwrap();
    }
    fn ellipse(&mut self, dm: DrawMode, origin: Point2<f32>, width: f32, height: f32) {
        ggez::graphics::ellipse(
            &mut self.ctx,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            width,
            height,
            self.precision,
        ).unwrap();
    }
    fn line(&mut self, origin: Point2<f32>, target: Point2<f32>, width: f32) {
        ggez::graphics::line(
            &mut self.ctx,
            &[
                ggez::nalgebra::Point2::new(origin.x, origin.y),
                ggez::nalgebra::Point2::new(target.x, target.y),
            ],
            width,
        ).unwrap();
    }
}


pub struct Main<W>
{
    world: W,
}

impl<W> Main<W> {
    pub fn new(world: W) -> Self {
        Self { world }
    }
}

impl<W> ggez::event::EventHandler for Main<W>
where
    for<'a> W: Update<ggez::Context> + Draw<Screen<'a>>,
{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {

        self.world.update(ctx);
        
        Ok(())
    }

    fn draw<'b>(&mut self, ctx: &'b mut ggez::Context) -> ggez::GameResult<()> {

        let mut screen : Screen<'b> = Screen { ctx, precision: 0.5 };

        self.world.draw(&mut screen);

        Ok(())
    }
}




fn main() {
    let game = Game::new(320, 240).run(World::new());
}
