use newton_rs::Source;
use newton_rs::lexer::token::*;
use newton_rs::lexer::lexer::*;
use newton_rs::parser::span::*;

fn main() {
    let source = Source::new("main", "
    fn main(argc: i32, argv: []string) => i32 {
        return 1;
    }
    ");

    let lexer = Lexer::new(&source);
    let tokens: Vec<Spanned<TokenType>> = lexer.map(std::result::Result::unwrap).collect();

    for token in tokens {
        println!("{}", token.node)
    }
}
