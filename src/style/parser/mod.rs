pub mod ast;

use cssparser::{ParseError, Parser, Token};

pub enum Error {}

pub fn peek<'a, 'i>(
    parser: &'a mut Parser<'i, '_>,
) -> Result<&'a Token<'i>, ParseError<'i, Error>> {
    let state = parser.state();
    let tok = parser.next()?;
    parser.reset(&state);
    Ok(tok)
}

pub fn parse_function<'i>(
    parser: &mut Parser<'i, '_>,
) -> Result<ast::Function<'i>, ParseError<'i, Error>> {
    let name = parser.expect_function()?;
    let mut func = ast::Function::new(name);

    parser.parse_nested_block::<_, ast::Function, _>(move |parser| loop {
        match peek(parser)? {
            Token::CloseParenthesis => return Ok(func),
            _ => {
                let arg = parse_component_value(parser)?;
                func.args.push(arg);
            }
        }
    })
}

pub fn parse_block<'i>(parser: &mut Parser<'i, '_>) -> Result<ast::Block, ParseError<'i, Error>> {}

pub fn parse_component_value<'i>(
    parser: &mut Parser<'i, '_>,
) -> Result<ast::ComponentValue<'i>, ParseError<'i, Error>> {
    match peek(parser)? {
        Token::Function(id) => parse_function(parser).map(ast::ComponentValue::from),
        Token::CurlyBracketBlock => {
            parser.next()?;
            parser
                .parse_nested_block(parse_block)
                .map(ast::ComponentValue::from)
        }
        Token::ParenthesisBlock => {
            parser.next()?;
            parser
                .parse_nested_block(parse_block)
                .map(ast::ComponentValue::from)
        }
        Token::SquareBracketBlock => {
            parser.next()?;
            parser
                .parse_nested_block(parse_block)
                .map(ast::ComponentValue::from)
        }
        tok => {
            parser.next()?;
            Ok(ast::ComponentValue::from(tok.clone()))
        }
    }
}

pub fn parse_at_rule<'i>(
    parser: &mut cssparser::Parser<'i, '_>,
) -> Result<ast::AtRule<'i>, ParseError<'i, Error>> {
    let mut at_rule = ast::AtRule::default();

    loop {
        match peek(parser)? {
            Token::Semicolon => {
                parser.next()?;
                return Ok(at_rule);
            }
            Token::CurlyBracketBlock => {
                parser.next()?;
                at_rule.block = Some(parser.parse_nested_block(parse_block)?);
                return Ok(at_rule);
            }
            _ => {
                let component_value = parse_component_value(parser)?;
                at_rule.prelude.push(component_value);
            }
        }
    }
}

pub fn parse_rule<'i>(parser: &mut Parser<'i, '_>) -> Result<ast::Rule<'i>, ParseError<'i, Error>> {
    match peek(parser)? {
        Token::AtKeyword(_) => parse_at_rule(parser).map(ast::Rule::from),
        _ => todo!(),
    }
}

pub fn parse<'i>(content: &'i str) -> Result<ast::Sheet<'i>, ParseError<'i, Error>> {
    let mut ipt = cssparser::ParserInput::new(content);
    let mut parser = cssparser::Parser::new(&mut ipt);

    let mut sheet = ast::Sheet::default();

    while !parser.is_exhausted() {
        sheet.add_rule(parser.try_parse(parse_rule)?);
    }

    Ok(sheet)
}
