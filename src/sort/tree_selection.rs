//! 算法新解, 刘新宇
//! Version: 0.6180339887498949
//! 8.4 本质改进
//! 对 Selection sort的本质改进

use crate::tree::binary::traverse::PreOrderVisitor;
use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::cmp::max;

//build Tournament tree, from bottom to top
fn build_tournament_tree<T: Copy + std::cmp::Ord>(a: &[T]) -> Tree<T> {
    let mut tree = Tree::new();

    //build leaf
    let mut nodes = Vec::new();
    for v in a {
        let node = TreeNode::from_value(*v);
        let i = tree.add_node(node);
        nodes.push(i);
    }

    while nodes.len() > 1 {
        let mut new_nodes = Vec::new();
        for chunk in nodes.chunks(2) {
            match chunk {
                &[t1, t2] => {
                    //create node
                    let t1_node = tree.node_at(t1).expect("invalid t1");
                    let t2_node = tree.node_at(t2).expect("invalid t1");
                    let value = max(t1_node.value, t2_node.value);
                    let node = TreeNode::new(value, Some(t1), Some(t2), None);
                    let node_i = tree.add_node(node);

                    //set parent
                    let t1_node = tree.node_at_mut(t1).expect("invalid t1");
                    t1_node.parent = Some(node_i);
                    let t2_node = tree.node_at_mut(t2).expect("invalid t1");
                    t2_node.parent = Some(node_i);

                    new_nodes.push(node_i);
                }
                &[t] => new_nodes.push(t),
                _ => unreachable!(),
            }
        }

        nodes = new_nodes;
    }

    //tree.arena last is root
    let root = tree.arena.len().saturating_sub(1);
    tree.set_root(Some(root));

    tree
}

#[test]
fn t_build_tree() {
    /*
                                            16
                        /                                       \
                     16                                           14
                 /         \                                /           \
            16                   13                    10                    14
         /     \               /     \               /    \                /     \
       7         16         8          13        10          9         12         14
     /  \       /  \       /  \       /  \      /   \      /  \       /  \       /  \
    7    6    15   16     8    4    13    3    5    10    9    1     12   2    11   14

    */
    let a = &[7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let tree = build_tournament_tree(a);
    let r = PreOrderVisitor::recursive(&tree);
    assert_eq!(
        r,
        vec![
            16, 16, 16, 7, 7, 6, 16, 15, 16, 13, 8, 8, 4, 13, 13, 3, 14, 10, 10, 5, 10, 9, 9, 1,
            14, 12, 12, 2, 14, 11, 14
        ]
    );
}
