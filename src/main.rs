#[derive(Debug)]
struct Universe {}

fn intialize_universe() -> Universe {
    return Universe {};
}

fn process_user_input(universe: &mut Universe) {}

fn update_universe(universe: &mut Universe) {}

fn render(universe: &mut Universe) {
    println!("{:?}", universe);
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
