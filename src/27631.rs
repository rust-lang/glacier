#[macro_use]
extern crate glium;
extern crate num;
extern crate image;
extern crate nalgebra;

use std::io::Cursor;


fn main() {
    use glium::{DisplayBuild, Surface};
    use nalgebra::*;
    use num::traits::Float;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();


    let image = image::load(Cursor::new(&include_bytes!("/Users/tyoc213/Desktop/lala.png")[..]),
                            image::PNG).unwrap();
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let image2 = image::load(Cursor::new(&include_bytes!("/Users/tyoc213/Desktop/lala2.png")[..]),
                            image::PNG).unwrap();
    let texture2 = glium::texture::Texture2d::new(&display, image2).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        out float some;

        uniform mat4 matrix;
        uniform mat4 trans;
        uniform mat4 vera;

        void main() {
            v_tex_coords = tex_coords;
            some = matrix[0][0];
            // gl_Position = matrix * vec4(position, 0.0, 1.0); // the other matrix
            gl_Position = vera * trans * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in float some;
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;
        uniform sampler2D tex2;

        void main() {
            if(v_tex_coords.y < 0.25){
                vec4 color1 = texture(tex, v_tex_coords) * vec4(1.0, 0, 0, 1.0);;
                vec4 color2 = texture(tex2, v_tex_coords); // * vec4(1.0, 0, 0, 1.0);;
                color = mix(color1, color2, 0.8+some);
            }else if(v_tex_coords.y < 0.5){
                vec4 color1 = texture(tex, v_tex_coords) * vec4(1.0, 0, 0, 1.0);;
                color = color1;
            }else if(v_tex_coords.y < 0.75){
                vec4 color2 = texture(tex2, v_tex_coords); // * vec4(1.0, 0, 0, 1.0);;
                color = color2;
            } else {
                color = texture(tex2, vec2(v_tex_coords.x + sin(v_tex_coords.y * 60.0 + some * 2.0) / 30.0, 1.0 - v_tex_coords.y)) * vec4(0.7, 0.7, 1.0, 1.0);
            }

        }
    "#;

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = -0.5;

    loop {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let mut thing: Mat4<f32> = nalgebra::one();
        thing.m11 = t.cos();
        thing.m12 = t.sin();
        thing.m21 = -t.sin();
        thing.m22 = t.cos();
        let mut trot = Rot3::new(Vec3::new(0.0f64, 0.0,  <f64 as BaseFloat>::pi()));
        println!("trot 1 {:?}", trot);
        trot.look_at(&Vec::new(1.2, 1.2, 1.2), &Vec3::new(0.0,0.0,1.0));
        println!("trot 2 {:?}", trot);
        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0], // [1.0, 0.0, 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0], // [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0],
            ],
            tex: &texture,
            tex2: &texture2,
            trans: thing,
            view: trot,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
