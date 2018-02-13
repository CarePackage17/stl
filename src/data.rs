pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn from_tuple(tuple: (f32, f32, f32)) -> Vertex {
        Vertex {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

pub struct Facet {
    pub normal: Vertex,
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Facet {
    pub fn from_tuple(tuple: (Vertex, Vertex, Vertex, Vertex)) -> Facet {
        Facet { normal: tuple.0, a: tuple.1, b: tuple.2, c: tuple.3 }
    }
}