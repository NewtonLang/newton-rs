use newton_rs::Source;
use newton_rs::lexer::lexer::*;
use newton_rs::parser::parser::*;

fn main() {
    let source: Source = Source::new("main", "
    type Vec2 struct {
        x: i64,
        y: i64
    }

    type Vec3 struct {
        x: i64,
        y: i64,
        z: i64
    }
    ");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    for toplevel in program.0 {
        println!("{:?}", toplevel)
    }
}
