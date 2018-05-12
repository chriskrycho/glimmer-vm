use std::collections::HashMap;
use std::borrow::ToOwned;

pub enum Action {
    StartProgram,
    EndProgram,
    StartBlock,
    EndBlock,
    Block,
    Mustache,
    OpenElement,
    CloseElement,
    Text,
    Comment,
}

type Dict<T> = HashMap<String, T>;

// Placeholder -- should be extern?
pub mod core {
    pub type EvalInfo = Vec<usize>;
}

// Placeholder -- should be extern?
pub mod ast {
    pub struct Node;
}

pub trait SymbolTable {
    fn top() -> ProgramSymbolTable where Self : Sized {
        ProgramSymbolTable::new()
    }

    fn has(&self, name: &str) -> bool;
    fn get(&self, name: &str) -> usize;

    fn get_locals_map(&self) -> Dict<usize>;
    fn get_eval_info(&self) -> core::EvalInfo;

    fn allocate_named(&mut self, name: &str) -> usize;
    fn allocate_block(&mut self, name: &str) -> usize;
    fn allocate(&mut self, identifier: &str) -> usize;

    fn child(&mut self, locals: Vec<String>) -> BlockSymbolTable where Self : Sized {
        let symbols: Vec<usize> = locals
            .iter()
            .map(|name| self.allocate(&name))
            .collect();

        return BlockSymbolTable::new(self, &locals, symbols);
    }
}

pub struct ProgramSymbolTable {
    pub symbols: Vec<String>,

    size: usize,
    named: Dict<usize>,
    blocks: Dict<usize>,
}

impl ProgramSymbolTable {
    fn new() -> ProgramSymbolTable {
        ProgramSymbolTable {
            symbols: Vec::new(),
            size: 1,
            named: Dict::new(),
            blocks: Dict::new(),
        }
    }
}

impl SymbolTable for ProgramSymbolTable {
    fn has(&self, _name: &str) -> bool {
        false
    }

    fn get(&self, _name: &str) -> usize {
        unreachable!()
    }

    fn get_locals_map(&self) -> Dict<usize> {
        Dict::new()
    }

    fn get_eval_info(&self) -> core::EvalInfo {
        core::EvalInfo::new()
    }

    // This is essentially a direct transcription of the version in TypeScript,
    // but it doesn't actually make much sense to me in even the medium-term in
    // Rust, for the simple reason that the "allocation" here is allocating a
    // string instead of just storing an actual reference. (Something like this
    // still might make sense if we want to avoid hairy lifetimes, but given
    // that's Rust's strong suit...)
    fn allocate_named(&mut self, name: &str) -> usize {
        let named = self.named.get(name).map(ToOwned::to_owned);
        match named {
            Some(named) => named,
            None => {
                let named = self.allocate(name);
                self.named.insert(name.to_owned(), named);
                named
            }
        }
    }

    fn allocate_block(&mut self, name: &str) -> usize {
        let block = self.blocks.get(name).map(ToOwned::to_owned);
        match block {
            Some(block) => block.to_owned(),
            None => {
                let block = self.allocate(&format!("&{}", name));
                self.blocks.insert(name.to_owned(), block);
                block
            }
        }
    }

    fn allocate(&mut self, identifier: &str) -> usize {
        self.symbols.push(identifier.to_owned());
        self.size += 1;
        self.size
    }
}

pub struct BlockSymbolTable<'p> {
    parent: &'p SymbolTable,
    pub symbols: Vec<String>,
    pub slots: Vec<usize>,
}

impl<'p> BlockSymbolTable<'p> {
    fn new(
        parent: &'p SymbolTable,
        symbols: &Vec<String>,
        slots: Vec<usize>,
    ) -> BlockSymbolTable<'p> {
        BlockSymbolTable {
            parent,
            symbols: symbols.to_vec(),
            slots: slots.to_vec(),
        }
    }
}

impl<'p> SymbolTable for BlockSymbolTable<'p> {
    fn has(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn get(&self, name: &str) -> usize {
        unimplemented!()
    }

    fn get_locals_map(&self) -> Dict<usize> {
        unimplemented!()
    }

    fn get_eval_info(&self) -> core::EvalInfo {
        unimplemented!()
    }

    fn allocate_named(&mut self, name: &str) -> usize {
        unimplemented!()
    }

    fn allocate_block(&mut self, name: &str) -> usize {
        unimplemented!()
    }

    fn allocate(&mut self, identifier: &str) -> usize {
        unimplemented!()
    }
}

// placeholder -- should be extern?
pub struct JSObject;

pub struct Frame {
    pub parent_node: Option<JSObject>,
    pub children: Option<Vec<ast::Node>>,
    pub child_index: Option<usize>,
    pub child_count: Option<usize>,
    pub child_template_count: usize,
    pub mustache_count: usize,
    pub actions: Vec<Action>,
    pub blank_child_text_nodes: Option<Vec<usize>>,
    pub symbols: Option<Box<SymbolTable>>,
}

pub struct TemplateVisitor {
    frame: Vec<Frame>,
}

impl TemplateVisitor {
    pub fn new() -> TemplateVisitor {
        TemplateVisitor { frame: Vec::new() }
    }
}
