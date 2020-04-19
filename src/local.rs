#[derive(Debug)]
pub struct Local {
    name: String,
    depth: usize,
    location: usize,
    initialized: bool,
}

impl Local {
    pub fn new(name: String, depth: usize, location: usize, initialized: bool) -> Self {
        Local {
            name,
            depth,
            location,
            initialized,
        }
    }

    pub fn location(&self) -> usize {
        self.location
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }
}

pub struct Locals {
    stack: Vec<Local>,
    scope_depth: usize,
}

impl Locals {
    pub fn new() -> Self {
        Locals {
            stack: vec![],
            scope_depth: 0,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;
        let index = self.stack.iter()
            .enumerate()
            .find_map(|(index, local)| if local.depth > self.scope_depth { Some(index) } else { None })
            .unwrap_or(self.stack.len());
        self.stack.split_off(index);
    }

    pub fn depth(&self) -> usize {
        self.scope_depth
    }

    pub fn get(&self, name: &str) -> Option<&Local> {
        self.stack.iter().rev().find(|local| local.name == name.to_string())
    }

    fn find_at_depth(&mut self, name: &str) -> Option<&Local> {
        self.stack.iter().rev().find(|local| &local.name == name && local.depth == self.scope_depth)
    }

    pub fn insert(&mut self, name: &str) -> usize {
        self.stack.push(Local {
            name: name.to_string(),
            depth: self.scope_depth,
            location: self.stack.len(),
            initialized: false,
        });
        self.stack.len() - 1
    }

    pub fn mark_initialized(&mut self) {
        self.stack.last_mut().unwrap().mark_initialized();
    }
}