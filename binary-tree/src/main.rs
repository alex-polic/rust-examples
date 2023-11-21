use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    let mut root = create_node(4);
    add_node(&mut root, 10);
    add_node(&mut root, 3);
    add_node(&mut root, 1);
    add_node(&mut root, 6);
    add_node(&mut root, 20);

    traverse_and_print(&root);
}

#[derive(PartialEq)]
struct BTreeNode {
    value: i32,
    left: Option<Box<BTreeNode>>,
    right: Option<Box<BTreeNode>>
}

fn create_node(init_value: i32) -> BTreeNode {
    BTreeNode{ value: init_value, left: None, right: None }
}

fn add_node(node: &mut BTreeNode, new_value: i32) {
    let mut queue: VecDeque<&mut BTreeNode> = VecDeque::new();
    queue.push_front(node);
    loop {
        let BTreeNode {
            ref mut left,
            ref mut right,
            value
        } = queue.pop_back().unwrap();

        if new_value < *value {
            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(create_node(new_value)));
                    return;
                }
            }
        }

        if new_value > *value {

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(create_node(new_value)));
                    return;
                }
            }
        }
    }
}

fn traverse_and_print(node: &BTreeNode) {
    println!("{}", node.value);

    if node.left.is_none() == false {
        traverse_and_print(node.left.as_deref().unwrap());
    }

    if node.right.is_none() == false {
        traverse_and_print(node.right.as_deref().unwrap());
    }
}

