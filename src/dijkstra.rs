//! Types and traits used in Dijkstra's algorithm.

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

use num::Zero;

/// Pathing information gathered by [`Graph::dijkstra`].
///
/// This object contains minimum distances from a starting node, as well as a
/// table of of previous nodes for each step in the graph. This object only has
/// meaning when returned from [`Graph::dijkstra`], as the values contained are
/// generated based on inputs to that method.
pub struct Traversal<T: Graph> {
    /// The starting  node of the traversal.
    ///
    /// All values in `distances` are the minimum distance from this node.
    pub from: T::Node,
    /// The destination node of the traversal.
    ///
    /// Stored in case the input `to` node has custom equality or hashing logic,
    /// thus it might not be able to index into the `distances`/`previous` maps.
    pub to: T::Node,
    distances: HashMap<T::Node, T::Distance>,
    previous: HashMap<T::Node, Option<T::Node>>,
}

impl<T: Graph> Traversal<T> {
    fn new(from: T::Node, to: T::Node) -> Self {
        Self {
            from,
            to,
            distances: HashMap::new(),
            previous: HashMap::new(),
        }
    }

    /// Returns a map of the shortest distances for nodes in the [`Graph`] starting at `from`.
    pub fn distances(&self) -> &HashMap<T::Node, T::Distance> {
        &self.distances
    }

    /// Returns a map of previous nodes for each step taken in the [`Graph`].
    ///
    /// This map can be used to reconstruct the full path by reverse lookup of
    /// each value as the next key, starting with `to`. For example:
    ///
    /// - `previous[to]` returns `Z`
    /// - `previous[Z]` returns `Y`
    /// - ...
    /// - `previous[A]` returns `from`
    /// - `previous[from]` returns [`None`]
    ///
    /// The above will create the shortest distance path as:
    ///
    /// ```txt
    ///     from -> A -> ... -> Y -> Z -> to
    /// ```
    ///
    /// Note that it is not guaranteed [`Traversal::from`] is the same as the
    /// input `from` to [`Graph::dijkstra`] if the node type has different logic
    /// for hashing and equality.
    pub fn previous(&self) -> &HashMap<T::Node, Option<T::Node>> {
        &self.previous
    }
}

/// A type that can be pathed by [Dijkstra's algorithm].
///
/// At a high level, the algorithm requires:
///
/// - A starting node or *vertex*
/// - A way to get *neighbors* for a given vertex `V` (e.g., a *vertex*
///   connected by a single *edge*)
/// - A way to measure *distance* between two verticies `V` and `U` (e.g., a
///   *weighted edge*)
///
/// Implementors of this trait can abstract the above concepts however
/// appropriate. For example, a two-dimensional grid can represent *vertices* as
/// coordinates, and cell values (the "cost" of moving into the cell) can be
/// returned as *edges*.
///
/// [Dijkstra's algorithm]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
pub trait Graph: Sized {
    /// The type used to represent nodes or *vertices* in the *graph*.
    type Node: Clone + Eq + Hash;

    /// The type used to represent edge weights.
    type Distance: Clone + Ord + Add + Zero;

    /// Executes Dijkstra's algorithm and returns traversal information.
    ///
    /// This method will short circuit once finding the `to` node. Matching this
    /// node relies on [`Graph::nodes_eq`], whereas visited information relies
    /// on the `Node` type's [`Hash`] implementation.
    ///
    /// Implementation adapted from: <https://codereview.stackexchange.com/a/202879>
    fn dijkstra(&self, from: Self::Node, to: Self::Node) -> Traversal<Self> {
        let mut map = Traversal::new(from.clone(), to.clone());
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        // using Zero trait allows us to generalize this for any distance type.
        // we also don't need to worry about "infinity" since the map will
        // return None for missing keys -- we can use that as "infinity" and not
        // have to further restrict the distance type (or use some faulty value
        // like usize::MAX)
        map.distances.insert(from.clone(), Self::Distance::zero());
        map.previous.insert(from.clone(), None);

        // using the Visit type here allows us to generalize "node with
        // distance", control the Ord logic for a min-heap, and not have to
        // require the Node type to provide its own distance value
        queue.push(Visit(from, Self::Distance::zero()));
        while let Some(Visit(node, dist)) = queue.pop() {
            if Self::nodes_eq(&node, &to) && self.is_done(&node, &to) {
                map.to = node;
                return map;
            }
            if !visited.insert(node.clone()) {
                continue;
            }

            for n in self.adjacent(&node) {
                let d = dist.clone() + self.edge(&node, &n);

                // using map_or here handles a None in the distance map -- this
                // is equivalent to "infinity" in the algorithm
                if map.distances.get(&n).map_or(true, |dist| &d < dist) {
                    // IMPORTANT: we need to ensure the value is overwritten for
                    // keys that can be `==` (equal) but have different hash values
                    map.distances.remove(&n);
                    map.previous.remove(&n);
                    map.distances.insert(n.clone(), d.clone());
                    map.previous.insert(n.clone(), Some(node.clone()));
                }
                // IMPORTANT: this neighbor needs to be added to the queue
                // regardless of the updated distance. this is a consequence of
                // allowing custom "equality" logic.
                queue.push(Visit(n, d));
            }
        }

        map
    }

