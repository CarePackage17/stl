#[macro_use]
extern crate glium;
extern crate stl;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use glium::{glutin, Surface};
use stl::binary;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No file path given on command line");
    } else {
        let filename = &args[1];
        println!("Trying to parse file at {}", filename);

        let mut file = File::open(filename).expect("failed to open file");
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        
        // let faces = ascii::read_stl(&buffer).unwrap().1;
        let faces = binary::read_stl(&buffer).unwrap().1;
        
        implement_vertex!(AppVertex, position, normal);

        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        //we get a vec of facets which needs to be converted to a flat buffer of vertex data,
        //i.e. per-vertex positions and normals
        //this doesn't seem to be possible with iterators, at least not straightforward (or I just don't see
        //it in the docs). maybe changing the API to return a pair of Normal and Triangle would be easier
        //because then I could use unzip to create separate collections for both...but then I still need to merge.
        //also, itertools might be useful, so I should look into that before making hasty decisions.
        let mut vertices: Vec<AppVertex> = Vec::with_capacity(faces.len() * 3 * 3);

        for facet in faces {
            let v1 = AppVertex {
                position: [ facet.v1.x(), facet.v1.y(), facet.v1.z() ],
                normal: [ facet.normal.x(), facet.normal.y(), facet.normal.z() ]
            };

            let v2 = AppVertex {
                position: [ facet.v2.x(), facet.v2.y(), facet.v2.z() ],
                normal: [ facet.normal.x(), facet.normal.y(), facet.normal.z() ]
            };

            let v3 = AppVertex {
                position: [ facet.v3.x(), facet.v3.y(), facet.v3.z() ],
                normal: [ facet.normal.x(), facet.normal.y(), facet.normal.z() ]
            };

            vertices.push(v1);
            vertices.push(v2);
            vertices.push(v3);
        }

        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140
            in vec3 position;
            in vec3 normal;
            out vec3 v_normal;
            out vec3 v_position;
            uniform mat4 matrix;

            void main() {
                v_position = position;
                v_normal = normal;
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 v_normal;
            out vec4 f_color;
            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                f_color = vec4(color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                                None).unwrap();

        let mut closed = false;
        while !closed {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);

            let matrix = [
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ];

            target.draw(&vertex_buffer, &indices, &program, &uniform! { matrix: matrix },
                        &Default::default()).unwrap();
            target.finish().unwrap();

            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => closed = true,
                        _ => ()
                    },
                    _ => (),
                }
            });
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct AppVertex {
        position: [f32; 3],
        normal: [f32; 3]
    }
}