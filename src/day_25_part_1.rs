use std::collections::HashMap;

use nalgebra::DMatrix;

pub fn run(input: &str) -> usize {
    // Use spectral bisection as suggested on https://www.reddit.com/r/adventofcode/comments/18qbsxs/2023_day_25_solutions/

    // Assuming that the minimum cuts = 3 and we don't care which 3. We just want to know the group
    // sizes after cutting.

    let edges = convert_input_to_edges(input);

    let matrix = generate_laplacian_matrix(edges);

    assert_eq!(matrix.transpose(), matrix);

    let eigen = matrix.symmetric_eigen();

    // Find l2.
    let (_, (l2, _)) = eigen.eigenvalues.iter().enumerate().fold(
        ((usize::MAX, f64::MAX), (usize::MAX, f64::MAX)),
        |acc, (index, eigenvalue)| {
            let (min, second) = acc;

            if *eigenvalue < min.1 {
                return ((index, *eigenvalue), min);
            }

            if *eigenvalue < second.1 {
                return (min, (index, *eigenvalue));
            }

            acc
        },
    );

    // Split v2 into v_plus and v_minus.
    let (v_plus, v_minus) =
        eigen
            .eigenvectors
            .column(l2)
            .iter()
            .fold((0usize, 0usize), |acc, vertex| {
                if *vertex > 0.0f64 {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + 1)
                }
            });

    v_plus * v_minus
}

fn convert_input_to_edges(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::default();

    input.split_terminator("\n").for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        let (lhs, rhs) = line.split_once(": ").unwrap();

        rhs.split_whitespace().for_each(|node| {
            edges.entry(lhs).or_default().push(node);
            edges.entry(node).or_default().push(lhs);
        });
    });

    edges
}

fn generate_laplacian_matrix<'a>(edges: HashMap<&'a str, Vec<&'a str>>) -> DMatrix<f64> {
    let indices: HashMap<&str, usize> =
        HashMap::from_iter(edges.keys().enumerate().map(|(index, node)| (*node, index)));

    let mut matrix = DMatrix::zeros(edges.len(), edges.len());

    edges.into_iter().for_each(|(key, value)| {
        matrix[(indices[key], indices[key])] = value.len() as f64;

        value.iter().for_each(|rhs| {
            matrix[(indices[key], indices[rhs])] = -1.0f64;
        });
    });

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
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

        assert_eq!(run(input), 54);
    }
}
