use std::fmt::Debug;

/// The error returned by the `Connection<T>` constructor
/// when given invalid input
enum ConnectionError {
    /// The start and end nodes are the same node (a node can't be connected to itself)
    StartIsEnd,
    /// Neither the start node nor the end node are in the `Vec` of given nodes
    NodesNotFound,
    /// The start node wasn't in the `Vec` of given nodes, but the end node was
    StartNotFound,
    /// The end node wasn't in the `Vec` of given nodes, but the start node was
    EndNotFound,
}

#[derive(Debug)]
pub struct Connection<T: Debug> {
    start: usize,
    end: usize,
    data: T,
}
impl<T: Debug> Connection<T> {
    fn new<N>(nodes: &Vec<N>, start: &N, end: &N, data: T) -> Result<Self, ConnectionError> {
        if std::ptr::eq(start as *const N, end as *const N) {
            Err(ConnectionError::StartIsEnd)
        } else {
            let start = nodes
                .iter()
                .enumerate()
                .find(|(_, x)| std::ptr::eq(start as *const N, *x as *const N));
            let end = nodes
                .iter()
                .enumerate()
                .find(|(_, x)| std::ptr::eq(end as *const N, *x as *const N));
            match (start, end) {
                (Some((start, _)), Some((end, _))) => Ok(Self { start, end, data }),
                (Some(_), None) => Err(ConnectionError::EndNotFound),
                (None, Some(_)) => Err(ConnectionError::StartNotFound),
                (None, None) => Err(ConnectionError::NodesNotFound),
            }
        }
    }
}
impl<T: Debug + Clone> Clone for Connection<T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
            data: self.data.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Path<N: Debug, C: Debug> {
    nodes: Vec<N>,
    connections: Vec<Connection<C>>,
}
impl<N: Debug, C: Debug> Path<N, C> {}
impl<N: Debug + Clone, C: Debug + Clone> Clone for Path<N, C> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            connections: self.connections.clone(),
        }
    }
}
