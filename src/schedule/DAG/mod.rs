use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;
use crate::{system::dependency::SystemMetadata, world::unsafe_world::UnsafeWorldContainer};

use super::Schedulable;

pub(crate) struct GraphNode {
    // Typeids of nodes which are dependent on this.
    // This map is to be used to reduce the indegree of dependent nodes
    // once the execution of this node completes
    // dependency_map: HashSet<std::any::TypeId>,

    // dependency_index_map: Vec<usize>, // dependency map based on index of graph node
    dependency_metadata: SystemMetadata,

    // The system to be executed by this node.
    system: Box<dyn Schedulable>,
}

impl GraphNode {
    fn check_dependency_conflict(&self, m2: &GraphNode) -> bool {
        self.dependency_metadata.is_resource_clashing(&m2.dependency_metadata)
    }
}

// @TODO: Define @SAFETY
unsafe impl Send for GraphNode{}
unsafe impl Sync for GraphNode{}

pub(crate) struct DependencyGraph {
    nodes: Vec<GraphNode>,
    indegrees: Vec<usize>,

    // (index) i1 - (index) i2 arr mapping
    // i2 is dependent on i1
    graph_edges: HashMap<usize, Vec<usize>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            indegrees: vec![],
            graph_edges: HashMap::new(),
        }
    }

    // @DONE: Find a way to identify position of a created node in the existing graph
    // @SOLN: Try insertion at a level where indegree is zero in a topological sort algo process

    pub fn add_boxed_system(&mut self, mut system: Box<dyn Schedulable>) {
        // 1. Construct the node from the given system
        let dependency_metadata = system.initialise_dependency_metadata();
        let graph_node = GraphNode {
            // dependency_map: hashbrown::HashSet::new(),
            // dependency_index_map: vec![],
            dependency_metadata,
            system,
        };

        // 2. Add the node to the graph after identifing the dependencies in the graph
        self.check_dependency_and_insert(graph_node);
    }
}

impl DependencyGraph {
    fn check_dependency_and_insert(&mut self, new_node: GraphNode) {
        let mut new_node_indegree = 0;
        let new_node_index = self.nodes.len();

        let mut sorted_vec: Vec<i32> = self.indegrees.clone().into_iter().map(|i| i as i32).collect();
        let mut visited_count = 0;

        println!("NODE: {}", self.nodes.len());
        println!("SORTED: {:?}", sorted_vec);
        
        while visited_count < self.nodes.len() {
            // println!("{visited_count}");
            
            // 1. Construct a topologically sorted graph, and create layers where each
            // layer consists of nodes with indegree of zero
            let top_nodes: Vec<usize> = sorted_vec
                .iter_mut()
                .enumerate()
                .filter_map(|(index, item)| {
                    if *item == 0 { 
                        *item = -1; // Dont come back to this node in the next iteration
                        visited_count += 1;
                        return Some(index as usize) 
                    }
                    None
                }).collect();
            
                
            let mut is_conflicting = false;
                
            // 2. Iterate over each layer.
            // Going through the nodes with indegree = 0 in the current iteration.
            for node_index in &top_nodes {
                let node = &mut self.nodes[*node_index];

                if node.check_dependency_conflict(&new_node) {
                    // Pushing the new node as a dependency of the present node
                    // node.dependency_index_map.push(new_node_index);
                    self.graph_edges.get_mut(node_index).unwrap().push(new_node_index);

                    is_conflicting = true;
                    new_node_indegree += 1;
                }
            }

            // 3. Check for dependencies and insert accordingly
            // Can execute with the current level, insert the system
            if !is_conflicting {
                break;
            }
            
            println!("{:?}", self.graph_edges);
            // Conflict found, cannot insert in the current level, move on to the next one
            for node_index in top_nodes {
                println!("{} {:?}", node_index, self.graph_edges.get(&node_index).unwrap());
                for child_node_index in self.graph_edges.get(&node_index).unwrap() {
                    if *child_node_index == new_node_index {
                        continue;
                    }
                    sorted_vec[*child_node_index] -= 1;
                }
            }
        }

        // Computation complete, graph has been updated to accomodate the node.
        // We can now insert the node and record its calculated indegree
        self.nodes.push(new_node);
        self.indegrees.push(new_node_indegree);
        self.graph_edges.insert(new_node_index, vec![]);
    }

    pub fn execute_system_graph(&mut self, world: &UnsafeWorldContainer) {
        let mut sorted_vec: Vec<i32> = self.indegrees.clone().into_iter().map(|i| i as i32).collect();
        let mut visited_count = 0;

        let node_count = self.nodes.len();
        // println!("Dependency graph has {node_count} nodes");
        
        // while visited_count < self.nodes.len() {
        while visited_count < node_count {
            let mut rest = self.nodes.as_mut_slice();
            // 1. Construct a topologically sorted graph, and create layers where each
            // layer consists of nodes with indegree of zero

            let mut top_nodes: Vec<usize> = sorted_vec
                .iter_mut()
                .enumerate()
                .filter_map(|(index, item)| {
                    if *item == 0 { 
                        *item = -1; // Dont come back to this node in the next iteration
                        visited_count += 1;

                        return Some(index as usize) 
                    }
                    None
                }).collect();

            let mut top_nodes_vec: Vec<&mut GraphNode> = vec![];
            let mut slice_count = 0;
            for index in &top_nodes {
                let (first, last) = rest.split_at_mut(*index - slice_count + 1);
                rest = last;                
                slice_count += first.len();
                top_nodes_vec.push( &mut first[first.len()-1] )
            }

            top_nodes_vec.into_par_iter().for_each(|item| {
                
                if let Some(res) = item.system.initialise_dependencies(world) {
                    // Initialisation failed. Resource does not exist. Do not run the system
                    return;
                }
                item.system.run();
            });

            // move on to the next layer execution
            for node_index in top_nodes {
                for child_node_index in self.graph_edges.get(&node_index).unwrap() {
                    sorted_vec[*child_node_index] -= 1;
                }
            }
        }
    }
}
