/// Manages local variables and scoping during compilation.
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Local {
    sym: Sym,
    depth: usize,
    pub slot: usize,
    pub is_initialized: bool,
    pub is_captured: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Upvalue {
    pub slot: usize,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct Locals {
    pub stack: Vec<Local>,
    pub depth: usize,
}

impl Locals {
    pub fn new() -> Self {
        Locals {
            stack: vec![],
            depth: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.depth += 1;
    }

    pub fn leave_scope(&mut self) -> Vec<Local> {
        self.depth -= 1;
        let index = self
            .stack
            .iter()
            .enumerate()
            .find_map(|(index, local)| {
                if local.depth > self.depth {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.stack.len());
        self.stack.split_off(index)
    }

    pub fn mark_initialized(&mut self) {
        let index = self.stack.len() - 1;
        self.stack[index].is_initialized = true;
    }

    pub fn mark_captured(&mut self, slot: usize) {
        let local = self.stack.iter_mut().find(|l| l.slot == slot);
        if let Some(local) = local {
            local.is_captured = true;
        }
    }

    pub fn insert(&mut self, sym: &Sym) {
        if self.get_at_depth(sym, self.depth) == None {
            self.stack.push(Local {
                sym: *sym,
                depth: self.depth,
                slot: self.stack.len(),
                is_initialized: false,
                is_captured: false,
            })
        }
    }

    pub fn get(&self, sym: Sym) -> Option<&Local> {
        self.stack.iter().rev().find(|local| local.sym == sym)
    }
    fn get_at_depth(&self, sym: &Sym, depth: usize) -> Option<&Local> {
        self.stack
            .iter()
            .rev()
            .find(|local| local.sym == *sym && local.depth == depth)
    }
}
