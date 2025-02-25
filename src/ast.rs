use crate::{span::Span, token::Kind};

/// Untyped AST Node kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstKind {
    Program,
    BooleanLiteral,
    NumericLiteral,
    StringLiteral,
    IdentifierReference,
    VariableExpression,
    VariableMember,
    ParenthesizedExpression,
    BlockExpression,
    BinaryExpression,
    UnaryExpression,
    TernaryExpression,
    ConditionalExpression,
    AssignmentExpression,
    ResourceExpression,
    ArrayAccessExpression,
    ArrowAccessExpression,
    CallExpression,
    LoopExpression,
    ForEachExpression,
    Break,
    Continue,
    This,
    Return,
}

/// Represents the root of a Molang expression AST, containing all the top-level
/// information.
#[derive(Debug, Clone)]
pub struct Program {
    pub span: Span,
    /// Determines whether the expression is complex or simple. If it contains
    /// at least one `;` or `=`, it is considered a complex expression.
    pub is_complex: bool,
    pub body: Vec<Expression>,
}

/// <https://bedrock.dev/docs/stable/Molang#Lexical%20Structure>
#[derive(Debug, Clone)]
pub enum Expression {
    BooleanLiteral(Box<BooleanLiteral>),
    NumericLiteral(Box<NumericLiteral>),
    StringLiteral(Box<StringLiteral>),
    Variable(Box<VariableExpression>),
    Parenthesized(Box<ParenthesizedExpression>),
    Block(Box<BlockExpression>),
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Ternary(Box<TernaryExpression>),
    Conditional(Box<ConditionalExpression>),
    Assignment(Box<AssignmentExpression>),
    Resource(Box<ResourceExpression>),
    ArrayAccess(Box<ArrayAccessExpression>),
    ArrowAccess(Box<ArrowAccessExpression>),
    Call(Box<CallExpression>),
    Loop(Box<LoopExpression>),
    ForEach(Box<ForEachExpression>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    This(Box<This>),
    Return(Box<Return>),
}

/// `true` or `false`
#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

impl BooleanLiteral {
    /// Returns `"true"` or `"false"` depending on this boolean's value.
    pub fn as_str(&self) -> &'static str {
        if self.value {
            "true"
        } else {
            "false"
        }
    }
}

/// `1.23` in `v.a = 1.23;`
#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub span: Span,
    pub value: f32,
}

/// <https://bedrock.dev/docs/stable/Molang#Strings>
///
/// `'foo bar'` in `v.a = 'foo bar';`
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub span: Span,
    pub value: String,
}

/// `foo` in `v.foo.bar`
#[derive(Debug, Clone)]
pub struct IdentifierReference {
    pub span: Span,
    pub name: String,
}

/// <https://bedrock.dev/docs/stable/Molang#Variables>
#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub span: Span,
    pub lifetime: VariableLifetime,
    pub member: VariableMember,
}

/// The variable lifetime associated with [`VariableExpression`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableLifetime {
    /// `temp` in `temp.foo`
    Temporary,
    /// `variable` in `variable.foo`
    Variable,
    /// `context` in `context.foo`
    Context,
}

impl VariableLifetime {
    /// String representation of the call kind ("temp", "variable", or "context").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Temporary => "temp",
            Self::Variable => "variable",
            Self::Context => "context",
        }
    }
}

impl From<Kind> for VariableLifetime {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Temporary => Self::Temporary,
            Kind::Variable => Self::Variable,
            Kind::Context => Self::Context,
            _ => unreachable!("Variable Lifetime: {kind:?}"),
        }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Structs>
#[derive(Debug, Clone)]
pub enum VariableMember {
    /// `foo.bar` in `v.foo.bar`
    Object {
        span: Span,
        object: Box<VariableMember>,
        property: IdentifierReference,
    },
    /// `foo` in `v.foo`
    Property {
        span: Span,
        property: IdentifierReference,
    },
}

#[derive(Debug, Clone)]
pub enum ParenthesizedExpression {
    /// `(1 + 1)` in `(1 + 1) * 2`
    Single { span: Span, expression: Expression },
    /// `(v.a = 1;)` in `(v.b = 'B'; v.a = 1;);`
    Complex {
        span: Span,
        expressions: Vec<Expression>,
    },
}

/// `{ v.a = 0; }` in `loop(10, { v.a = 0; })`
#[derive(Debug, Clone)]
pub struct BlockExpression {
    pub span: Span,
    pub expressions: Vec<Expression>,
}

/// `1 + 1` in `v.a = 1 + 1;`
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub span: Span,
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

/// Operators used in [`BinaryExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /// `==`
    Equality,
    /// `!=`
    Inequality,
    /// `<`
    LessThan,
    /// `<=`
    LessEqualThan,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqualThan,
    /// `+`
    Addition,
    /// `-`
    Subtraction,
    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `||`
    Or,
    /// `&&`
    And,
    /// `??`
    Coalesce,
}

impl BinaryOperator {
    /// The string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equality => "==",
            Self::Inequality => "!=",
            Self::LessThan => "<",
            Self::LessEqualThan => "<=",
            Self::GreaterThan => ">",
            Self::GreaterEqualThan => ">=",
            Self::Addition => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
            Self::Or => "||",
            Self::And => "&&",
            Self::Coalesce => "??",
        }
    }
}

