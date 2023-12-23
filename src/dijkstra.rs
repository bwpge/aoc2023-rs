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
/// meaning when returned from [`Graph::dijkstra`], as the lookup tables are
/// generated based on inputs to that method.
pub struct Traversal<T: Graph> {
    distances: HashMap<<T::Node as Vertex>::EqTy, T::Distance>,
    previous: HashMap<<T::Node as Vertex>::EqTy, Option<<T::Node as Vertex>::VisitTy>>,
}

impl<T: Graph> Traversal<T> {
    fn new() -> Self {
        Self {
            distances: HashMap::new(),
            previous: HashMap::new(),
        }
    }

    /// Returns a map of the shortest distances for node taken in the [`Graph`].
    pub fn distances(&self) -> &HashMap<<T::Node as Vertex>::EqTy, T::Distance> {
        &self.distances
    }

    /// Returns a map of previous nodes for each step taken in the [`Graph`].
    pub fn previous(
        &self,
    ) -> &HashMap<<T::Node as Vertex>::EqTy, Option<<T::Node as Vertex>::VisitTy>> {
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
    type Node: Vertex;
    /// The type used to represent edge weights.
    type Distance: Clone + Ord + Add + Zero;

    /// Executes Dijkstra's algorithm and returns traversal information.
    ///
    /// This method will short circuit once finding the `to` node. Matching this
    /// node relies on [`Vertex::eq_repr`], whereas visited information relies
    /// on [`Vertex::visit_repr`]. See [`Vertex`] for more information.
    ///
    /// Implementation adapted from: <https://codereview.stackexchange.com/a/202879>
    fn dijkstra(&self, from: Self::Node, to: Self::Node) -> Traversal<Self> {
        let mut map = Traversal::new();
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        // using Zero trait allows us to generalize this for any distance type.
        // we also don't need to worry about "infinity" since the map will
        // return None for missing keys -- we can use that as "infinity" and not
        // have to further restrict the distance type (or use some faulty value
        // like usize::MAX)
        map.distances.insert(from.eq_repr(), Self::Distance::zero());
        map.previous.insert(from.eq_repr(), None);

        // using the Visit type here allows us to generalize "node with
        // distance", control the Ord logic for a min-heap, and not have to
        // require the Node type to provide its own distance value
        queue.push(Visit(from, Self::Distance::zero()));
        while let Some(Visit(node, dist)) = queue.pop() {
            // check if we found the target (use eq type)
            if node.eq_repr() == to.eq_repr() {
                return map;
            }
            // check if we saw this node (use visit type))
            if !visited.insert(node.visit_repr()) {
                continue;
            }

            for n in self.adjacent(&node) {
                // we must use the eq type for distance/previous tracking
                let key = n.eq_repr();
                let d = dist.clone() + self.edge(&node, &n);

                // using map_or here handles a None in the distance map -- this
                // is equivalent to "infinity" in the algorithm
                if map.distances.get(&key).map_or(true, |dist| &d < dist) {
                    map.distances.insert(key.clone(), d.clone());
                    map.previous.insert(key, Some(node.visit_repr()));
                }
                // IMPORTANT: this neighbor needs to be added to the queue
                // regardless of the updated distance. this is a consequence of
                // allowing different visited logic with equality logic.
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

        map.distances[&to.eq_repr()].clone()
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
    /// Because of the short-circuiting nature of the algorithm (e.g., marking
    /// nodes as "closed" or "visited" once all neighbors are visited), the
    /// `edge` method must always provide a positive number.
    fn edge(&self, from: &Self::Node, to: &Self::Node) -> Self::Distance;
}

/// A [`Graph`] *vertex*.
///
/// This trait encapsulates pathing and traversal logic used by [`Graph`] for
/// visiting nodes and checking equality.
pub trait Vertex: Clone {
    /// The type that handles equality logic of *vertices*.
    ///
    /// This may be the same as `VisitTy`, but is not required to be.
    type EqTy: Clone + Eq + Hash;

    /// The type that handles visited logic of *vertices*.
    ///
    /// This may be the same as `EqTy`, but is not required to be.
    type VisitTy: Clone + Eq + Hash;

    /// Returns the node's *equality* representation.
    ///
    /// This is used for checking equality of nodes (e.g., if the algorithm has
    /// reached the destination). This differs from [`Vertex::visit_repr`]
    /// because the algorithm can track "seen" or "visited" nodes with different
    /// qualities from those that determine equality (e.g., comparing X/Y
    /// coordinates for equality and comparing direction, rotation, etc. to
    /// consider a node "visited"). This allows nodes to possibly be visited
    /// multiple times from different directions, while still correctly tracking
    /// weights for *equivalent* nodes.
    fn eq_repr(&self) -> Self::EqTy;

    /// Returns the node's *visited* representation for tracking intermediate
    /// steps in the algorithm.
    ///
    /// This type may be different from the [`Vertex::eq_repr`], and will
    /// typically be more detailed to track movement constraints in the *graph*.
    ///
    /// Note that there is a significant performance penalty if this value does
    /// not have the same logic as `eq_repr`, since the algorithm will visit the
    /// same node multiple times.
    fn visit_repr(&self) -> Self::VisitTy;
}

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

    impl Vertex for char {
        type EqTy = Self;
        type VisitTy = Self;

        fn eq_repr(&self) -> Self::EqTy {
            *self
        }

        fn visit_repr(&self) -> Self::VisitTy {
            *self
        }
    }

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
