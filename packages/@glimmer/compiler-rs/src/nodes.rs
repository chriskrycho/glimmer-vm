use std::any::Any;
use std::default::Default;

pub struct SourceLocation {
    pub source: Option<String>,
    pub start: Position,
    pub end: Position,
}

pub struct Position {
    /// >= 1
    pub line: u16,
    /// >= 0
    pub column: u8,
}

impl Position {
    pub fn new(line: u16, column: u8) -> Result<Position, String> {
        if line >= 1 {
            Ok(Position { line, column })
        } else {
            Err("line number cannot be 0".into())
        }
    }
}

impl Default for Position {
    fn default() -> Position {
        Position { line: 1, column: 0 }
    }
}

pub struct Program {
    pub body: Vec<Statement>,
    pub block_params: Vec<String>,
}

pub enum Statement {
    MustacheStatement(MustacheStatement),
    BlockStatement(BlockStatement),
    PartialStatement(PartialStatement),
    MustacheComment(MustacheCommentStatement),
    TextNode,
    ElementNode,
}

pub enum CallExpression {
    PathExpression(PathExpression),
    SubExpression(SubExpression),
}

pub struct Call {
    pub name: Option<CallExpression>,
    pub path: PathExpression,
    pub params: Vec<Expression>,
    pub hash: Hash,
}

pub enum MustachePath {
    Path(PathExpression),
    Literal(Literal),
}

pub struct MustacheStatement {
    pub path: MustachePath,
    pub params: Vec<Expression>,
    pub hash: Hash,
    pub escaped: bool,
}

pub struct BlockStatement {
    pub path: PathExpression,
    pub params: Vec<Expression>,
    pub hash: Hash,
    pub program: Program,
    pub inverse: Option<Program>,
}

pub struct ElementModifierStatement {
    pub path: PathExpression,
    pub params: Vec<Expression>,
    pub hash: Hash,
}

pub struct PartialStatement {
    pub name: CallExpression,
    pub params: Vec<Expression>,
    pub hash: Hash,
    pub ident: String,
    pub strip: StripFlags,
}

pub fn is_call(node: &Any) -> bool {
    node.downcast_ref::<SubExpression>().is_some()
        || node
            .downcast_ref::<MustacheStatement>()
            .map(|mustache| match mustache.path {
                MustachePath::Path(_) => true,
                _ => false,
            })
            .unwrap_or(false)
}

pub struct CommentStatement {
    pub value: String,
}

pub struct MustacheCommentStatement {
    pub value: String,
}

pub struct ElementNode {
    pub tag: String,
    pub self_closing: bool,
    pub attributes: Vec<AttrNode>,
    pub block_params: Vec<String>,
    pub modifiers: Vec<ElementModifierStatement>,
    pub comments: Vec<MustacheCommentStatement>,
    pub children: Vec<Statement>,
}

pub enum AttrValue {
    TextNode(TextNode),
    MustacheStatement(MustacheStatement),
    ConcatStatement(ConcatStatement),
}

pub struct AttrNode {
    pub name: String,
    pub value: AttrValue,
}

pub struct TextNode {
    pub chars: String,
}

pub enum ConcatParts {
    TextNode(TextNode),
    MustacheStatement(MustacheStatement),
}

pub struct ConcatStatement {
    pub parts: Vec<ConcatParts>,
}

pub enum Expression {
    SubExpression(SubExpression),
    PathExpression(PathExpression),
    Literal(Literal),
}

pub struct SubExpression {
    pub call: Box<Call>,
    pub path: PathExpression,
    pub params: Vec<Expression>,
    pub hash: Hash,
}

pub struct PathExpression {
    pub call: Box<Call>,
    pub data: bool,
    pub original: String,
    pub this: bool,
    pub parts: Vec<String>,
}

pub enum Literal {
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    UndefinedLiteral(UndefinedLiteral),
    NullLiteral(NullLiteral),
}

pub struct StringLiteral {
    pub value: String,
    pub original: String,
}

pub struct BooleanLiteral {
    pub value: bool,
    pub original: bool,
}

/// The type is `f64` because JavaScript `number` type is an IEEE 754 64-bit
/// floating-point number.
pub struct NumberLiteral {
    pub value: f64,
    pub original: f64,
}

/// A placeholder type to represent the JS `undefined` value/type.
pub struct Undefined;

/// A placeholder type to represent the JS `null` value/type.
pub struct Null;

pub struct UndefinedLiteral {
    pub value: Undefined,
    pub original: Undefined,
}

pub struct NullLiteral {
    pub value: Null,
    pub original: Null,
}

pub struct Hash {
    pub pairs: Vec<HashPair>,
}

pub struct HashPair {
    pub key: String,
    pub value: Expression,
}

pub struct StripFlags {
    pub open: bool,
    pub close: bool,
}

pub struct Node {
    pub loc: SourceLocation,
    pub node: Nodes,
}

pub enum Nodes {
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
