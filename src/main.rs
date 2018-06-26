#[macro_use] extern crate prismatic;
#[macro_use] extern crate stdweb;
#[macro_use] extern crate stdweb_derive;
#[macro_use] extern crate serde_derive;

mod webpage;

use webpage::prelude::*;

fn main() {
    let webpage = Webpage::new("FSM");
    let context = webpage.context();

    let vert_code = include_str!("shader.vert");
    let frag_code = include_str!("shader.frag");

    let vert_shader = context.create_shader(GL::VERTEX_SHADER).unwrap();
    let frag_shader = context.create_shader(GL::FRAGMENT_SHADER).unwrap();

    context.shader_source(&vert_shader, vert_code);
    context.shader_source(&frag_shader, frag_code);

    context.compile_shader(&vert_shader);
    context.compile_shader(&frag_shader);

    let program = context.create_program().unwrap();

    context.attach_shader(&program, &vert_shader);
    context.attach_shader(&program, &frag_shader);

    context.link_program(&program);

    let position = context.get_attrib_location(&program, "a_position") as u32;
    let buffer = context.create_buffer().unwrap();

    let vertices = TypedArray::<f32>::from(&[
        0.0, 0.0,
        0.0, 0.5,
        0.7, 0.0,
    ][..]).buffer();

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_1(GL::ARRAY_BUFFER, Some(&vertices), GL::STATIC_DRAW);

    let width = context.canvas().width();
    let height = context.canvas().height();

    context.viewport(0, 0, width as i32, height as i32);
    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(GL::COLOR_BUFFER_BIT);

    context.use_program(Some(&program));
    context.enable_vertex_attrib_array(position);

    context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer(position, 2, GL::FLOAT, false, 0, 0);

    context.draw_arrays(GL::TRIANGLES, 0, 3);
}
