pub mod graph {
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, entries: &[(&str, &str)]) -> Self {
                    for (key, value) in entries {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_str())
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                src: String,
                dest: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        src: a.to_string(),
                        dest: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, entries: &[(&str, &str)]) -> Self {
                    for (key, value) in entries {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, entries: &[(&str, &str)]) -> Self {
            for (key, value) in entries {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == node_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[test]
    fn empty_graph() {
        let graph = Graph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
    }

    #[test]
    #[ignore]
    fn graph_with_one_node() {
        let nodes = vec![Node::new("a")];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.nodes, vec![Node::new("a")]);
    }

    #[test]
    #[ignore]
    fn graph_with_one_node_with_keywords() {
        let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];
        let graph = Graph::new().with_nodes(&nodes);
        assert!(graph.edges.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(
            graph.nodes,
            vec![Node::new("a").with_attrs(&[("color", "green")])]
        );
    }

    #[test]
    #[ignore]
    fn graph_with_one_edge() {
        let edges = vec![Edge::new("a", "b")];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
    }

    #[test]
    #[ignore]
    fn graph_with_one_edge_with_keywords() {
        let edges = vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])];
        let graph = Graph::new().with_edges(&edges);
        assert!(graph.nodes.is_empty());
        assert!(graph.attrs.is_empty());
        assert_eq!(
            graph.edges,
            vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])]
        );
    }

    #[test]
    #[ignore]
    fn graph_with_one_attribute() {
        let graph = Graph::new().with_attrs(&[("foo", "1")]);
        #[allow(clippy::useless_conversion, reason = "allow String and &str")]
        let expected_attrs = HashMap::from([("foo".into(), "1".into())]);
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert_eq!(graph.attrs, expected_attrs);
    }
    #[test]
    #[ignore]
    fn graph_with_attributes() {
        let nodes = vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ];
        let edges = vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ];
        let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];
        #[allow(clippy::useless_conversion, reason = "allow String and &str")]
        let expected_attrs = HashMap::from([
            ("foo".into(), "1".into()),
            ("title".into(), "Testing Attrs".into()),
            ("bar".into(), "true".into()),
        ]);
        let graph = Graph::new()
            .with_nodes(&nodes)
            .with_edges(&edges)
            .with_attrs(&attrs);
        assert_eq!(
            graph.nodes,
            vec![
                Node::new("a").with_attrs(&[("color", "green")]),
                Node::new("c"),
                Node::new("b").with_attrs(&[("label", "Beta!")]),
            ]
        );
        assert_eq!(
            graph.edges,
            vec![
                Edge::new("b", "c"),
                Edge::new("a", "b").with_attrs(&[("color", "blue")]),
            ]
        );
        assert_eq!(graph.attrs, expected_attrs);
    }

    #[test]
    #[ignore]
    fn edges_store_attributes() {
        let nodes = vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ];
        let edges = vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
        ];
        let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];
        let graph = Graph::new()
            .with_nodes(&nodes)
            .with_edges(&edges)
            .with_attrs(&attrs);
        assert_eq!(
            graph.edges,
            vec![
                Edge::new("b", "c"),
                Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
            ]
        );
        assert_eq!(graph.edges[1].attr("color"), Some("blue"));
        assert_eq!(graph.edges[1].attr("fill"), Some("darkblue"));
        assert_eq!(graph.edges[1].attr("foo"), None);
        assert_eq!(graph.edges[0].attr("color"), None);
        assert_eq!(graph.edges[0].attr("fill"), None);
        assert_eq!(graph.edges[0].attr("foo"), None);
    }

    #[test]
    #[ignore]
    fn graph_nodes_store_attributes() {
        let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
        let graph = Graph::new().with_nodes(
            &["a", "b", "c"]
                .iter()
                .zip(attributes.iter())
                .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
                .collect::<Vec<_>>(),
        );
        let a = graph.node("a").expect("node a must be stored");
        assert_eq!(a.attr("foo"), Some("bar"));
        assert_eq!(a.attr("bat"), None);
        assert_eq!(a.attr("bim"), None);
        let b = graph.node("b").expect("node b must be stored");
        assert_eq!(b.attr("foo"), None);
        assert_eq!(b.attr("bat"), Some("baz"));
        assert_eq!(b.attr("bim"), None);
        let c = graph.node("c").expect("node c must be stored");
        assert_eq!(c.attr("foo"), None);
        assert_eq!(c.attr("bat"), None);
        assert_eq!(c.attr("bim"), Some("bef"));
    }
}
