use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use nalgebra::{DMatrix, DVector};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-25.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(_)) => println!("No part 2"),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    // See
    // https://en.wikipedia.org/wiki/Graph_partition#Spectral_partitioning_and_spectral_bisection.
    //
    // We are going to assume the graph will be bisected by cutting some 3 wires, as we only care
    // about the count on either side of the partition.

    let edges = parse_input_into_edges(input)?;
    let vertices = edges.keys().copied().collect::<Vec<_>>();

    let degree_matrix = DMatrix::from_diagonal(&DVector::from_iterator(
        vertices.len(),
        vertices.iter().map(|vertex| edges[vertex].len() as f64),
    ));
    let adjacency_matrix = DMatrix::from_fn(vertices.len(), vertices.len(), |row, col| {
        if row == col {
            return 0.0f64;
        }

        if edges[vertices[row]].contains(vertices[col]) {
            1.0
        } else {
            0.0
        }
    });
    let laplacian_matrix = degree_matrix - adjacency_matrix;

    let symmetric_eigen = laplacian_matrix.symmetric_eigen();
    let (_, (pivot, _)) = symmetric_eigen.eigenvalues.iter().enumerate().fold(
        ((0, f64::MAX), (0, f64::MAX)),
        |acc, (index, eigenvalue)| {
            if *eigenvalue < acc.0.1 {
                ((index, *eigenvalue), acc.0)
            } else if *eigenvalue < acc.1.1 {
                (acc.0, (index, *eigenvalue))
            } else {
                acc
            }
        },
    );
    let (left_partition, right_partition) =
        symmetric_eigen
            .eigenvectors
            .column(pivot)
            .into_iter()
            .partition::<Vec<&f64>, _>(|&eigenvector| *eigenvector > 0.0);

    Ok(left_partition.len() * right_partition.len())
}

fn parse_input_into_edges(input: &str) -> Result<HashMap<&str, HashSet<&str>>> {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let Some((vertex, connected_vertices)) = line.split_once(": ") else {
            return Err(anyhow!(
                "Cannot split input into vertex and connected vertices: {}",
                line
            ));
        };

        for connected_vertex in connected_vertices.split_whitespace() {
            edges.entry(vertex).or_default().insert(connected_vertex);
            edges.entry(connected_vertex).or_default().insert(vertex);
        }
    }

    Ok(edges)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 54);

        Ok(())
    }
}
