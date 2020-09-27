mod execution;
mod parse;
mod resolution;

fn main() {
    let tree = parse::parse(
        r#"
To greet the world:
  Display \"Hello World!\".

To start:
  Greet the world.
"#,
    );
    println!("Parse tree: {:?}", tree);
    let ast = resolution::resolve(tree.unwrap());
    println!("Resolved tree: {:?}", ast);
    execution::execute(ast.unwrap()).unwrap();
}
