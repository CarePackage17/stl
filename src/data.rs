/// A single vertex in an .stl file.
/// Consists of 3 f32 coordinates.
#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    /// Creates a Vertex from a 3-tuple of f32.
    pub fn from_tuple(tuple: (f32, f32, f32)) -> Vertex {
        Vertex {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

/// A triangle facet consisting of a normal vector and 3 vertices.
#[derive(Debug)]
pub struct Facet {
    pub normal: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
    pub attribute: u16
}

impl Facet {
    pub fn from_tuple(tuple: (Vertex, Vertex, Vertex, Vertex)) -> Facet {
        Facet { normal: tuple.0, v1: tuple.1, v2: tuple.2, v3: tuple.3, attribute: 0 }
    }

    pub fn from_tuple_with_attribute(tuple: (Vertex, Vertex, Vertex, Vertex, u16)) -> Facet {
        Facet { normal: tuple.0, v1: tuple.1, v2: tuple.2, v3: tuple.3, attribute: tuple.4 }
    }
}