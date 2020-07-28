use askama::Template;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

// graph

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: Option<f64>,
}

impl FromStr for Edge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = s.trim().split(' ').collect::<Vec<_>>();
        if e.len() != 2 && e.len() != 3 {
            return Err("invalid format".into());
        }
        let from = e[0];
        let to = e[1];
        let weight = if e.len() > 2 {
            if let Ok(w) = e[2].parse::<f64>() {
                Some(w)
            } else {
                return Err("not a number".into());
            }
        } else {
            None
        };
        match (from.parse::<usize>(), to.parse::<usize>()) {
            (Ok(from), Ok(to)) => Ok(Edge { from, to, weight }),
            _ => Err("not a number".into()),
        }
    }
}

pub struct Graph {
    pub edges: Vec<Edge>,
    pub n: usize,
    pub is_directed: bool,
}

// template

#[derive(Template)]
#[template(path = "graph.dot.txt")]
struct GraphTemplate {
    graph: Graph,
}

// main
// input:
// <vertex_n> <graph_option>
// <from> <to> [<weight>]
// ...
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(io::stdin());
    let mut lines = reader.lines();
    let (n, is_directed) = if let Some(l) = lines.next() {
        let l = l?;
        let words = l.split(" ").collect::<Vec<_>>();
        if words.len() != 2 {
            return Err("invalid format".into());
        }
        let n = words[0].parse::<usize>()?;
        let is_directed = words[1].contains("d");
        (n, is_directed)
    } else {
        return Err("no input".into());
    };
    let mut edges = Vec::new();
    for l in lines {
        let l = l?;
        let e = l.parse::<Edge>()?;
        edges.push(e);
    }
    let graph = Graph {
        edges,
        n,
        is_directed,
    };
    let template = GraphTemplate { graph };
    println!("{}", template.render().unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    mod edge {
        use crate::Edge;

        #[test]
        fn parse_from_to() {
            assert_eq!(
                "1 2".parse::<Edge>(),
                Ok(Edge {
                    from: 1,
                    to: 2,
                    weight: None
                })
            );
            assert!("1".parse::<Edge>().is_err());
            assert!("-1 2".parse::<Edge>().is_err());
        }

        #[test]
        fn parse_from_to_weight() {
            assert_eq!(
                "1 2 3".parse::<Edge>(),
                Ok(Edge {
                    from: 1,
                    to: 2,
                    weight: Some(3_f64)
                })
            );
            assert_eq!(
                "1 2 -3".parse::<Edge>(),
                Ok(Edge {
                    from: 1,
                    to: 2,
                    weight: Some(-3_f64)
                })
            );
            assert_eq!(
                "1 2 3.45".parse::<Edge>(),
                Ok(Edge {
                    from: 1,
                    to: 2,
                    weight: Some(3.45_f64)
                })
            );
            assert!("1 2 x".parse::<Edge>().is_err());
        }
    }
}
