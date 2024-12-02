use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
    

    //reads from a input file that contains the input string
    let input: String = fs::read_to_string("test.txt").expect("Unable to read file");

    //TODO: code to build huffman tree


    //assigns huffman codes to specific chars in the string
    let current_code:String = String::new();
    let mut codes:HashMap<char, i32> = HashMap::new();
    assign_codes(&huffman_tree_head, &mut codes, current_code);

    //encodes the input string
    let encoded_string:String = encode(&current_code, &codes);

    //writes the encoded string to an output file
    //creates a new output file
    let mut output_file: fs::File = fs::File::create("output.txt").expect("File already exists");
    output_file.write_all(encoded_string.as_bytes()).expect("Could not write to file");

    
    //use this function if you want to decode the string you just encoded
    //encode(input, &huffman_tree_head);


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

}