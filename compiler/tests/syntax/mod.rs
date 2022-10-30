use compiler::syntax::lex::Lexer;

const TEST_SCRIPT: &'static str = r#"
<?php
use rsphp\Mixed;

class Foo {
    private $bar;

    public function __construct(Mixed $bar) {
        $this->bar = $bar;
    }

    public function bar() {
        return "baz";
    }
}

$foo = new Foo("bar");
echo $foo->bar();
"#;

#[test]
pub fn intitial_lex() {
    let mut lexer = Lexer::new(TEST_SCRIPT);

    loop {
        let token = lexer.next();
        if let Ok(x) = token {
            if x.is_none() {
                println!("EOF???");
            }
            println!("{:?}", x);
        } else {
            println!("Parse error: {:?}", token);
            break;
        }
    }
}
