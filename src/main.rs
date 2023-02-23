use newton_rs::Source;
use newton_rs::lexer::lexer::*;
use newton_rs::parser::parser::*;

fn main() {
    let source: Source = Source::new(
        "main",
        "
    type Pair struct<K, V> {
        @key: K;
        @value: V;

        fn init(self: &Pair, key: K, value: V) => Pair {
            return new Pair {
                key,
                value
            };
        };

        fn get_key(self: &Pair) => K {
            return self.key;
        };

        fn get_value(self: &Pair) => V {
            return self.value;
        };
    }

    type test struct {
        @unsized_array: [?]i32;
        @sized_array: [64]i32;
    }

    type Nullable<T> = ?T;
    ",
    );

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    for toplevel in program.0 {
        println!("{:?}", toplevel)
    }
}
