use ggez::Context;
use ggez::graphics::Point2;
use math::VectorUtils;
use entities::Renderable;

#[derive(Debug)]
pub struct Bezier {
    points: Vec<Point2>,
    weights: Vec<Point2>,
}

impl Bezier {
    pub fn from(origin: Point2, weight: Point2) -> Bezier {
        let mut points = Vec::<Point2>::new();
        let mut weights = Vec::<Point2>::new();
        points.push(origin);
        weights.push(weight);
        Bezier {
            points,
            weights,
        }
    }
    pub fn to(mut self, p: Point2, w: Point2) -> Self {
        self.points.push(p);
        self.weights.push(w);
        self
    }

    pub fn get(&self, tt: f32) -> Point2 {
        if self.points.len() == 0 {
            panic!("Bezier points.len() can't be 0")
        }
        else if self.points.len() == 1 {
            *self.points.get(0).unwrap()
        }
        else if tt >= 1.0 {
            *self.points.get(self.points.len()-1).unwrap()
        }
        else {
            let n = (self.points.len() - 1) as f32;
            let i = (tt * n).floor() as usize;
            let t = (tt * n) - i as f32;

            let o = self.points.get(i).unwrap();
            let c1 = o.add(*self.weights.get(i).unwrap());

            let d = self.points.get(i+1).unwrap();
            let c2 = d.sub(*self.weights.get(i+1).unwrap());

            let a1 = o.mul((1.0-t).powi(3));
            let a2 = c1.mul(3.0*(1.0-t).powi(2)*t);
            let a3 = c2.mul(3.0*(1.0-t)*t.powi(2));
            let a4 = d.mul(t.powi(3));

            a1.add(a2).add(a3).add(a4)
        }
    }
}

impl Renderable for Bezier {
    fn render(&mut self, ctx: &mut Context) {
        use ggez::graphics;
        use palette::Palette;

        graphics::set_color(ctx, Palette::DebugA.into()).unwrap();

        let precision = 10;

        let lines = self.points.len() * precision;

        let v = (0..lines+1).map(|x| self.get(x as f32 / lines as f32)).collect::<Vec<Point2>>();

        graphics::line(ctx, &v, 1.0).unwrap();
    }
}

#[test]
fn bezier_with_two_points() {
    let b = Bezier::from(Point2::new(0.0, 0.0), Point2::new(1.0, 0.0))
        .to(Point2::new(0.0, 1.0), Point2::new(-1.0, 0.0));
    assert_eq!(b.get(0.0), Point2::new(0.0, 0.0));
    assert_eq!(b.get(0.5), Point2::new(0.75, 0.5));
    assert_eq!(b.get(1.0), Point2::new(0.0, 1.0));
}

#[test]
fn bezier_with_three_points() {
    let b = Bezier::from(Point2::new(0.0, 0.0), Point2::new(1.0, 0.0))
        .to(Point2::new(0.0, 1.0), Point2::new(-1.0, 0.0))
        .to(Point2::new(0.0, 2.0), Point2::new(1.0, 0.0));
    assert_eq!(b.get(0.0), Point2::new(0.0, 0.0));
    assert_eq!(b.get(0.25), Point2::new(0.75, 0.5));
    assert_eq!(b.get(0.5), Point2::new(0.0, 1.0));
    assert_eq!(b.get(0.75), Point2::new(-0.75, 1.5));
    assert_eq!(b.get(1.0), Point2::new(0.0, 2.0));
}
