use nfs::{flow, Graph};

fn main() {
    let graph = read();
    let result = flow(&graph);
    for (node, (time, nodes)) in result.into_iter().enumerate() {
        println!("node {node}: time {time}, nodes {nodes}");
    }
}

/// Read Graph from stdin according to the expected format.
/// Panics if input data does not match the expected format.
fn read() -> Graph {
    use std::io::BufRead;
    let lines: std::io::Result<Vec<String>> = std::io::stdin().lock().lines().collect();
    let lines = lines.expect("failed to read form stdin");

    let head = lines.first().expect("failed to read header line");
    let (nodes, edges) = split2(head);
    if lines.len() != edges + 1 {
        panic!("invalid input")
    }

    let mut graph = Graph::new(nodes);
    for line in lines.iter().skip(1) {
        let (src, dst, weight) = split3(line);
        graph.link(src, dst, weight);
    }
    graph
}

fn split2(s: &str) -> (usize, usize) {
    let split = s
        .splitn(2, |c: char| c.is_ascii_whitespace())
        .map(|chunk| chunk.parse::<usize>().expect("invalid input"))
        .collect::<Vec<usize>>();
    assert_eq!(split.len(), 2, "invalid input: expected 2 tokens");
    (split[0], split[1])
}

fn split3(s: &str) -> (usize, usize, usize) {
    let split = s
        .splitn(3, |c: char| c.is_ascii_whitespace())
        .map(|chunk| chunk.parse::<usize>().expect("invalid input"))
        .collect::<Vec<usize>>();
    assert_eq!(split.len(), 3, "invalid input: expected 3 tokens");
    (split[0], split[1], split[2])
}
