//node for min heap
struct Node<'a> {
    //specific char
    char: Option<char>,

    //frequency of the char
    //note OPTIONAL
    freq: u32,

    //left child
    //note OPTIONAL
    left: Option<&'a Node<'a>>,

    //right child
    //note OPTIONAL
    right: Option<&'a Node<'a>>
}



//min heap struct
struct MinHeap<'a> {
    //current size of the heap
    size: usize,

    //array of nodes in the tree
    //root will be at nodes[0]
    //for any ith node at node[i]
        //arr[(i-1)/2] = parent node
        //arr[(2*i)+1] = left child node
        //arr[(2*i)+2] = right child node
    nodes: Vec<Node<'a>>
}


    //returns a new node
    fn create_node<'a>(char: Option<char>, freq: u32, left: Option<&'a Node<'a>>, right: Option<&'a Node<'a>>) -> Node<'a>{
            Node {
                char,
                freq,
                left,
                right
            }
    }

    //adds the passed in node to the min heap vector and heapifys it
    fn add_heap_node<'a>(heap: &mut MinHeap<'a>, node_to_add: Node<'a>){
        heap.nodes.push(node_to_add);
    }

    //heapifys a single node
    //based off of GFG heapify function
    fn heapify(heap: &mut MinHeap, index: usize){
        //Indices of relevant roots
        let mut smallest: usize = index;
        let left: usize = 2 * index + 1;
        let right: usize = 2 * index + 2;
        let heap_size: usize = heap.size;

        //if left child is smaller than root
        if left < heap_size && heap.nodes[left].freq < heap.nodes[smallest].freq{
            smallest = left;
        }

        //if right child is smaller than smallest so far
        if right < heap_size && heap.nodes[right].freq < heap.nodes[smallest].freq{
            smallest = right;
        }

        //if smallest is not root
        if smallest != index{
            heap.nodes.swap(index, smallest);
            heapify(heap,smallest)
        }
        }

        //creates a minimum heap out of unsorted node vector
        //based off of GFG buildHeap function
        fn build_min_heap(heap: &mut MinHeap){
            //start at index of last non-leaf
            let n: usize = heap.size;

            //heapify each node, starting at the last non leaf node
            //start at index (n-1)/2
                //find better way to iterate in rust
            for i in (0..((n-1)/2)+1).rev(){
                heapify(heap,i);
            }
        }

    // }

//builds the huffman tree out of the passed in min heap
    //steps by GFG
    //1.Extract two nodes with the minimum freq from the min heap
    //2.Create a new internal node wtih a freq equal to the sum of the two extracted nodes freqs.
    //  Make the first extracted node as its left child and the other extracted node as its right child.
    //  Add this node to the min heap
    //3. Repeat steps 1 and 2 until the heap contains only one node. The remaining node is the root node and the tree is complete.

fn build_huffman_tree(heap: &mut MinHeap){
    //todo
}   


//depth first search to find chars
//add a 0 when going to the left node
//add a 1 when going to the right node
    //when a char is found, the string of 0's and 1's will be its code, so print current char and string slice
    fn depth_first_search_print(current_node: &Node, code: &mut String){
        //There exists a left child
        if current_node.left.is_some() {
                //find better way to do this
            code.insert(code.len(), '0');
            depth_first_search_print(current_node.left.unwrap(), code);
        }
            
        //There exists a left child
        if current_node.right.is_some(){
            //find better way to do thi
            code.insert(code.len(), '1');
            depth_first_search_print(current_node.right.unwrap(), code);
        }
    
        //Reached a leaf, so print it's char and code
        println!("{}, {}", current_node.char.unwrap(), code);
    }

//prints the huffman codes for the input data
fn huffman_codes(chars: &[char], freq: &[u32]){
    //builds a min heap from the data
    let mut heap = MinHeap {
        size: chars.len(),
        nodes: vec![]
    };

    //fills the heap's vector with nodes
    for i in (0..heap.size){
        add_heap_node(&mut heap, create_node(Some(chars[i]), freq[i], None, None));
    }

    //build a min heap
    build_min_heap(&mut heap);


    //creates a huffman tree from the min heap
    build_huffman_tree(&mut heap);

    //prints the huffman codes
    let mut code = String::from("");
    depth_first_search_print(&heap.nodes[0], &mut code);


}

fn main(){
    //example input data
    let input_data: [char; 7] = ['a', 'u', 'c', 'd', 'b', 'f', 'y'];
    let freq: [u32; 7] = [32, 100, 50, 37, 1, 8, 46];
    huffman_codes(&input_data, &freq);
}