use newton_rs::Source;
use newton_rs::lexer::token::*;
use newton_rs::lexer::lexer::*;
use newton_rs::parser::span::*;
use newton_rs::parser::parser::*;

fn main() {
    let source: Source<'static> = Source::new("main", "
    type Nullable<T> = T?;
    ");

    let lexer: Lexer<'static> = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
}
