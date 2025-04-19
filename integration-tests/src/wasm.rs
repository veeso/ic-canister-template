use std::path::Path;

pub enum Canister {
    HelloWorld,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::HelloWorld => Path::new("../.artifact/hello_world.wasm.gz"),
        }
    }
}
