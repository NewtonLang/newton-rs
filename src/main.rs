use newton_rs::lexer::lexer::*;
use newton_rs::parser::parser::*;
use newton_rs::Source;

fn main() {
    let source: Source = Source::new(
        "main",
        "
    type Pair struct<K, V> {
        @key: K;
        @value: V;

        fn init(self: *Pair, key: K, value: V) => Pair {
            self.key = key;
            self.value = value;
        };

        fn get_key(self: *Pair) => K {
            return self.key;
        };

        fn get_value(self: *Pair) => V {
            return self.value;
        };
    }
    ",
    );

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    for toplevel in program.0 {
        println!("{:?}", toplevel)
    }
}
