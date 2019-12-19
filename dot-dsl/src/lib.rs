pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Default, Clone)]
            pub struct Edge {
                node1: String,
                node2: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(n1: &str, n2: &str) -> Self {
                    Edge {
                        node1: String::from(n1),
                        node2: String::from(n2),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs
                            .iter()
                            .map(|(key, val)| (String::from(*key), String::from(*val)))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs
                            .iter()
                            .map(|(key, val)| (String::from(*key), String::from(*val)))
                            .collect(),
                        ..self.clone()
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default, Clone)]
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
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(&self, nodes: &[Node]) -> Graph {
            Graph {
                nodes: Vec::from(nodes),
                ..self.clone()
            }
        }

        pub fn with_edges(&self, edges: &[Edge]) -> Graph {
            Graph {
                edges: Vec::from(edges),
                ..self.clone()
            }
        }

        pub fn get_node(&self, node: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.label == node)
        }

        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs
                    .iter()
                    .map(|(key, val)| (String::from(*key), String::from(*val)))
                    .collect(),
                ..self.clone()
            }
        }
    }
}
