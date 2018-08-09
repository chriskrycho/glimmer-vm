use std::borrow::ToOwned;
use std::collections::HashMap;

#[derive(Copy, Clone)]
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
    #[derive(PartialEq)]
    pub struct Program;

    #[derive(Clone, PartialEq)]
    pub struct ConcatStatement;

    #[derive(Clone, PartialEq)]
    pub enum AttrValue {
        TextNode(TextNode),
        MustacheStatement(MustacheStatement),
        ConcatStatement(ConcatStatement),
    }

    #[derive(Clone, PartialEq)]
    pub struct AttrNode {
        pub name: String,
        pub value: AttrValue,
    }

    #[derive(Clone, PartialEq)]
    pub struct ElementModifierStatement;

    #[derive(Clone, PartialEq)]
    pub struct MustacheStatement;

    #[derive(Clone, PartialEq)]
    pub struct BlockStatement;

    #[derive(Clone, PartialEq)]
    pub struct PartialStatement;

    #[derive(Clone, PartialEq)]
    pub struct TextNode {
        pub chars: String,
    }

    #[derive(Clone, PartialEq)]
    pub struct CommentStatement {
        value: String,
    }

    #[derive(Clone, PartialEq)]
    pub struct MustacheCommentStatement {
        value: String,
    }

    #[derive(Clone, PartialEq)]
    pub struct PathExpression;

    #[derive(Clone, PartialEq)]
    pub struct SubExpression;

    #[derive(Clone, PartialEq)]
    pub struct Hash;

    #[derive(Clone, PartialEq)]
    pub struct HashPair;

    #[derive(Clone, PartialEq)]
    pub struct StringLiteral;

    #[derive(Clone, PartialEq)]
    pub struct BooleanLiteral;

    #[derive(Clone, PartialEq)]
    pub struct NumberLiteral;

    #[derive(Clone, PartialEq)]
    pub struct UndefinedLiteral;

    #[derive(Clone, PartialEq)]
    pub struct NullLiteral;

    #[derive(Clone, PartialEq)]
    pub struct ElementNode {
        tag: String,
        attributes: Vec<AttrNode>,
        block_params: Vec<String>,
        modifiers: Vec<ElementModifierStatement>,
        comments: Vec<MustacheCommentStatement>,
        children: Vec<Statement>,
    }

    #[derive(Clone, PartialEq)]
    pub enum Statement {
        MustacheStatement(MustacheStatement),
        BlockStatement(BlockStatement),
        PartialStatement(PartialStatement),
        MustacheCommentStatement(MustacheCommentStatement),
        CommentStatement(CommentStatement),
        TextNode(TextNode),
        ElementNode(ElementNode),
    }

    #[derive(PartialEq)]
    pub enum Node {
        Program(Program),
        ElementNode(ElementNode),
        AttrNode(AttrNode),
        TextNode(TextNode),
        MustacheStatement(MustacheStatement),
        BlockStatement(BlockStatement),
        PartialStatement(PartialStatement),
        ConcatStatement(ConcatStatement),
        MustacheCommentStatement(MustacheCommentStatement),
        ElementModifierStatement(ElementModifierStatement),
        CommentStatement(CommentStatement),
        PathExpression(PathExpression),
        SubExpression(SubExpression),
        Hash(Hash),
        HashPair(HashPair),
        StringLiteral(StringLiteral),
        BooleanLiteral(BooleanLiteral),
        NumberLiteral(NumberLiteral),
        UndefinedLiteral(UndefinedLiteral),
        NullLiteral(NullLiteral),
    }
}

pub trait SymbolTable {
    fn top() -> ProgramSymbolTable
    where
        Self: Sized,
    {
        ProgramSymbolTable::new()
    }

    fn has(&self, name: &str) -> bool;
    fn get(&self, name: &str) -> usize;

    fn get_locals_map(&self) -> Dict<usize>;
    fn get_eval_info(&self) -> core::EvalInfo;

    fn allocate_named(&mut self, name: &str) -> usize;
    fn allocate_block(&mut self, name: &str) -> usize;
    fn allocate(&mut self, identifier: &str) -> usize;

    fn child(&mut self, locals: Vec<String>) -> BlockSymbolTable
    where
        Self: Sized,
    {
        let symbols: Vec<usize> = locals.iter().map(|name| self.allocate(&name)).collect();

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
    parent: &'p mut SymbolTable,
    pub symbols: Vec<String>,
    pub slots: Vec<usize>,
}

impl<'p> BlockSymbolTable<'p> {
    fn new(
        parent: &'p mut SymbolTable,
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
        // TODO: this is *dumb*. Generally points to utility of `Vec<&str>`, I
        // suspect, but will need to see how lifetimes play out.
        self.symbols.contains(&name.to_owned()) || self.parent.has(name)
    }

    // This implementation is garbage. I hate it. However, it is equivalent to
    // the TS implementation, so it's a reasonable starting point. It would be
    // nice not to have to maintain slots and symbols independently, of course.
    fn get(&self, name: &str) -> usize {
        let slot = self.symbols.iter().position(|symbol| symbol == name);
        match slot {
            Some(slot) => self.slots
                .iter()
                .nth(slot)
                .expect("nth slot and symbol position should play nice")
                .to_owned(),
            None => self.parent.get(name),
        }
    }

    fn get_locals_map(&self) -> Dict<usize> {
        let mut dict = self.parent.get_locals_map();
        self.symbols.iter().for_each(|symbol| {
            dict.insert(symbol.to_owned(), self.get(&symbol));
        });
        dict
    }

