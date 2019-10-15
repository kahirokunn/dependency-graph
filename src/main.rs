use std::collections::HashMap;

type FilePath = String;
type DependencyDict = HashMap<FilePath, Vec<FilePath>>;

fn resolve_dependency(root_file_name: &String, dep_dict: &DependencyDict) {
    let mut stack = Vec::new();
    _resolve_dependency(&mut stack, root_file_name, dep_dict);
    println!("{:?}", stack);
}

fn _resolve_dependency(stack: &mut Vec<FilePath>, src: &FilePath, dep_dict: &DependencyDict) {
    stack.push((*src).clone());
    println!("stack trace: {:?}", stack);
    match dep_dict.get(src) {
        None => {
            println!("{:?} is a reef", src);
        },
        Some(dependencies) => {
            for child_path in dependencies {
                _resolve_dependency(stack, child_path, dep_dict);
            }
        },
    };
    stack.pop();
}

fn main() {
    let mut graph: DependencyDict = HashMap::new();

    graph.insert(String::from("src/index.ts"), vec![
        String::from("src/b.ts"),
        String::from("src/c.ts")
    ]);

    graph.insert(String::from("src/b.ts"), vec![
        String::from("src/b1.ts"),
        String::from("src/b2.ts")
    ]);

    graph.insert(String::from("src/b2.ts"), vec![
        String::from("src/b21.ts"),
        String::from("src/b22.ts")
    ]);

    graph.insert(String::from("src/c.ts"), vec![
        String::from("src/c1.ts"),
        String::from("src/c2.ts")
    ]);

    resolve_dependency(&String::from("src/index.ts"), &graph);
}