impl From<Kind> for BinaryOperator {
    fn from(token: Kind) -> Self {
        match token {
            Kind::Eq => Self::Equality,
            Kind::NotEq => Self::Inequality,
            Kind::Lt => Self::LessThan,
            Kind::Gt => Self::GreaterThan,
            Kind::LtEq => Self::LessEqualThan,
            Kind::GtEq => Self::GreaterEqualThan,
            Kind::Or => Self::Or,
            Kind::And => Self::And,
            Kind::NullCoal => Self::Coalesce,
            Kind::Minus => Self::Subtraction,
            Kind::Plus => Self::Addition,
            Kind::Star => Self::Multiplication,
            Kind::Slash => Self::Division,
            _ => unreachable!("Binary Operator: {token:?}"),
        }
    }
}

/// `-1` in `q.foo(-1)`
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression,
}

/// Operators used in [`UnaryExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `-`
    Negate,
    /// `!`
    Not,
}

impl UnaryOperator {
    /// The string representation of this operator as it appears in source code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Negate => "-",
            Self::Not => "!",
        }
    }
}

impl From<Kind> for UnaryOperator {
    fn from(token: Kind) -> Self {
        match token {
            Kind::Minus => Self::Negate,
            Kind::Not => Self::Not,
            _ => unreachable!("Unary Operator: {token:?}"),
        }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Conditionals>
///
/// `q.foo ? 0 : 1`
#[derive(Debug, Clone)]
pub struct TernaryExpression {
    pub span: Span,
    pub test: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

/// <https://bedrock.dev/docs/stable/Molang#Conditionals>
///
/// `q.foo ? 0`
#[derive(Debug, Clone)]
pub struct ConditionalExpression {
    pub span: Span,
    pub test: Expression,
    pub consequent: Expression,
}

/// `v.a = 0;`
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub span: Span,
    pub left: VariableExpression,
    pub right: Expression,
}

/// <https://bedrock.dev/docs/stable/Molang#Resource%20Expression>
#[derive(Debug, Clone)]
pub struct ResourceExpression {
    pub span: Span,
    pub section: ResourceSection,
    pub name: IdentifierReference,
}

/// The resource section in [`ResourceExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceSection {
    /// `geometry` in `geometry.foo`
    Geometry,
    /// `material` in `material.foo`
    Material,
    /// `texture` in `texture.foo`
    Texture,
}

impl ResourceSection {
    /// String representation of the resource section ("geometry", "material", or "texture").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Geometry => "geometry",
            Self::Material => "material",
            Self::Texture => "texture",
        }
    }
}

impl From<Kind> for ResourceSection {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Geometry => Self::Geometry,
            Kind::Material => Self::Material,
            Kind::Texture => Self::Texture,
            _ => unreachable!("Resource Section: {kind:?}"),
        }
    }
}

/// <https://bedrock.dev/docs/stable/Molang#Array%20Expressions>
///
/// `array.foo[0]`
#[derive(Debug, Clone)]
pub struct ArrayAccessExpression {
    pub span: Span,
    pub name: IdentifierReference,
    pub index: Expression,
}

/// <https://bedrock.dev/docs/stable/Molang#-%3E%20%20Arrow%20Operator>
///
/// `v.foo->q.bar`
#[derive(Debug, Clone)]
pub struct ArrowAccessExpression {
    pub span: Span,
    pub left: Expression,
    pub right: Expression,
}

/// <https://bedrock.dev/docs/stable/Molang#Lexical%20Structure>
/// <https://bedrock.dev/docs/stable/Molang#Math%20Functions>
///
/// `math.random(1, 2)` or `math.random`
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub span: Span,
    pub kind: CallKind,
    pub callee: IdentifierReference,
    pub arguments: Option<Vec<Expression>>,
}

impl CallKind {
    /// String representation of the call kind ("math" or "query").
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Math => "math",
            Self::Query => "query",
        }
    }
}

impl From<Kind> for CallKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Math => Self::Math,
            Kind::Query => Self::Query,
            _ => unreachable!("Call Kind: {kind:?}"),
        }
    }
}

/// The call kind for [`CallExpression`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallKind {
    /// `math` in `math.foo`
    Math,
    /// `query` in `query.foo`
    Query,
}

/// <https://bedrock.dev/docs/stable/Molang#loop>
///
/// `loop(10, { v.x = v.x + 1; });`
#[derive(Debug, Clone)]
pub struct LoopExpression {
    pub span: Span,
    pub count: Expression,
    pub expression: BlockExpression,
}

/// <https://bedrock.dev/docs/stable/Molang#for_each>
///
/// `for_each(t.foo, q.baz, { v.x = v.x + 1; });`
#[derive(Debug, Clone)]
pub struct ForEachExpression {
    pub span: Span,
    pub variable: VariableExpression,
    pub array: Expression,
    pub expression: BlockExpression,
}

/// <https://bedrock.dev/docs/stable/Molang#break>
///
/// `break` in `loop(10, { v.x = v.x + 1; (v.x > 20) ? break; });`
#[derive(Debug, Clone)]
pub struct Break {
    pub span: Span,
}

/// <https://bedrock.dev/docs/stable/Molang#continue>
///
/// `continue` in `loop(10, { (v.x > 5) ? continue; v.x = v.x + 1; });`
#[derive(Debug, Clone)]
pub struct Continue {
    pub span: Span,
}

/// `this` in `q.foo(this)`
#[derive(Debug, Clone)]
pub struct This {
    pub span: Span,
}

/// `return` in `v.a = 1; return v.a;`
#[derive(Debug, Clone)]
pub struct Return {
    pub span: Span,
    pub argument: Expression,
}