    fn get_eval_info(&self) -> core::EvalInfo {
        self.get_locals_map()
            .values()
            .map(ToOwned::to_owned)
            .collect()
    }

    fn allocate_named(&mut self, name: &str) -> usize {
        self.parent.allocate_named(name)
    }

    fn allocate_block(&mut self, name: &str) -> usize {
        self.parent.allocate_block(name)
    }

    fn allocate(&mut self, identifier: &str) -> usize {
        self.parent.allocate(identifier)
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
    pub blank_child_text_nodes: Option<Vec<isize>>,
    pub symbols: Option<Box<SymbolTable>>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            parent_node: None,
            children: None,
            child_index: None,
            child_count: None,
            child_template_count: 0,
            mustache_count: 0,
            actions: Vec::new(),
            blank_child_text_nodes: None,
            symbols: None,
        }
    }
}

pub struct TemplateVisitor {
    current_frame_actual: Option<Frame>,
    frame_stack: Vec<Frame>,
    pub actions: Vec<Action>,
    program_depth: isize, // TODO: might actually be better as an `Option` in Rust?
}

impl TemplateVisitor {
    pub fn new() -> TemplateVisitor {
        TemplateVisitor {
            current_frame_actual: None,
            frame_stack: Vec::new(),
            program_depth: -1,
            actions: Vec::new(),
        }
    }

    pub fn current_frame(&mut self) -> &mut Frame {
        self.current_frame_actual.as_mut().expect("Expected a current frame")
    }

    pub fn visit(&mut self, node: ast::Node) {
        match node {
            ast::Node::Program(program) => self.program(program),
            ast::Node::ElementNode(element) => self.element_node(element),
            ast::Node::AttrNode(attr) => self.attr_node(attr),
            ast::Node::TextNode(text) => self.text_node(text),
            ast::Node::BlockStatement(block) => self.block_statement(block),
            ast::Node::PartialStatement(partial) => self.partial_statement(partial),
            ast::Node::CommentStatement(comment) => self.comment_statement(comment),
            ast::Node::MustacheCommentStatement(mustache_comment) => {
                self.mustache_comment_statement(mustache_comment)
            }
            ast::Node::MustacheStatement(mustache_statement) => {
                self.mustache_statement(mustache_statement)
            }
            _ => unimplemented!(),
        }
    }

    pub fn program(&mut self, program: ast::Program) {
        unimplemented!()
        // self.program_depth += 1;

        // let parentFrame = self.get_current_frame();
        // let programFrame = self.push_frame();
    }

    pub fn element_node(&self, element: ast::ElementNode) {
        unimplemented!()
    }

    pub fn attr_node(&mut self, attr: ast::AttrNode) {
        match attr.value {
            ast::AttrValue::TextNode(_) => (),
            _ => self.current_frame().mustache_count += 1,
        }
    }

    pub fn text_node(&mut self, text: ast::TextNode) {
        let frame = self.current_frame();
        if text.chars.is_empty() {
            let nodes = frame.blank_child_text_nodes.as_mut().expect("frame must have child nodes");
            let children = frame.children.as_ref().expect("frame must have children");
            nodes.push(dom_index_of(children, DOMNode::TextNode(text)));
        }
    }

    pub fn block_statement(&self, node: ast::BlockStatement) {
        unimplemented!()
    }

    pub fn partial_statement(&self, node: ast::PartialStatement) {
        unimplemented!()
    }

    pub fn comment_statement(&self, node: ast::CommentStatement) {
        unimplemented!()
    }

    pub fn mustache_comment_statement(&self, node: ast::MustacheCommentStatement) {
        unimplemented!()
    }

    pub fn mustache_statement(&self, node: ast::MustacheStatement) {
        unimplemented!()
    }

    fn get_current_frame(&self) -> Option<&Frame> {
        self.frame_stack.last()
    }

    fn push_frame(&mut self) -> &Frame {
        let frame = Frame::new();
        self.frame_stack.push(frame);
        self.get_current_frame().expect("Just pushed frame, so it must be present")
    }

    fn pop_Frame(&mut self) -> Option<Frame> {
        self.frame_stack.pop()
    }
}

#[derive(PartialEq)]
enum DOMNode {
    TextNode(ast::TextNode),
    ElementNode(ast::ElementNode),
}

trait IntoSafe<T> : Sized {
    fn into_safe(&self) -> Option<T>;
}

impl IntoSafe<DOMNode> for ast::Node {
    fn into_safe(&self) -> Option<DOMNode> {
        match self {
            ast::Node::TextNode(tn) => Some(DOMNode::TextNode(tn.to_owned())),
            ast::Node::ElementNode(en) => Some(DOMNode::ElementNode(en.clone())),
            _ => None,
        }
    }
}

impl PartialEq<DOMNode> for ast::Node {
    fn eq(&self, other: &DOMNode) -> bool {
        match self.into_safe() {
            Some(dn) => other == &dn,
            None => false,
        }
    }
}

fn dom_index_of(nodes: &Vec<ast::Node>, dom_node: DOMNode) -> isize {
    let mut index = -1;

    for i in 0..nodes.len() {
        let node = nodes.get(i).expect("Only getting nodes within vec bounds");

        match node {
            ast::Node::TextNode(_) | ast::Node::ElementNode(_) => index += 1,
            _ => continue,
        }

        if node == &dom_node {
            return index
        }
    }

    -1
}
