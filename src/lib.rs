#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertex() {
        let vertex_str = b"vertex 1.0 2.3 3.4";

        let vertex = vertex_parser(vertex_str).unwrap().1;
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
        let facet_bytes = b"facet  normal 0.2 0.3 0.4\nouter loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop endfacet";

        let facet = facet_parser_struct(facet_bytes).unwrap().1;

        //normal
        assert_eq!(facet.normal.x, 0.2f32);
        assert_eq!(facet.normal.y, 0.3f32);
        assert_eq!(facet.normal.z, 0.4f32);
        //vertex 1
        assert_eq!(facet.a.x, 1.0f32);
        assert_eq!(facet.a.y, 2.3f32);
        assert_eq!(facet.a.z, 3.4f32);
        //vertex 2
        assert_eq!(facet.b.x, 2.2f32);
        assert_eq!(facet.b.y, 2.5f32);
        assert_eq!(facet.b.z, 3.9f32);
        //vertex 3
        assert_eq!(facet.c.x, 3.5f32);
        assert_eq!(facet.c.y, 40.1f32);
        assert_eq!(facet.c.z, 22.3f32);
    }

    #[test]
    fn parse_facets() {
        let facet_bytes = b"facet  normal 0.2 0.3 0.4\nouter loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop endfacet
                            \nfacet normal 55.5 66.6 77.7\nouter loop\nvertex 10.1 11.1 12.3\nvertex 20.2 21.3 24.6\nvertex 3.3 6.9 4.7\nendloop endfacet";

        let vec = facet_list_struct(facet_bytes).unwrap().1;
        assert_eq!(vec.len(), 2);

        let first = &vec[0];
        let second = &vec[1];

        //first face normal
        assert_eq!(first.normal.x, 0.2f32);
        assert_eq!(first.normal.y, 0.3f32);
        assert_eq!(first.normal.z, 0.4f32);

        assert_eq!(second.normal.x, 55.5f32);
        assert_eq!(second.normal.y, 66.6f32);
        assert_eq!(second.normal.z, 77.7f32);
    }

    #[test]
    fn parse_name() {
        let bytes = b"solid     dude aaaaa whatever   \n";

        let name = parse_beginning(bytes).unwrap().1;
        assert_eq!(name.unwrap(), b"dude");
    }

    #[test]
    fn parse_full() {
        let bytes = include_bytes!("../test_files/cube.stl");
        let verts = parse_ascii_stl(bytes).unwrap().1;

        let first = &verts[0];
        assert_eq!(first.normal.x, 0.0f32);
        assert_eq!(first.normal.y, 0.0f32);
        assert_eq!(first.normal.z, -1.0f32);

        assert_eq!(first.a.x, 0.0f32);
        assert_eq!(first.a.y, 0.0f32);
        assert_eq!(first.a.z, 0.0f32);

        assert_eq!(first.b.x, 1.0f32);
        assert_eq!(first.b.y, 1.0f32);
        assert_eq!(first.b.z, 0.0f32);

        assert_eq!(first.c.x, 1.0f32);
        assert_eq!(first.c.y, 0.0f32);
        assert_eq!(first.c.z, 0.0f32);
    }
}

#[macro_use]
extern crate nom;

mod data;

use data::{Vertex, Facet};
use nom::{float, is_space, alpha};


named!(vertex_parser<&[u8], Vertex>, 
    map!(
        ws!(
            preceded!(
                tag!("vertex"), tuple!(float, float, float)
            )
        ),
        Vertex::from_tuple
    )
);

named!(normal_parser<&[u8], Vertex>, 
    map!(
        ws!(
            preceded!(
                tag!("normal"), tuple!(float, float, float)
            )
        ),
        Vertex::from_tuple
    )
);

named!(triangle_parser<&[u8], (Vertex, Vertex, Vertex)>,
    ws!(
        tuple!(vertex_parser, vertex_parser, vertex_parser)
    )
);

//we also need to deal with outer loop and endloop words
named!(loop_record<&[u8], (Vertex, Vertex, Vertex)>, ws!(delimited!(tag!("outer loop"), triangle_parser, tag!("endloop"))));

//do_parse! seems to be the solution for multiple subparser chaining and result aggregation
named!(facet_parser_complete<&[u8], (Vertex, Vertex, Vertex, Vertex)>,
    ws!(
        do_parse!(
            tag!("facet") >>
            normal: normal_parser >>
            tri: loop_record >>
            tag!("endfacet") >>
            (normal, tri.0, tri.1, tri.2)
        )
    )
);

named!(facet_parser_struct<&[u8], Facet>,
    map!(
        facet_parser_complete, Facet::from_tuple
    )
);

named!(facet_list_struct<&[u8], Vec<Facet>>,
    ws!(
        many0!(facet_parser_struct)
    )
);

named!(parse_beginning<&[u8], Option<&[u8]>>,
    do_parse!(
        tag!("solid") >>
        take_while!(is_space) >>
        name: opt!(alpha) >>
        take_until_and_consume!("\n") >>
        (name)
    )
);

//sometimes there's no name after endsolid, sometimes there is
named!(parse_ending<&[u8], Option<&[u8]>>,
    do_parse!(
        tag!("endsolid") >>
        take_while!(is_space) >>
        name: opt!(alpha) >>
        (name)
    )
);

named!(pub parse_ascii_stl<&[u8], Vec<Facet>>,
    do_parse!(
        parse_beginning >>
        facets: facet_list_struct >>
        parse_ending >>
        (facets)
    )
);
