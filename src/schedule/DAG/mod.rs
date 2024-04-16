use std::borrow::BorrowMut;

use crate::{system::dependency::SystemMetadata, world::unsafe_world::UnsafeWorldContainer};
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

use super::{DependentSystems, Schedulable};

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
        self.dependency_metadata
            .is_resource_clashing(&m2.dependency_metadata)
    }
}

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
        let graph_node = Self::initialise_system_node(system);

        // 2. Add the node to the graph after identifing the dependencies in the graph
        let mut indegree_vec: Vec<i32> = self.indegrees.clone().iter_mut().map(|i| *i as i32).collect();
        let _ = self.check_indegree_and_insert(graph_node, indegree_vec);
    }

    pub fn add_dependent_systems(&mut self, dependent: DependentSystems) {
        assert!(dependent.systems.len() > 1);

        let mut sys_iter= dependent.systems.into_iter();
        let first_system_node = Self::initialise_system_node(sys_iter.next().unwrap());
        
        // Inserting the parent system
        let mut indegree_vec: Vec<i32> = self.indegrees.clone().iter_mut().map(|i| *i as i32).collect();
        indegree_vec = self.check_indegree_and_insert(first_system_node, indegree_vec);

        // The parent system is guarenteed to be present at the last index of the nodes array
        let mut parent_index = self.nodes.len()-1;

        while let Some(system) = sys_iter.next() {
            let new_node = Self::initialise_system_node(system);
            // 1. Getting the indegree vector of the next iteration.
            indegree_vec = self.check_indegree_and_insert(new_node, indegree_vec);
            
            // 2. Find the place of the child node in the graph following the top layer and insert it
            //      based on its dependencies
            let child_node_index = self.nodes.len() - 1;

            // 3. Enforce the dependency of child on parent by
            //      - inserting the dependency of child in parent edge vector
            //      - incrementing the indegree of the child node
            self.indegrees[child_node_index] += 1;
            self.graph_edges.get_mut(&parent_index).unwrap().push(child_node_index);


            // Repeat the process for the next ordered system
            parent_index = child_node_index;
        }
    }
}

impl DependencyGraph {
    fn initialise_system_node(mut system: Box<dyn Schedulable>) -> GraphNode {
        let dependency_metadata = system.initialise_dependency_metadata();
        GraphNode {
            // dependency_map: hashbrown::HashSet::new(),
            // dependency_index_map: vec![],
            dependency_metadata,
            system,
        }
    }

    /// Gets the indegree vector and inserts the node accordingly in the correct position
    /// The indegree vector should be structured in the following manner
    ///     - indegree = -1 => Node has been visited in a previous layer
    ///     - indegree = 0 => Node is in the first layer of graph
    ///     - indegree > 0 => Node is in some layer after the first layer of graph
    fn check_indegree_and_insert(&mut self, new_node: GraphNode, mut sorted_vec: Vec<i32>) -> Vec<i32> {
        let mut new_node_indegree = 0;
        let new_node_index = self.nodes.len();

        loop {
            // 1. Construct a topologically sorted graph, and create layers where each
            // layer consists of nodes with indegree of zero
            let top_nodes: Vec<usize> = sorted_vec
                .iter_mut()
                .enumerate()
                .filter_map(|(index, item)| {
                    if *item == 0 {
                        *item = -1; // Dont come back to this node in the next iteration

                        return Some(index as usize);
                    }
                    None
                })
                .collect();

            if top_nodes.len() == 0 {
                break;
            }

            let mut is_conflicting = false;

            // 2. Iterate over each layer.
            // Going through the nodes with indegree = 0 in the current iteration.
            for node_index in &top_nodes {
                let node = &mut self.nodes[*node_index];

                if node.check_dependency_conflict(&new_node) {
                    // Pushing the new node as a dependency of the present node
                    // node.dependency_index_map.push(new_node_index);
                    self.graph_edges
                        .get_mut(node_index)
                        .unwrap()
                        .push(new_node_index);

                    is_conflicting = true;
                    new_node_indegree += 1;
                }
            }

            
            // Conflict found, cannot insert in the current level, move on to the next one
            for node_index in top_nodes {
                for child_node_index in self.graph_edges.get(&node_index).unwrap() {
                    if *child_node_index == new_node_index {
                        continue;
                    }
                    sorted_vec[*child_node_index] -= 1;
                }
            }

            // 3. Check for dependencies and insert accordingly
            // Can execute with the current level, insert the system
            if !is_conflicting {
                break;
            }
        }

        // Computation complete, graph has been updated to accomodate the node.
        // We can now insert the node and record its calculated indegree
        self.nodes.push(new_node);
        self.indegrees.push(new_node_indegree);
        self.graph_edges.insert(new_node_index, vec![]);

        // Pushing the current iteration indegree of the newly created node
        sorted_vec.push(-1);
        sorted_vec
    }

    pub fn execute_system_graph(&mut self, world: &UnsafeWorldContainer) {
        let mut sorted_vec: Vec<i32> = self
            .indegrees
            .clone()
            .into_iter()
            .map(|i| i as i32)
            .collect();
        let mut visited_count = 0;

        let node_count = self.nodes.len();

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

                        return Some(index as usize);
                    }
                    None
                })
                .collect();

            let mut top_nodes_vec: Vec<&mut GraphNode> = vec![];
            let mut slice_count = 0;
            for index in &top_nodes {
                let (first, last) = rest.split_at_mut(*index - slice_count + 1);
                rest = last;

                // offset for future indexes
                slice_count += first.len();
                top_nodes_vec.push(&mut first[first.len() - 1])
            }

            let world_ref = world.get_world();

            top_nodes_vec.into_par_iter().for_each(|item| {
                if let Some(res) = item.system.initialise_dependencies(world_ref) {
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
