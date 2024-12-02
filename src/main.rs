use std::collections::HashMap;
mod frequency;

//node for min heap
struct Node {
    
    //specific char
    character: Option<char>,

    //frequency of the char
    //note OPTIONAL
    freq: i32,

    //left child
    //note OPTIONAL
    left: Option<Box<Node>>,

    //right child
    //note OPTIONAL
    right: Option<Box<Node>>,
}

    //returns a new node
    fn create_node<'a>(char: Option<char>, freq: i32) -> Node{
        Node {
            freq: freq,
            character: char,
            left: None,
            right: None
        }
}


//builds the huffman tree out of the passed in min heap
    //steps by GFG
    //1.Extract two nodes with the minimum freq from the min heap
    //2.Create a new internal node wtih a freq equal to the sum of the two extracted nodes freqs.
    //  Make the first extracted node as its left child and the other extracted node as its right child.
    //  Add this node to the min heap
    //3. Repeat steps 1 and 2 until the heap contains only one node. The remaining node is the root node and the tree is complete.


    //returns a huffman tree
fn build_huffman_tree( huffman_nodes:&mut Vec<Box<Node>>){
    while huffman_nodes.len() > 1 {

        huffman_nodes.sort_by(|node_a,node_b| (&(node_b.freq)).cmp(&(node_a.freq)));

        let node_a = huffman_nodes.pop().unwrap();
        let node_b = huffman_nodes.pop().unwrap();

        let mut node_c = Box::new(create_node(None,node_a.freq + node_b.freq));

        node_c.left = Some(node_a);
        node_c.right = Some(node_b);
        huffman_nodes.push(node_c);

    }  
}


//prints the huffman codes for the input data
fn assign_huffman_codes(assign_node:&Box<Node>,codes:&mut HashMap<char, String>,encoded_value:String){
    
    if let Some(character) = assign_node.character {
        codes.insert(character,encoded_value);
    }
    
    else {

        if let Some(ref left_node) = assign_node.left {
            assign_huffman_codes(left_node, codes, encoded_value.clone() + "1");
        }
        if let Some(ref right_node) = assign_node.right {
            assign_huffman_codes(right_node, codes, encoded_value + "0");
        }
    }


}

fn main(){

    let message:&str = "aabbccdde";
    //example input data
    let map = frequency::frequency(message);
    
    let mut huffman_nodes:Vec<Box<Node>> = map.iter().map(|nodes| Box::new(create_node(Some(*nodes.0), *nodes.1))).collect();
    
    build_huffman_tree(&mut huffman_nodes);


    let mut huffman_codes:HashMap<char, String> = HashMap::new();
    assign_huffman_codes(&huffman_nodes.pop().unwrap(), &mut huffman_codes,"".to_string()); 
    println!("{:?}", huffman_codes);

}