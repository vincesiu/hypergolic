use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use std::process::exit;

struct Universe {
    surface: GlfwSurface,
}

fn intialize_universe() -> Universe {
    let surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "Hello, world!",
        WindowOpt::default(),
    );

    let surface: GlfwSurface = match surface {
        Ok(surface) => {
            eprintln!("graphics surface created");
            surface
        }

        Err(e) => {
            eprintln!("cannot create graphics surface:\n{}", e);
            exit(1);
        }
    };
    return Universe { surface: surface };
}

fn process_user_input(universe: &mut Universe) {
    for event in universe.surface.poll_events() {
        match event {
            WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => exit(0),
            _ => (),
        }
    }
}

fn update_universe(universe: &mut Universe) {}

fn render(universe: &mut Universe) {
    // rendering code goes here

    // swap buffer chains
    universe.surface.swap_buffers();
}

fn main() {
    // the venerable game loop
    let mut universe: Universe = intialize_universe();
    loop {
        process_user_input(&mut universe);
        update_universe(&mut universe);
        render(&mut universe);
    }
}