    /// Returns the minimum distance between nodes `from` and `to`, starting at
    /// `from`.
    ///
    /// Minimum distance is determined by the [`PartialOrd`] implementation for
    /// the [`Graph::Distance`] type, specifically the `<` operator. However,
    /// this behavior can be changed by using a wrapper type with custom logic.
    /// Note that whatever logic is used, all invariants for [`Ord`] must also
    /// be upheld.
    fn min_distance<T, U>(&self, from: T, to: U) -> Self::Distance
    where
        T: Into<Self::Node>,
        U: Into<Self::Node>,
    {
        let from: Self::Node = from.into();
        let to: Self::Node = to.into();
        let map = self.dijkstra(from, to.clone());

        map.distances[&map.to].clone()
    }

    /// Returns a list of neighboring nodes, given an input `node` (e.g., all
    /// other *vertices* connected to `node` by a single *edge*).
    ///
    /// Implementors may include additional filtering or constraints on movement
    /// in this method (for example, preventing movement in a certain direction,
    /// constraining the maximum distance a neighbor can have, etc.).
    ///
    /// Every node returned by this method must be a valid input to
    /// [`Graph::edge`], as both `from` and `to` inputs.
    fn adjacent(&self, node: &Self::Node) -> Vec<Self::Node>;

    /// Returns the *edge* (e.g., weight or distance) between adjacent nodes
    /// `from` and `to`.
    ///
    /// In the case of a directed graph, `from` is the starting position. In the
    /// case of a grid-like structure, the `to` cell value will typically be the
    /// *edge* in question.
    ///
    /// The input nodes `from` and `to` are guaranteed to be adjacent, so long
    /// as the [`Graph::adjacent`] implementation is sound.
    ///
    /// # Warning
    ///
    /// Because of the nature of the algorithm that "settles" previous nodes
    /// (e.g., marking them as "closed" once all neighbors are visited), the
    /// `edge` method must always provide `0` or a positive number. In a
    /// mathematical sense, for every value `D` returned by `edge`, it must hold
    /// that `D + D_prev <= result`.
    fn edge(&self, from: &Self::Node, to: &Self::Node) -> Self::Distance;

    /// Returns whether or not nodes are considered equal for purposes of ending
    /// the algorithm.
    ///
    /// This serves as a custom equality method that can be implemented outside
    /// a node's type. This also allows custom logic that can be encapsulated
    /// within the context of this trait and not affect other behavior (such as
    /// [`Hash`] or [`Ord`]).
    ///
    /// This method uses `==` for the [`Graph::Node`] type by default.
    fn nodes_eq(lhs: &Self::Node, rhs: &Self::Node) -> bool {
        lhs.eq(rhs)
    }

    /// Returns whether or not the algorithm is allowed to stop on a destination
    /// node match with [`Graph::nodes_eq`].
    ///
    /// This allows for some final checks on the algorithm implemented outside
    /// of the node type. The `current` node is the last node removed from the
    /// top of the min-heap, while the `to` node is the input received from the
    /// original invocation.
    ///
    /// By default, this method simply returns `true`.
    fn is_done(&self, _current: &Self::Node, _to: &Self::Node) -> bool {
        true
    }
}

/// Wrapper type for node-distance pairs that implements min-heap [`Ord`] logic
/// for [`BinaryHeap`].
struct Visit<T, D>(T, D);

impl<T, D: Ord> Ord for Visit<T, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // this is intentionally backwards so BinaryHeap acts as a min-heap
        other.1.cmp(&self.1)
    }
}

impl<T, D: Ord> PartialOrd for Visit<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, D: PartialEq> PartialEq for Visit<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T, D: PartialOrd> Eq for Visit<T, D> {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGraph;

    impl Graph for TestGraph {
        type Node = char;
        type Distance = usize;

        // form a simple diamond graph:
        //     A
        //  5 /  \ 2
        //   B-10-C
        //  1 \  / 2
        //     D

        fn adjacent(&self, node: &Self::Node) -> Vec<Self::Node> {
            match *node {
                'A' => vec!['B', 'C'],
                'B' => vec!['A', 'C', 'D'],
                'C' => vec!['A', 'B', 'D'],
                'D' => vec!['B', 'C'],
                _ => panic!("unknown node `{node}`"),
            }
        }

        fn edge(&self, from: &Self::Node, to: &Self::Node) -> Self::Distance {
            match (*from, *to) {
                ('A', 'B') | ('B', 'A') => 5,
                ('A', 'C') | ('C', 'A') => 2,
                ('B', 'C') | ('C', 'B') => 10,
                ('B', 'D') | ('D', 'B') => 1,
                ('C', 'D') | ('D', 'C') => 2,
                _ => panic!("unknown edge `{from}`->`{to}`"),
            }
        }
    }

    #[test]
    fn dijkstra_distances() {
        let g = TestGraph;
        let map = g.dijkstra('A', 'D');
        assert_eq!(map.distances[&'A'], 0);
        assert_eq!(map.distances[&'B'], 5);
        assert_eq!(map.distances[&'C'], 2);
        assert_eq!(map.distances[&'D'], 4);
    }

    #[test]
    fn dijkstra_min_distance() {
        let g = TestGraph;
        assert_eq!(g.min_distance('A', 'D'), 4);
    }
}
