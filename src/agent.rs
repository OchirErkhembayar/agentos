/// Agent struct
pub struct Agent {
    pub name: String,
    tools: Vec<Box<dyn Tool>>,
}

pub trait Tool {
}

impl Agent {
    /// Creates a new [`Agent`].
    pub fn new(name: String) -> Self {
        Self {
            name,
            tools: vec![],
        }
    }
}
