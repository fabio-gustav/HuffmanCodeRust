//node for huffman tree
struct Node {
    //specific char
    char: char,

    //frequency of the char
    freq: u32,
}

//min heap struct
struct MinHeap {
    //current size of the heap
    size: usize,

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

    //heapifys a single node
    //based off of GFG heapify function
    fn heapify(&mut self, index: usize){
        //Indices of relevant roots
        let mut smallest: usize = index;
        let left: usize = 2 * index + 1;
        let right: usize = 2 * index + 2;
        let heap_size: usize = self.size;

        //if left child is smaller than root
        if left < heap_size && self.nodes[left].freq < self.nodes[smallest].freq{
            smallest = left;
        }

        //if right child is smaller than smallest so far
        if right < heap_size && self.nodes[right].freq < self.nodes[smallest].freq{
            smallest = right;
        }

        //if smallest is not root
        if smallest != index{
            self.nodes.swap(index, smallest);
            self.heapify(smallest)
        }
        }

        //creates a minimum heap out of unsorted node vector
        //based off of GFG buildHeap function
        fn create_min_heap(&mut self){
            //start at index of last non-leaf
            let n: usize = self.size;

            //heapify each node, starting at the last non leaf node
            //start at index (n-1)/2
            for i in (0..((n-1)/2)+1).rev(){
                self.heapify(i);
            }
        }

    }

//builds the huffman tree out of the passed in min heap
fn build_huffman_tree(heap: &mut MinHeap){
    
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