mod execution;
mod parse;
mod resolution;

fn main() {
    let tree = parse::parse(
        "To start:
  Display \"Hello World!\"
    ",
    );
    println!("{:?}", tree);
    let ast = resolution::resolve(tree.unwrap());
    println!("{:?}", ast);
    execution::execute(ast.unwrap()).unwrap();
}
