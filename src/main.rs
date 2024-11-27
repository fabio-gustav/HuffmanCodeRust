//node for huffman tree
struct Node {
    //specific char
    char: char,

    //frequency of the char
    freq: u32,
}

//min heap struct
struct MinHeap {
    //current size
    size: u32,
    
    //capacity
    capacity: u32,

    //array of nodes in the tree
    //root will be at nodes[0]
    //for any ith node at node[i]
        //arr[(i-1)/2] = parent node
        //arr[(2*i)+1] = left child node
        //arr[(2*i)+2] = right child node
    nodes: Vec<Node>
}


impl MinHeap{
    //adds a node to the unsorted node vector with char its freq
    fn create_node(&mut self, char: char, freq: u32) -> Node{
            Node {
                char,
                freq
            }
    }

    //adds the passed in node to the min heap vector
    fn add_node(&mut self, node_to_add: Node){
        self.nodes.push(node_to_add)
    }

    //sorts all of the nodes in the array so that they represent a min heap
    //after this is done, self.nodes will be a min heap
    fn heapify(&mut self){
        //todo
    }

}

//creates a minimum heap out of unsorted node vector
fn create_min_heap(heap: &mut MinHeap){
    //todo
}

//builds the huffman tree out of the passed in min heap
fn build_huffman_tree(heap: &mut MinHeap){
    //todo
}


//prints the huffman codes for the input data
fn huffman_codes(chars: &mut [char], freq: &mut [u32]){
    //todo
}

fn main(){
    //example input data
    let input_data: [char; 7] = ['a', 'u', 'c', 'd', 'b', 'f', 'y'];
    let freq: [i32; 7] = [32, 100, 50, 37, 1, 8, 46];
}