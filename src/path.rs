use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeIndex(usize);
impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    start: NodeIndex,
    end: NodeIndex, // I don't like this
}
impl From<(usize, usize)> for Connection {
    fn from(value: (usize, usize)) -> Self {
        Self {
            start: NodeIndex(value.0),
            end: NodeIndex(value.1),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Node {
    position: Vector,
    g_cost: Option<f32>,
    h_cost: Option<f32>, // this should only be evaluated once
    parent: Option<NodeIndex>,
}
impl Node {
    /// Gets the h_cost, without reevaluating it
    /// if it's already been evaluated once.
    fn h_cost_calculate(&mut self, target: &Node) -> f32 {
        if let Some(h_cost) = self.h_cost {
            h_cost
        } else {
            let cost = (target.position - self.position).length();
            self.h_cost = Some(cost);
            cost
        }
    }

    fn h_cost(&self) -> f32 {
        if let Some(h_cost) = self.h_cost {
            h_cost
        } else {
            panic!("The h_cost of this node hasn't been calculated!");
        }
    }

    /// # Panics
    /// Panics if the parent node does not have a g_cost
    fn set_g_cost(&mut self, parent: &Node, parent_index: NodeIndex) {
        self.parent = Some(parent_index);
        self.g_cost = Some(parent.g_cost.unwrap() + (parent.position - self.position).length());
    }
}
impl Node {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            g_cost: 0.0.into(),
            h_cost: None,
            parent: None,
        }
    }

    #[inline(always)]
    pub fn f_cost_calculate(&mut self, target: &Node) -> Option<f32> {
        self.g_cost
            .map(|g_cost| g_cost + self.h_cost_calculate(target))
    }

    #[inline(always)]
    pub fn f_cost(&self) -> Option<f32> {
        self.g_cost.map(|g_cost| g_cost + self.h_cost())
    }
}

struct Pathfinder {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    best_route: Option<Vec<NodeIndex>>,
}
impl Pathfinder {
    pub fn new(nodes: Vec<Vector>, connections: Vec<Connection>) -> Option<Self> {
        let mut final_connections = Vec::with_capacity(connections.len());
        if nodes.len() == 0
            || connections.iter().any(
                |Connection {
                     start: NodeIndex(start),
                     end: NodeIndex(end),
                 }| *start >= nodes.len() || *end >= nodes.len(),
            )
        {
            None
        } else {
            for potential_connection in connections.into_iter() {
                if !final_connections
                    .iter()
                    .any(|connection| *connection == potential_connection)
                {
                    final_connections.push(potential_connection);
                }
            }
            Some(Self {
                nodes: nodes
                    .into_iter()
                    .map(|position| Node::new(position))
                    .collect(),
                connections: final_connections,
                best_route: None,
            })
        }
    }

    pub fn pathfind(&mut self, start: NodeIndex, end: NodeIndex) {
        // everything added to `open` must have a calculated g_cost
        let mut open = vec![];
        let mut closed = vec![];
        self.nodes[start.0].g_cost = Some(0.0);
        let target = self.nodes[end.0].clone();
        self.nodes[start.0].h_cost_calculate(&target);
        open.push(start);

        loop {
            let current_index = open
                .iter()
                .fold(None, |acc, el| {
                    if acc.is_some_and(|acc: NodeIndex| {
                        self.nodes[acc.0].f_cost() < self.nodes[el.0].f_cost()
                    }) {
                        acc
                    } else {
                        Some(*el)
                    }
                })
                .unwrap();
            let current = open.remove(current_index.0);
            closed.push(current);

            if current == end {
                // TODO: construct path and return it
                return;
            }

            for neighbour in self.neighbours(current_index) {
                if !closed.contains(&neighbour) {
                    let current_node = &self.nodes[current.0];
                    let neighbour_node = &self.nodes[neighbour.0];
                    let new_path_g_cost = current_node.g_cost.unwrap()
                        + (current_node.position - neighbour_node.position).length();
                    let current_g_cost = neighbour_node.g_cost;

                    let neighbour_in_open = open.contains(&neighbour);
                    if current_g_cost.is_none()
                        || current_g_cost.unwrap() > new_path_g_cost
                        || !neighbour_in_open
                    {
                        let parent = self.nodes[current.0].clone();
                        self.nodes[neighbour.0].set_g_cost(&parent, current_index);
                        if !neighbour_in_open {
                            self.nodes[neighbour.0].h_cost_calculate(&target);
                            open.push(neighbour);
                        }
                    }
                }
            }
        }
    }

    fn neighbours<'a>(&'a mut self, node_index: NodeIndex) -> Vec<NodeIndex> {
        self.connections
            .iter()
            .filter(|x| x.start.0 == node_index.0)
            .map(|x| x.end)
            .collect()
    }
}
