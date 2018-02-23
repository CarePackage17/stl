//all functions that deal with binary stl go here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binary_header() {
        //take 80 bytes
        
    }

    #[test]
    fn parse_vertex() {
        let vertex = [1.4f32, 1.6, 3.7];
        let vertex_bytes = unsafe { ::std::mem::transmute::<[f32; 3], [u8; 12]>(vertex) };
        
        let res = read_vertex(&vertex_bytes).unwrap().1;
        assert_eq!(res.x, 1.4f32);
        assert_eq!(res.y, 1.6f32);
        assert_eq!(res.z, 3.7f32);
    }
}

use data::{Vertex, Facet};
use nom::{le_f32, le_u16, IResult};

//should we check that it doesn't start with "solid"?
named!(read_header, take!(80));

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
        read_header >>
        triangles: le_u16 >>
        facets: many_m_n!(triangles as usize, triangles as usize, read_facet) >>
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