//node for huffman tree
struct Node<'a> {
    char: char,
    freq: u32,
    left_node: &'a mut Node<'a>,
    right_node: &'a mut Node<'a>,
}

//min heap struct
struct MinHeap<'a> {
    //current size
    size: u32,
    
    //capacity
    capacity: u32,

    //array of nodes in the tree
        //need to find a valid format that works like the one here but doesnt give an error
    nodes: [&'a mut Node<'a>; capacity]
}

//function that allocates a new node given its char and freq
//returns a reference to that new node
fn new_node(data: char, freq: u32) -> &'static Node<'static>{
    &Node{
        char:data,
        freq: freq,
        left_node: None,
        right_node: None,
    }
}

//creats a min heap for a given capacity
//returns ownership to a minHeap
fn create_min_heap(capacity: u32) -> MinHeap{
    MinHeap{
        size: 0,
        capacity: capacity,
        
    }
}


fn main(){
    //example input data
    let input_data: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
}