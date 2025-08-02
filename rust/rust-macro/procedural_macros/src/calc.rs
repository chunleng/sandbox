use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    LitInt, parenthesized,
    parse::{Parse, ParseStream},
    token::{Minus, Plus, Slash, Star},
};

// any_expression: expression | expression_with_bracket
// expression_with_bracket: l_paren any_expression r_paren
// expression: literal (operator any_expression)?

pub enum AnyExpression {
    Expression(Expression),
    BracketExpression(BracketExpression),
}
impl Parse for AnyExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(x) = BracketExpression::parse(input) {
            Ok(AnyExpression::BracketExpression(x))
        } else {
            Ok(AnyExpression::Expression(input.parse::<Expression>()?))
        }
    }
}
impl ToTokens for AnyExpression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            AnyExpression::Expression(x) => x.to_tokens(tokens),
            AnyExpression::BracketExpression(x) => x.to_tokens(tokens),
        }
    }
}

pub struct Expression {
    literal: Literal,
    operator: Option<Operator>,
    expression: Option<Box<AnyExpression>>,
}
impl Parse for Expression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let literal = input.parse::<Literal>()?;
        let operator = Operator::parse(input).ok();
        let expression = match operator {
            Some(_) => Some(input.parse()?),
            None => None,
        };

        Ok(Self {
            literal,
            operator,
            expression,
        })
    }
}
impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            literal,
            operator,
            expression,
        } = &self;
        let output = quote! {
            #literal #operator #expression
        };
        tokens.extend(output)
    }
}

pub struct BracketExpression {
    expression: Box<AnyExpression>,
}
impl Parse for BracketExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        _ = parenthesized!(content in input);
        Ok(Self {
            expression: content.parse()?,
        })
    }
}
impl ToTokens for BracketExpression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expression = &self.expression;
        quote! { ( #expression ) }.to_tokens(tokens)
    }
}

struct Literal(LitInt);
impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0
            .base10_parse::<i32>()
            .expect("Parsing error")
            .to_tokens(tokens);
    }
}

enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}
impl Parse for Operator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Plus) {
            _ = input.parse::<Plus>();
            Ok(Operator::Plus)
        } else if input.peek(Minus) {
            _ = input.parse::<Minus>();
            Ok(Operator::Minus)
        } else if input.peek(Star) {
            _ = input.parse::<Star>();
            Ok(Operator::Times)
        } else if input.peek(Slash) {
            _ = input.parse::<Slash>();
            Ok(Operator::Divide)
        } else {
            Err(input.error("Error parsing operator"))
        }
    }
}
impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Operator::Plus => quote! {+},
            Operator::Minus => quote! {-},
            Operator::Times => quote! {*},
            Operator::Divide => quote! {/},
        }
        .to_tokens(tokens)
    }
}
