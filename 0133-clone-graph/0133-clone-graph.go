/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */

func cloneGraph(node *Node) *Node {
    if node == nil {
        return nil
    }

    visited := make(map[*Node]*Node)
    // Create a clone for the starting node and add it to the visited map.
    visited[node] = &Node{Val: node.Val, Neighbors: []*Node{}}
    
    // Initialize a queue with the original graph's starting node.
    queue := []*Node{node}
    
    for len(queue) > 0 {
        // Dequeue a node from the front of the queue.
        current := queue[0]
        queue = queue[1:]
        
        // Process the neighbors of the current node.
        for _, neighbor := range current.Neighbors {
            if _, ok := visited[neighbor]; !ok {
                // If the neighbor hasn't been visited, create a clone, and enqueue the neighbor.
                visited[neighbor] = &Node{Val: neighbor.Val, Neighbors: []*Node{}}
                queue = append(queue, neighbor)
            }
            // Add the clone of the neighbor to the clone of the current node's neighbors list.
            visited[current].Neighbors = append(visited[current].Neighbors, visited[neighbor])
        }
    }
    
    // Return the clone of the starting node.
    return visited[node]
}