//node for huffman tree
struct Node <'a>{
    //specific char
    char: char,

    //frequency of the char
    freq: u32,

    //reference to the left node
    left_node: &'a mut Node<'a>,

    //reference to the right node
    right_node: &'a mut Node<'a>
}

//min heap struct
struct MinHeap<'a> {
    //current size
    size: u32,
    
    //capacity
    capacity: u32,

    //array of nodes in the tree
    nodes: Vec<&'a mut Node>
}

//function that allocates a new node given its char and freq and adds it to the heap
fn add_new_node(data: char, freq: u32, tree: &'a mut MinHeap<'a>){
    //todo
}

//creates a min heap for a given capacity
//returns ownership to a minHeap
fn create_min_heap(capacity: u32) -> MinHeap<'a>{
    //todo
}

//swaps two min heap nodes
fn swap_min_heap_node(node1: &'a mut Node<'a>, node2: &'a mut Node<'a>){
    //todo
}

//heapify the tree
fn min_heapify(tree: &'a mut MinHeap<'a>, index: u32){
    //todo
}

//extracts minimum value node from heap
fn extract_min(heap: &'a mut MinHeap<'a>) -> &'a mut Node<'a> {
    //todo
}

//builds the min heap
fn build_min_heap(heap: &'a mut MinHeap<'a>){
    //todo
}

//creats a min heap of capacity equal to size
//inserts all chars of data[] in min heap
fn create_and_build_min_heap(data: Vec<char>, freq: Vec<u32>, size: u32)->&'a mut MinHeap<'a>{
    //todo
}

//builds huffman tree
//todo
fn build_huffman_tree()

//builds and prints huffman tree
//todo
fn HuffmanCodes()

fn main(){
    //example input data
    let input_data: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
}