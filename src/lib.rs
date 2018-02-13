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

    #[test]
    fn parse_facets() {
        let facet_bytes = b"facet  normal 0.2 0.3 0.4\nouter loop\nvertex 1.0 2.3 3.4\nvertex 2.2 2.5 3.9\nvertex 3.5 40.1 22.3\nendloop endfacet
                            \nfacet normal 55.5 66.6 77.7\nouter loop\nvertex 10.1 11.1 12.3\nvertex 20.2 21.3 24.6\nvertex 3.3 6.9 4.7\nendloop endfacet";

        let vec = facet_list(facet_bytes).unwrap().1;
        assert_eq!(vec.len(), 2);

        let first = &vec[0];
        let second = &vec[1];

        //first face normal
        assert_eq!(first.0.x, 0.2f32);
        assert_eq!(first.0.y, 0.3f32);
        assert_eq!(first.0.z, 0.4f32);

        assert_eq!(second.0.x, 55.5f32);
        assert_eq!(second.0.y, 66.6f32);
        assert_eq!(second.0.z, 77.7f32);
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
        assert_eq!(first.0.x, 0.0f32);
        assert_eq!(first.0.y, 0.0f32);
        assert_eq!(first.0.z, -1.0f32);

        assert_eq!(first.1.x, 0.0f32);
        assert_eq!(first.1.y, 0.0f32);
        assert_eq!(first.1.z, 0.0f32);

        assert_eq!(first.2.x, 1.0f32);
        assert_eq!(first.2.y, 1.0f32);
        assert_eq!(first.2.z, 0.0f32);

        assert_eq!(first.3.x, 1.0f32);
        assert_eq!(first.3.y, 0.0f32);
        assert_eq!(first.3.z, 0.0f32);
    }
}

#[macro_use]
extern crate nom;

mod data;

use data::Vertex;
use nom::{float, is_space, alpha};


named!(pub vertex_parser<&[u8], Vertex>, 
    map!(
        ws!(
            preceded!(
                tag!("vertex"), tuple!(float, float, float)
            )
        ),
        Vertex::from_tuple
    )
);

named!(pub normal_parser<&[u8], Vertex>, 
    map!(
        ws!(
            preceded!(
                tag!("normal"), tuple!(float, float, float)
            )
        ),
        Vertex::from_tuple
    )
);

named!(pub triangle_parser<&[u8], (Vertex, Vertex, Vertex)>,
    ws!(
        tuple!(vertex_parser, vertex_parser, vertex_parser)
    )
);

//we also need to deal with outer loop and endloop words
named!(pub loop_record<&[u8], (Vertex, Vertex, Vertex)>, ws!(delimited!(tag!("outer loop"), triangle_parser, tag!("endloop"))));

//single facet needs a custom return type that wraps the normal and the 3 vertices
named!(pub facet_parser<&[u8], (Vertex, (Vertex, Vertex, Vertex))>, ws!(tuple!(normal_parser, loop_record)));

//do_parse! seems to be the solution for multiple subparser chaining and result aggregation
named!(pub facet_parser_complete<&[u8], (Vertex, Vertex, Vertex, Vertex)>,
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

named!(pub facet_list<&[u8], Vec<(Vertex, Vertex, Vertex, Vertex)>>,
    ws!(
        many0!(facet_parser_complete)
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

named!(parse_ascii_stl<&[u8], Vec<(Vertex, Vertex, Vertex, Vertex)>>,
    do_parse!(
        parse_beginning >>
        facets: facet_list >>
        parse_ending >>
        (facets)
    )
);

//we need to verify! if the name matches at the beginning/end
//also, we should probably return custom structs instead of tuples (readability and such)
