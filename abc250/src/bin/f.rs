enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

fn print_tree(t: Tree<i32>) {
    match t {
        Tree::Leaf(v) => {
            println!("{}", v)
        }
        Tree::Node(l, v, r) => {
            print_tree(*l);
            println!("{}", v);
            print_tree(*r);
        }
    }
}

fn main() {
    let t: Tree<i32> = Tree::Node(
        Box::new(Tree::Leaf(0)),
        3,
        Box::new(Tree::Node(
            Box::new(Tree::Leaf(9)),
            8,
            Box::new(Tree::Leaf(239)),
        )),
    );
    print_tree(t);
}
