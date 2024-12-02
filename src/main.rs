use std::collections::HashMap;

mod frequency;

use std::fs;
use std::io::Write;


fn main() {
    

    //reads from a input file that contains the input string
    let input: String = fs::read_to_string("test.txt").expect("Unable to read file");

    //TODO: code to build huffman tree
    let map = frequency::frequency(input);
    
    let mut huffman_nodes:Vec<Box<Node>> = map.iter().map(|nodes| Box::new(create_node(Some(*nodes.0), *nodes.1))).collect();
    
    build_huffman_tree(&mut huffman_nodes);

    //assigns huffman codes to specific chars in the string
    let mut huffman_codes:HashMap<char, String> = HashMap::new();
    assign_huffman_codes(&huffman_nodes.pop().unwrap(), &mut huffman_codes,"".to_string());

    //encodes the input string
    let encoded_string:String = encode(&current_code, &codes);

    //writes the encoded string to an output file
    //creates a new output file
    let mut output_file: fs::File = fs::File::create("output.txt").expect("File already exists");
    output_file.write_all(encoded_string.as_bytes()).expect("Could not write to file");

    
    //use this function if you want to decode the string you just encoded
    //encode(input, &huffman_tree_head);


}



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


//encodes a specific message, encode it by concatenating all of the huffman codes for 
//each specific char
//returns that encoded string
fn encode(input: &str, hash_map: &HashMap<char, i32>) -> String{
    let mut encoded_string: String = "".to_string();
    
    //for every char in the string, look up its corresponding code in the hashmap table
    //and add that code to the output string
    for char in input.chars(){
        encoded_string.push_str(&hash_map[&char].to_string());
    }

    //returns the encoded string
    encoded_string

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

//decodes a specific message given the encoded string and head of the huffman tree
fn decode(to_decode: String, head: &Box<Node>) -> String{
    //go left if the next char is a 0
    //go right if the next char is a 1
    
    //holds a reference to the current node in the tree traversal
    let current_node: &Box<Node> = &head;

    //holds the current decoded string
    let decoded_string: String = "".to_string();

    //loops through the input data
    for char in to_decode.chars(){
        //if the current node contains a char, it's a leaf node so add it to 
        //the string and re-traverse the tree starting from the head
        if (current_node.character.is_some()){
            decoded_string.push(current_node.character.unwrap());
            current_node = &head
        }


        //next num is a 0, so traverse left in the tree
        if (char == '0'){
            current_node = current_node.left.as_ref().unwrap();
        }

        //next num is a 1, so traverse right in the tree
        else{
            current_node = current_node.right.as_ref().unwrap();
        }

    }

    //return the completed string
    decoded_string
  
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
