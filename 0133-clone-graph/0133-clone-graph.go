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
    return dfs(node, visited)
}

func dfs(node *Node, visited map[*Node]*Node) *Node {
    // If the node has been visited, then return the clone from the visited map.
    if clone, ok := visited[node]; ok {
        return clone
    }
    
    // Clone the current node.
    clone := &Node{Val: node.Val, Neighbors: make([]*Node, len(node.Neighbors))}
    // Mark the current node as visited.
    visited[node] = clone
    
    // Process the neighbors.
    for i, neighbor := range node.Neighbors {
        clone.Neighbors[i] = dfs(neighbor, visited)
    }
    
    return clone
}