//Reads a string of characters from an input file and outputs its huffman code to an output file

//imports
use std::collections::HashMap;
mod frequency;
use std::fs;
use std::io::Write;


//huffman node struct
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

    //returns a new node, setting its character and freq to the passed in values 
    fn create_node<'a>(char: Option<char>, freq: i32) -> Node{
        Node {
            freq: freq,
            character: char,
            left: None,
            right: None
        }
}


//encodes a specific passed in message by concatenating all of the huffman codes for 
//each specific char in the message
//returns the encoded message as a String
fn encode(input: &str, hash_map: &HashMap<char, String>) -> String {
    let mut encoded_string: String = "".to_string();
    
    //for every char in the string, look up its corresponding code in the hashmap table
    //and add that code to the output string
    for char in input.chars(){
        encoded_string.push_str(&hash_map[&char].to_string());
    }

    //returns the encoded string
    encoded_string

    //returns a huffman tree

}

//builds a huffman tree out of a list of huffman nodes 
//when this function returns, vector that is pointer to by the passed in refernce will only contain the root of the huffman tree
fn build_huffman_tree(huffman_nodes:&mut Vec<Box<Node>>){
    
    //continue making the tree until only the root remains in the passed in vector
    while huffman_nodes.len() > 1 {
        
        //sorts the nodes in descending order
        huffman_nodes.sort_by(|node_a,node_b| (&(node_b.freq)).cmp(&(node_a.freq)));

        //pops the two minimum values from the vector of huffman nodes
        let node_a = huffman_nodes.pop().unwrap();
        let node_b = huffman_nodes.pop().unwrap();

        //create a new huffman node that will go onto the huffman tree
        //its freq equals the previously popped minimum nodes freqs' added together
        let mut node_c = Box::new(create_node(None,node_a.freq + node_b.freq));

        //assigns the two previously popped minimum nodes as the left and right of the
        //newly created Huffman node
        node_c.left = Some(node_a);
        node_c.right = Some(node_b);

        //adds the newly created huffman tree node back to the vector
        huffman_nodes.push(node_c);

    }  
}

//decodes a specific message given the encoded string and root of the huffman tree
//each leaf of the huffman tree contains a char
fn decode(to_decode: String, root: &Box<Node>) -> String{
    //go left if the next char is a 0
    //go right if the next char is a 1
    
    //holds a reference to the current node in the tree traversal
    let mut current_node: &Box<Node> = &root;

    //holds the current decoded string
    let mut decoded_string: String = "".to_string();

    //loops through the input data
    for char in to_decode.chars(){
        //if the current node contains a char, it's a leaf node so add it to 
        //the string and re-traverse the tree starting from the root
        
        //next num is a 0, so traverse left in the tree
        if char == '0'{
            current_node = current_node.left.as_ref().unwrap();
        }

        //next num is a 1, so traverse right in the tree
        else{
            current_node = current_node.right.as_ref().unwrap();
        }

        //current node is a leaf node, so push its char to the returned string
        //and restart from the root
        if current_node.character.is_some(){
            decoded_string.push(current_node.character.unwrap());
            current_node = &root;
            
        }

    }

    //return the completed string

    decoded_string

}

//assigns huffman codes to specific chars in the string
fn assign_huffman_codes(assign_node:&Box<Node>,codes:&mut HashMap<char, String>,encoded_value:String){
    
    if let Some(character) = assign_node.character {
        codes.insert(character,encoded_value);
    }
    
    else {

        if let Some(ref left_node) = assign_node.left {
            assign_huffman_codes(left_node, codes, encoded_value.clone() + "0");
        }
        if let Some(ref right_node) = assign_node.right {
            assign_huffman_codes(right_node, codes, encoded_value + "1");
        }
    }


}


fn main() {

    //reads from a input file that contains the input string
    let input: String = fs::read_to_string("input.txt").expect("Unable to read file");

    //creats a hashmap of the frequency of each char in the input string
    let map = frequency::frequency(input.as_str());

    //creats a vector of huffman nodes 
    let mut huffman_nodes:Vec<Box<Node>> = map.iter().map(|nodes| Box::new(create_node(Some(*nodes.0), *nodes.1))).collect();
    
    //builds the huffman tree from the huffman nodes
    build_huffman_tree(&mut huffman_nodes);

    //gets the root of the huffman tree
    let root_node = &huffman_nodes.pop().unwrap();

    //assigns huffman codes to the corresponding chars in the string
    let mut huffman_codes:HashMap<char, String> = HashMap::new();
    assign_huffman_codes(root_node, &mut huffman_codes,"".to_string());
    
    //encodes the input string
    let encoded_string:String = encode(&input, &huffman_codes);

    //creates a new output file and writes the encoded string to it 
    //if the output file is already created, overwrite the existing data within it
    let mut output_file: fs::File = fs::File::create("output.txt").expect("File already exists");
    output_file.write_all(&encoded_string.as_bytes()).expect("Could not write to file");

    //use the decode function if you would like to decode the string you just encoded
    //decode(encoded_string, root_node);


}
