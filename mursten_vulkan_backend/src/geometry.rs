use nalgebra::geometry::{Point2, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub color: [f32; 4],
    pub texture: [f32; 2],
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Point3::origin(),
            color: [1.0, 1.0, 1.0, 1.0],
            texture: [0.0, 0.0],
        }
    }
}

impl Vertex {
    pub fn at(position: Point3<f32>) -> Self {
        Vertex {
            position,
            ..Self::default()
        }
    }

    pub fn color(self: Vertex, r: f32, g: f32, b: f32, a: f32) -> Vertex {
        Vertex {
            color: [r, g, b, a],
            ..self
        }
    }
}

impl From<Point2<f32>> for Vertex {
    fn from(position: Point2<f32>) -> Self {
        Vertex {
            position: Point3::new(position.x, position.y, 0.0),
            ..Self::default()
        }
    }
}

impl From<Point3<f32>> for Vertex {
    fn from(position: Point3<f32>) -> Self {
        Vertex::at(position)
    }
}

pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Triangle { v1, v2, v3 }
    }
}

use std::vec;

impl IntoIterator for Triangle {
    type Item = Vertex;
    type IntoIter = vec::IntoIter<Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.v1, self.v2, self.v3].into_iter()
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            v1: Vertex::default(),
            v2: Vertex::default(),
            v3: Vertex::default(),
        }
    }
}

use nalgebra::Transform3;

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub transform: Transform3<f32>,
}
