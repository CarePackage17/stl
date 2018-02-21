#[macro_use]
extern crate glium;
extern crate parsing_experiments;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use glium::{glutin, Surface};
use parsing_experiments::{parse_ascii_stl, Vertex};

fn main() {
    //so, our vertex and facet types are there;
    //however, glium expects flat arrays for implement_vertex!
    //could we use the into trait, maybe?
    //could we make the data layout fixed by default and return a Vec<f32> that
    //contains all the vertices?
    //but then normals would have to be duplicated unless I'm wrong and there is
    //a way to do per-facet normals with vertex buffers which I'm not aware of.
    //anyway, first sample should be simple and easy and doesn't need to be efficient
    //at all. so we can just parse and then copy everything Into<AppVertex> or whatever.
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No file path given on command line");
    } else {
        let filename = &args[1];
        println!("Trying to parse file at {}", filename);

        let mut file = File::open(filename).expect("failed to open file");
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        
        let faces = parse_ascii_stl(&buffer).unwrap().1;
        
        //create vertex buffer here
    //     let mut events_loop = glutin::EventsLoop::new();
    //     let window = glutin::WindowBuilder::new();
    //     let context = glutin::ContextBuilder::new();
    //     let display = glium::Display::new(window, context, &events_loop).unwrap();

    //     let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    //     // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    //     // let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
    //     //                                     &teapot::INDICES).unwrap();

    //     let vertex_shader_src = r#"
    //         #version 140
    //         in vec3 position;
    //         in vec3 normal;
    //         uniform mat4 matrix;
    //         void main() {
    //             gl_Position = matrix * vec4(position, 1.0);
    //         }
    //     "#;

    //     let fragment_shader_src = r#"
    //         #version 140
    //         out vec4 color;
    //         void main() {
    //             color = vec4(1.0, 0.0, 0.0, 1.0);
    //         }
    //     "#;

    //     let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
    //                                             None).unwrap();

    //     let mut closed = false;
    //     while !closed {
    //         let mut target = display.draw();
    //         target.clear_color(0.0, 0.0, 1.0, 1.0);

    //         let matrix = [
    //             [0.01, 0.0, 0.0, 0.0],
    //             [0.0, 0.01, 0.0, 0.0],
    //             [0.0, 0.0, 0.01, 0.0],
    //             [0.0, 0.0, 0.0, 1.0f32]
    //         ];

    //         // target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix },
    //         //             &Default::default()).unwrap();
    //         target.finish().unwrap();

    //         events_loop.poll_events(|event| {
    //             match event {
    //                 glutin::Event::WindowEvent { event, .. } => match event {
    //                     glutin::WindowEvent::Closed => closed = true,
    //                     _ => ()
    //                 },
    //                 _ => (),
    //             }
    //         });
    //     }
    }
}