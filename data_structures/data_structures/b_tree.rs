use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

// Why to need a different Struct for props...
// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
    where
        T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node{
            keys: match _keys{
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree-1),
            },
            children: match _children {
                Some(_childeren) =>_childeren,
                None => Vec::with_capacity(degree),
            },
        }
    }
    // 리프노드인지 확인 (자식 노드가 없는 노드)
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps{
            degree,
            max_keys : degree-1,
            mid_key_index : (degree-1) / 2,
        }
    }

    //이거 사용용도가 뭐지?
    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    // Split Child expects the Child Node to be full
    /// Move the middle_key to parent node and split the child_node's
    /// keys/chilren_nodes into half
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        //왜 이렇게 구현?
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first(){
            Some((_first, _others)) => {
                _others.to_vec()
            },
            None => Vec::with_capaticy(self.max_keys),
        };
        let right_children = if !child.is_leaf() {
            //leaf 노드가 아니면 splift_off 결과 반환??
            Some(child.children.split_off(self.mid_key_index+1))
        }else{
            None
        };
        let new_child_node: Node<T> =  Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index+1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() -1;
        while  index>=0 && node.keys[index as usize] >=key{
            index -=1;
        }

        let mut u_index:usize = usize::try_from(index+1).ok().unwrap();
        if node.is_leaf(){
            //리프노드이면, 그냥 인서트
            node.keys.insert(u_index, key);
        }else{
            //리프노드가 아니면
            if self.is_maxed_out(&node.children[u_index]){
                self.split_child(node, u_index);
                if node.keys[u_index] < key{
                    u_index+=1;
                }
            }
        }
    }

    fn traverse_node<T: Ord + Debug>(&self, node: &Node<T>, depth: usize) {

    }
}

impl<T> BTree<T>
    where
        T: Ord + Copy + Debug + Default,
{
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        BTree {
            root:Node::new(degree, None,None),
            props: BTreeProps::new(degree),
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root){
            //empty root를 생성하고 old root를 나누기
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root,0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    pub fn traverse(&self) {
        self.props.traverse_node(&self.root, 0);
        println!();
    }

    pub fn search(&self, key: T) -> bool {
        let mut current_node = &self.root;
        let mut index: isize;
        loop{
            index = isize::try_from(current_node.keys.len()).ok().unwrap() -1;
            while index >=0 && current_node.keys[index as usize]>key{
                index -=1;
            }

            let u_index: usize = usoze::isize::try_from(index+1).ok().unwrap() ;
            if index >=0 && current_node.keys[u_index-1] ==key {
                break true;
            }else if current_node.is_leaf(){
                break false;
            }else{
                current_node = &current_node.children[u_index];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn test_search() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(11);
        tree.insert(12);
        tree.insert(15);
        assert!(tree.search(15));
        assert_eq!(tree.search(16), false);
    }
}