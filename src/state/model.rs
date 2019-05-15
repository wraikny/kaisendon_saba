#[derive(Debug)]
crate struct Model {
    pub next_id : u32,
}

impl Model {
    crate fn new() -> Model {
        Model { next_id : 0 }
    }
}