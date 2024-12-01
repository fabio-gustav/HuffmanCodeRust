use std::collections::HashMap;

//takes in a root of a huffman tree and hashmap mutable reference 
//and assigns huffman codes to the specific chars
fn assign_codes(current_node: &Box<Node>, codes: &mut HashMap<char, i32>, current_code: String){
    //left == add 0
    //right == add 1

    //there is a left node from the current node, so add 0 to the code and recurse from that node
    if (current_node.left.is_some()){
        current_code.push('0');
        assign_codes(&current_node.left, codes, current_code);
    }

    //there is a right node from the current node, so add 1 to the code and recurse from that node
    if (current_node.right.is_some()){
        current_code.push('1');
        assign_codes(&current_node.right, codes, current_code);
    }

    //found a potential leaf node, check
    if(current_node.ch.is_some()){
        //found char, so add it to the vector along with the current code
        codes.insert(current_node.ch.unwrap(), current_node.freq);
    }
}

//encodes a specific message, encode it by concatenating all of the huffman codes for 
//each specific char
//returns that encoded string
fn encode(input: &str, hash_map: HashMap<char, i32>) -> String{
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
fn decode(to_decode: String, head: Box<Node>) -> String{
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
        
        //next num is a 0, so traverse left in the tree
        if (char == '0'){
            current_node = &current_node.left;
        }

        //next num is a 1, so traverse right in the tree
        else{
            current_node = &current_node.right;
        }

    }

    //return the completed string
    decoded_string

}

fn main(){

}
