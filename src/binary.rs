//all functions that deal with binary stl go here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binary_header() {
        let bytes = include_bytes!("../test_files/cube_bin.stl");
        let verts = read_stl(bytes).unwrap().1;

        println!("{:?}", verts);
    }

    #[test]
    fn parse_vertex() {
        let vertex = [1.4f32, 1.6, 3.7];
        let vertex_bytes = unsafe { ::std::mem::transmute::<[f32; 3], [u8; 12]>(vertex) };
        
        let res = read_vertex(&vertex_bytes).unwrap().1;
        assert_eq!(res.x(), 1.4f32);
        assert_eq!(res.y(), 1.6f32);
        assert_eq!(res.z(), 3.7f32);
    }
}

use data::{Vertex, Facet};
use nom::{le_f32, le_u16, IResult};
use std::str::from_utf8;

//should we check that it doesn't start with "solid"?
//yeah, use verify! for that
// named!(read_header, take!(80));

named!(verify_header, 
    verify!(
            take!(80), |header: &[u8]| {
                let bytes = &header[0..5];
                let s = from_utf8(bytes).unwrap();

                !s.starts_with("solid")
            }
        )
);

named!(read_vertex<Vertex>, 
    map!(
        tuple!(le_f32, le_f32, le_f32),
        Vertex::from_tuple
    )
);

named!(read_facet<Facet>,
    map!(
        tuple!(read_vertex, read_vertex, read_vertex, read_vertex, le_u16),
        Facet::from_tuple_with_attribute
    )
);

//u16! macro requires endianness parameter to work
named!(read_all_facets<Vec<Facet>>,
    do_parse!(
        verify_header >>
        triangles: le_u16 >>
        facets: count!(read_facet, triangles as usize) >>
        (facets)
    )
);

pub fn read_stl(data: &[u8]) -> IResult<&[u8], Vec<Facet>> {
    read_all_facets(data)
}

//the format looks like this:
//UINT8[80] – Header
//UINT32 – Number of triangles
// foreach triangle
// REAL32[3] – Normal vector
// REAL32[3] – Vertex 1
// REAL32[3] – Vertex 2
// REAL32[3] – Vertex 3
// UINT16 – Attribute byte count
// end