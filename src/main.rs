//node for huffman tree
struct Node<'a> {
    char: char,
    freq: u32,
    left_node: &'a Node<'a>,
    right_node: &'a Node<'a>,
}

struct Tree<'a>{
    head: &'a Node<'a>,
}

fn main(){
    //example input data
    let input_data: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
}