pub trait Attributable: Clone {
    fn attr(&self, key: &str) -> Option<&str>;
    fn set_attr(&mut self, key: &str, value: &str);
    fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        let mut new = self.clone();
        for (k, v) in attrs {
            new.set_attr(k, v);
        }
        new
    }
}

pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use crate::Attributable;
    use maplit::hashmap;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap! {},
            }
        }

        pub fn with_nodes(&self, nodes: &[Node]) -> Self {
            let nodes = nodes.to_vec();
            let mut graph = self.clone();
            graph.nodes = nodes;
            graph
        }

        pub fn with_edges(&self, edges: &[Edge]) -> Self {
            let edges = edges.to_vec();
            let mut graph = self.clone();
            graph.edges = edges;
            graph
        }

        pub fn node(&self, id: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.id == id).cloned()
        }
    }

    impl Attributable for Graph {
        fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }

        fn set_attr(&mut self, key: &str, value: &str) {
            self.attrs.insert(key.into(), value.into());
        }
    }

    impl Default for Graph {
        fn default() -> Self {
            Self::new()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use crate::Attributable;
            use maplit::hashmap;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: hashmap! {},
                    }
                }
            }

            impl Attributable for Edge {
                fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }

                fn set_attr(&mut self, key: &str, value: &str) {
                    self.attrs.insert(key.into(), value.into());
                }
            }
        }

        pub mod node {
            use crate::Attributable;
            use maplit::hashmap;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_string(),
                        attrs: hashmap! {},
                    }
                }
            }

            impl Attributable for Node {
                fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }

                fn set_attr(&mut self, key: &str, value: &str) {
                    self.attrs.insert(key.into(), value.into());
                }
            }
        }
    }
}
