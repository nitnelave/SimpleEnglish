mod execution;
mod parse;
mod resolution;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    let input = r#"
To greet the world:
  Display "Hello World!".

To start:
  Greet the world.
"#;
    let tree = parse::parse(input);
    println!("Parsing: {}", input);
    println!("Parse tree: {:?}", tree);
    let ast = resolution::resolve(tree.unwrap());
    println!("Resolved tree: {:?}", ast);
    execution::execute(ast.unwrap()).unwrap();
}
