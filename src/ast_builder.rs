use crate::{ast::*, span::Span};

/// Builder for creating AST nodes.
#[derive(Clone, Copy)]
pub struct AstBuilder {}

impl AstBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn alloc<T>(self, value: T) -> Box<T> {
        Box::new(value)
    }

    #[inline]
    pub fn vec<T>(self) -> Vec<T> {
        Vec::new()
    }

    #[inline]
    pub fn program(self, span: Span, is_complex: bool, body: Vec<Expression>) -> Program {
        Program {
            span,
            is_complex,
            body,
        }
    }

    #[inline]
    pub fn identifier_reference<S>(self, span: Span, name: S) -> IdentifierReference
    where
        S: Into<String>,
    {
        IdentifierReference {
            span,
            name: name.into(),
        }
    }

    #[inline]
    pub fn expression_boolean_literal(self, span: Span, value: bool) -> Expression {
        Expression::BooleanLiteral(self.alloc(self.boolean_literal(span, value)))
    }

    #[inline]
    pub fn expression_numeric_literal(self, span: Span, value: f32) -> Expression {
        Expression::NumericLiteral(self.alloc(self.numeric_literal(span, value)))
    }

    #[inline]
    pub fn expression_string_literal<S>(self, span: Span, value: S) -> Expression
    where
        S: Into<String>,
    {
        Expression::StringLiteral(self.alloc(self.string_literal(span, value)))
    }

    #[inline]
    pub fn expression_variable(
        self,
        span: Span,
        lifetime: VariableLifetime,
        member: VariableMember,
    ) -> Expression {
        Expression::Variable(self.alloc(self.variable_expression(span, lifetime, member)))
    }

    #[inline]
    pub fn expression_parenthesized_single(self, span: Span, expression: Expression) -> Expression {
        Expression::Parenthesized(
            self.alloc(self.parenthesized_single_expression(span, expression)),
        )
    }

    #[inline]
    pub fn expression_parenthesized_complex(
        self,
        span: Span,
        expressions: Vec<Expression>,
    ) -> Expression {
        Expression::Parenthesized(
            self.alloc(self.parenthesized_complex_expression(span, expressions)),
        )
    }

    #[inline]
    pub fn expression_block(self, span: Span, expressions: Vec<Expression>) -> Expression {
        Expression::Block(self.alloc(self.block_expression(span, expressions)))
    }

    #[inline]
    pub fn expression_binary(
        self,
        span: Span,
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
    ) -> Expression {
        Expression::Binary(self.alloc(self.binary_expression(span, left, operator, right)))
    }

    #[inline]
    pub fn expression_unary(
        self,
        span: Span,
        operator: UnaryOperator,
        argument: Expression,
    ) -> Expression {
        Expression::Unary(self.alloc(self.unary_expression(span, operator, argument)))
    }

    #[inline]
    pub fn expression_ternary(
        self,
        span: Span,
        test: Expression,
        consequent: Expression,
        alternate: Expression,
    ) -> Expression {
        Expression::Ternary(self.alloc(self.ternary_expression(span, test, consequent, alternate)))
    }

    #[inline]
    pub fn expression_conditional(
        self,
        span: Span,
        test: Expression,
        consequent: Expression,
    ) -> Expression {
        Expression::Conditional(self.alloc(self.conditional_expression(span, test, consequent)))
    }

    #[inline]
    pub fn expression_assignment(
        self,
        span: Span,
        left: VariableExpression,
        right: Expression,
    ) -> Expression {
        Expression::Assignment(self.alloc(self.assignment_expression(span, left, right)))
    }

    #[inline]
    pub fn expression_resource(
        self,
        span: Span,
        section: ResourceSection,
        name: IdentifierReference,
    ) -> Expression {
        Expression::Resource(self.alloc(self.resource_expression(span, section, name)))
    }

    #[inline]
    pub fn expression_array_access(
        self,
        span: Span,
        name: IdentifierReference,
        index: Expression,
    ) -> Expression {
        Expression::ArrayAccess(self.alloc(self.array_access_expression(span, name, index)))
    }

    #[inline]
    pub fn expression_arrow_access(
        self,
        span: Span,
        left: Expression,
        right: Expression,
    ) -> Expression {
        Expression::ArrowAccess(self.alloc(self.arrow_access_expression(span, left, right)))
    }

    #[inline]
    pub fn expression_call(
        self,
        span: Span,
        kind: CallKind,
        callee: IdentifierReference,
        arguments: Option<Vec<Expression>>,
    ) -> Expression {
        Expression::Call(self.alloc(self.call_expression(span, kind, callee, arguments)))
    }

    #[inline]
    pub fn expression_loop(
        self,
        span: Span,
        count: Expression,
        expression: BlockExpression,
    ) -> Expression {
        Expression::Loop(self.alloc(self.loop_expression(span, count, expression)))
    }

    #[inline]
    pub fn expression_for_each(
        self,
        span: Span,
        variable: VariableExpression,
        array: Expression,
        expression: BlockExpression,
    ) -> Expression {
        Expression::ForEach(self.alloc(self.for_each_expression(span, variable, array, expression)))
    }

    #[inline]
    pub fn expression_break(self, span: Span) -> Expression {
        Expression::Break(self.alloc(self.r#break(span)))
    }

    #[inline]
    pub fn expression_continue(self, span: Span) -> Expression {
        Expression::Continue(self.alloc(self.r#continue(span)))
    }

    #[inline]
    pub fn expression_this(self, span: Span) -> Expression {
        Expression::This(self.alloc(self.this(span)))
    }

    #[inline]
    pub fn expression_return(self, span: Span, argument: Expression) -> Expression {
        Expression::Return(self.alloc(self.r#return(span, argument)))
    }

    #[inline]
    pub fn boolean_literal(self, span: Span, value: bool) -> BooleanLiteral {
        BooleanLiteral { span, value }
    }

    #[inline]
    pub fn numeric_literal(self, span: Span, value: f32) -> NumericLiteral {
        NumericLiteral { span, value }
    }

    #[inline]
    pub fn string_literal<S>(self, span: Span, value: S) -> StringLiteral
    where
        S: Into<String>,
    {
        StringLiteral {
            span,
            value: value.into(),
        }
    }

    #[inline]
    pub fn variable_expression(
        self,
        span: Span,
        lifetime: VariableLifetime,
        member: VariableMember,
    ) -> VariableExpression {
        VariableExpression {
            span,
            lifetime,
            member,
        }
    }

    #[inline]
    pub fn variable_member_object(
        self,
        span: Span,
        object: VariableMember,
        property: IdentifierReference,
    ) -> VariableMember {
        VariableMember::Object {
            span,
            object: Box::new(object),
            property,
        }
    }

    #[inline]
    pub fn variable_member_property(
        self,
        span: Span,
        property: IdentifierReference,
    ) -> VariableMember {
        VariableMember::Property { span, property }
    }

    #[inline]
    pub fn parenthesized_single_expression(
        self,
        span: Span,
        expression: Expression,
    ) -> ParenthesizedExpression {
        ParenthesizedExpression::Single { span, expression }
    }

    #[inline]
    pub fn parenthesized_complex_expression(
        self,
        span: Span,
        expressions: Vec<Expression>,
    ) -> ParenthesizedExpression {
        ParenthesizedExpression::Complex { span, expressions }
    }

    #[inline]
    pub fn block_expression(self, span: Span, expressions: Vec<Expression>) -> BlockExpression {
        BlockExpression { span, expressions }
    }

    #[inline]
    pub fn binary_expression(
        self,
        span: Span,
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
    ) -> BinaryExpression {
        BinaryExpression {
            span,
            left,
            operator,
            right,
        }
    }

    #[inline]
    pub fn unary_expression(
        self,
        span: Span,
        operator: UnaryOperator,
        argument: Expression,
    ) -> UnaryExpression {
        UnaryExpression {
            span,
            operator,
            argument,
        }
    }

    #[inline]
    pub fn ternary_expression(
        self,
        span: Span,
        test: Expression,
        consequent: Expression,
        alternate: Expression,
    ) -> TernaryExpression {
        TernaryExpression {
            span,
            test,
            consequent,
            alternate,
        }
    }

    #[inline]
    pub fn conditional_expression(
        self,
        span: Span,
        test: Expression,
        consequent: Expression,
    ) -> ConditionalExpression {
        ConditionalExpression {
            span,
            test,
            consequent,
        }
    }

    #[inline]
    pub fn assignment_expression(
        self,
        span: Span,
        left: VariableExpression,
        right: Expression,
    ) -> AssignmentExpression {
        AssignmentExpression { span, left, right }
    }

    #[inline]
    pub fn resource_expression(
        self,
        span: Span,
        section: ResourceSection,
        name: IdentifierReference,
    ) -> ResourceExpression {
        ResourceExpression {
            span,
            section,
            name,
        }
    }

    #[inline]
    pub fn array_access_expression(
        self,
        span: Span,
        name: IdentifierReference,
        index: Expression,
    ) -> ArrayAccessExpression {
        ArrayAccessExpression { span, name, index }
    }

    pub fn arrow_access_expression(
        self,
        span: Span,
        left: Expression,
        right: Expression,
    ) -> ArrowAccessExpression {
        ArrowAccessExpression { span, left, right }
    }

    #[inline]
    pub fn call_expression(
        self,
        span: Span,
        kind: CallKind,
        callee: IdentifierReference,
        arguments: Option<Vec<Expression>>,
    ) -> CallExpression {
        CallExpression {
            span,
            kind,
            callee,
            arguments,
        }
    }

    #[inline]
    pub fn loop_expression(
        self,
        span: Span,
        count: Expression,
        expression: BlockExpression,
    ) -> LoopExpression {
        LoopExpression {
            span,
            count,
            expression,
        }
    }

    #[inline]
    pub fn for_each_expression(
        self,
        span: Span,
        variable: VariableExpression,
        array: Expression,
        expression: BlockExpression,
    ) -> ForEachExpression {
        ForEachExpression {
            span,
            variable,
            array,
            expression,
        }
    }

    #[inline]
    pub fn r#break(self, span: Span) -> Break {
        Break { span }
    }

    #[inline]
    pub fn r#continue(self, span: Span) -> Continue {
        Continue { span }
    }

    #[inline]
    pub fn this(self, span: Span) -> This {
        This { span }
    }

    #[inline]
    pub fn r#return(self, span: Span, argument: Expression) -> Return {
        Return { span, argument }
    }
}
