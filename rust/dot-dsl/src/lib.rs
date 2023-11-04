// Alternative using &'a str: https://exercism.org/tracks/rust/exercises/dot-dsl/solutions/rsalmei
pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    #[derive(Default)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, _nodes: &[Node]) -> Self {
            self.nodes = _nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, _edges: &[Edge]) -> Self {
            self.edges =  _edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, _attr: &[(&str, &str)]) -> Self {
            self.attrs = _attr.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            self
        }

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.id == id)
        }

    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge{
                from: String,
                to: String,
                attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(s: &str, t: &str) -> Self {
                    Self {
                        from: s.to_string(),
                        to: t.to_string(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, _attrs: &[(&str, &str)]) -> Self {
                    self.attrs = _attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node{
                pub id: String,
                attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(_id: &str) -> Self {
                    Self{
                        id: _id.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, _attr: &[(&str, &str)]) -> Self {
                    self.attrs = _attr.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|c| c.as_str())
                }
            }
        }
    }
}
