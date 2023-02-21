use newton_rs::lexer::lexer::*;
use newton_rs::parser::parser::*;
use newton_rs::Source;

fn main() {
    let source: Source = Source::new(
        "main",
        "
    // You can now declare structs and enumerators!
    type Vec2 struct {
        @somefield: i32;
        @someotherfield: i32;

        fn init() => Vec2 {};
    }

    type Colors enum: u64 {}
    ",
    );

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    for toplevel in program.0 {
        println!("{:?}", toplevel)
    }
}
