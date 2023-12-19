use std::collections::HashMap;

type NodeId = u32;

struct Node {
  id: NodeId,
  edges: Vec<NodeId>
}


struct Graph {
  nodes: HashMap<NodeId, Node>,
  next_id: NodeId
}

fn create_node(graph: &mut Graph) -> NodeId {
  let id = graph.next_id;
  let node = Node { id, edges: Vec::new() };
  graph.nodes.insert(id, node);
  graph.next_id += 1;
  return id;
}

fn get_node(graph: &Graph, id: NodeId) -> &Node {
  return graph.nodes.get(&id).expect("Node not found");
}

fn add_edge(graph: &mut Graph, id1: NodeId, id2: NodeId) -> () {
  graph.nodes.get_mut(&id1).expect("Node not found").edges.push(id2);
  graph.nodes.get_mut(&id2).expect("Node not found").edges.push(id1);
}

fn get_root(graph: &Graph) -> &Node {
  return get_node(graph, 0)
}

fn find_furthest(graph: &Graph) -> u32 {
  let root = get_root(graph);
  let mut distances: HashMap<u32, u32> = HashMap::new();
  let mut queue = root.edges.clone();
  distances.insert(root.id, 0);
  while !queue.is_empty() {
    let node_id = queue.pop().expect("Queue is empty");
    let node = get_node(graph, node_id);
    for new_node_id in node.edges.clone() {
      if !distances.contains_key(&new_node_id) {
        let distance = distances.get(&node_id).expect("Node not found");
        distances.insert(new_node_id, distance + 1);
        queue.push(new_node_id);
      }
    }
  }
  distances.values().max().unwrap_or(&0).clone()
  // return max_node;
}

fn day_ten() -> u32 {
  let mut graph = Graph { nodes: HashMap::new(), next_id: 0 };
  let root = create_node(&mut graph);
  let one = create_node(&mut graph);
  let two = create_node(&mut graph);
  let three = create_node(&mut graph);
  let four = create_node(&mut graph);
  let five = create_node(&mut graph);
  let six = create_node(&mut graph);
  let seven = create_node(&mut graph);
  add_edge(&mut graph, root, one);
  add_edge(&mut graph, root, two);
  add_edge(&mut graph, one, three);
  add_edge(&mut graph, three, five);
  add_edge(&mut graph, five, seven);
  add_edge(&mut graph, two, four);
  add_edge(&mut graph, four, six);
  add_edge(&mut graph, six, seven);
  return find_furthest(&graph)
}