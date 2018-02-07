#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertex() {
        let vertex = "vertex 1.0 2.3 3.4";

        let tuple = vertex_parser(vertex.as_bytes()).unwrap().1;
        assert_eq!(tuple.0, 1.0f32);
        assert_eq!(tuple.1, 2.3f32);
        assert_eq!(tuple.2, 3.4f32);
    }

    #[test]
    fn parse_vertex_custom_type() {
        let vertex_str = b"vertex 1.0 2.3 3.4";

        let vertex = vertex_with_custom_type(vertex_str).unwrap().1;
        assert_eq!(vertex.x, 1.0f32);
        assert_eq!(vertex.y, 2.3f32);
        assert_eq!(vertex.z, 3.4f32);
    }

    #[test]
    fn parse_triangle() {
        // let triangle_str = b"vertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3";
        let triangle_str = b"  vertex  1.0 2.3 3.4 \nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3";

        //we get a (Vertex, Vertex, Vertex) here
        let triangle = triangle_parser(triangle_str).unwrap().1;

        assert_eq!(triangle.0.x, 1.0f32);
        assert_eq!(triangle.0.y, 2.3f32);
        assert_eq!(triangle.0.z, 3.4f32);

        assert_eq!(triangle.1.x, 2.2f32);
        assert_eq!(triangle.1.y, 2.5f32);
        assert_eq!(triangle.1.z, 3.9f32);

        assert_eq!(triangle.2.x, 3.5f32);
        assert_eq!(triangle.2.y, 40.1f32);
        assert_eq!(triangle.2.z, 22.3f32);
    }

    #[test]
    fn parse_loop_record() {
        let loop_str =
            b"outer loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop";

        let triangle = loop_record(loop_str).unwrap().1;

        assert_eq!(triangle.0.x, 1.0f32);
        assert_eq!(triangle.0.y, 2.3f32);
        assert_eq!(triangle.0.z, 3.4f32);

        assert_eq!(triangle.1.x, 2.2f32);
        assert_eq!(triangle.1.y, 2.5f32);
        assert_eq!(triangle.1.z, 3.9f32);

        assert_eq!(triangle.2.x, 3.5f32);
        assert_eq!(triangle.2.y, 40.1f32);
        assert_eq!(triangle.2.z, 22.3f32);
    }

    #[test]
    fn parse_facet() {
        let facet_bytes = b"normal 0.2 0.3 0.4\nouter loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop";

        let facet = facet_parser(facet_bytes).unwrap().1;
        assert_eq!(facet.0.x, 0.2f32);
        assert_eq!(facet.0.y, 0.3f32);
        assert_eq!(facet.0.z, 0.4f32);
    }

    #[test]
    fn parse_facet_complete() {
        let facet_bytes = b"facet  normal 0.2 0.3 0.4\nouter loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop endfacet";

        //in this case facet is a Vertex 4-tuple
        let four_tuple = facet_parser_complete(facet_bytes).unwrap().1;

        //normal
        assert_eq!(four_tuple.0.x, 0.2f32);
        assert_eq!(four_tuple.0.y, 0.3f32);
        assert_eq!(four_tuple.0.z, 0.4f32);
        //vertex 1
        assert_eq!(four_tuple.1.x, 1.0f32);
        assert_eq!(four_tuple.1.y, 2.3f32);
        assert_eq!(four_tuple.1.z, 3.4f32);
        //vertex 2
        assert_eq!(four_tuple.2.x, 2.2f32);
        assert_eq!(four_tuple.2.y, 2.5f32);
        assert_eq!(four_tuple.2.z, 3.9f32);
        //vertex 3
        assert_eq!(four_tuple.3.x, 3.5f32);
        assert_eq!(four_tuple.3.y, 40.1f32);
        assert_eq!(four_tuple.3.z, 22.3f32);
    }
}

#[macro_use]
extern crate nom;

use nom::float;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(tuple: (f32, f32, f32)) -> Vertex {
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
    // pub fn from_vertices(tuple: (Vertex, Vertex, Vertex)) -> Facet {
    //     Facet {
    //         vertices =
    //     }
    // }
}

named!(pub vertex_parser<&[u8], (f32, f32, f32)>, ws!(preceded!(tag!("vertex"), tuple!(float, float, float))));

named!(pub normal_parser<&[u8], (f32, f32, f32)>, ws!(preceded!(tag!("normal"), tuple!(float, float, float))));

named!(pub vertex_with_custom_type<&[u8], Vertex>, map!(vertex_parser, Vertex::new));

named!(pub normal_with_custom_type<&[u8], Vertex>, map!(normal_parser, Vertex::new));

named!(pub triangle_parser<&[u8], (Vertex, Vertex, Vertex)>,
    ws!(
        tuple!(vertex_with_custom_type, vertex_with_custom_type, vertex_with_custom_type)
    )
);

//we also need to deal with outer loop and endloop words
named!(pub loop_record<&[u8], (Vertex, Vertex, Vertex)>, ws!(delimited!(tag!("outer loop"), triangle_parser, tag!("endloop"))));

//single facet needs a custom return type that wraps the normal and the 3 vertices
// named!(pub facet_parser<&[u8], (Vertex, Vertex, Vertex, Vertex)>, ws!(delimited!(preceded!("facet", vertex_with_custom_type), loop_record, tag!("endfacet"))));

named!(pub facet_parser<&[u8], (Vertex, (Vertex, Vertex, Vertex))>, ws!(tuple!(normal_with_custom_type, loop_record))); //at the end we still need endfacet -> terminated!

//do_parse! seems to be the solution for multiple subparser chaining and result aggregation
named!(pub facet_parser_complete<&[u8], (Vertex, Vertex, Vertex, Vertex)>,
    ws!(
        do_parse!(
            tag!("facet") >>
            normal: normal_with_custom_type >>
            tri: loop_record >>
            tag!("endfacet") >>
            (normal, tri.0, tri.1, tri.2)
        )
    )
);

//for parsing multiple facets whe should use many0

// facet normal ni nj nk
//     outer loop
//         vertex v1x v1y v1z
//         vertex v2x v2y v2z
//         vertex v3x v3y v3z
//     endloop
// endfacet
