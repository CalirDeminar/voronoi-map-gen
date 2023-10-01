pub mod island {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    use nannou::noise::NoiseFn;
    use nannou::noise::Perlin;
    use nannou::noise::Seedable;
    use rand::RngCore;
    use uuid::Uuid;

    const NOISE_SCALE: f32 = 4.0;

    use crate::{graph::graph::Graph, X_SCALE, Y_SCALE};

    fn find_edge_corner_ids(graph: &Graph) -> Vec<&Uuid> {
        let mut output: Vec<&Uuid> = Vec::new();
        for corner in graph.corners.values() {
            if corner.pos.0.eq(&(X_SCALE as f32))
                || corner.pos.0.eq(&0.0)
                || corner.pos.1.eq(&(Y_SCALE as f32))
                || corner.pos.1.eq(&0.0)
            {
                output.push(&corner.id)
            }
        }
        return output;
    }

    fn find_edge_cell_ids(graph: &Graph) -> Vec<&Uuid> {
        let mut output: Vec<&Uuid> = Vec::new();
        let edge_corners = find_edge_corner_ids(graph);
        for cell in graph.cells.values() {
            let is_border = cell
                .edges
                .iter()
                .any(|(c1, c2)| edge_corners.contains(&c1) || edge_corners.contains(&c2));
            if is_border {
                output.push(&cell.id);
            }
        }
        return output;
    }

    pub fn run_island_gen(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();

        let mut rng = rand::thread_rng();
        let seed = rng.next_u32();
        let perlin = Perlin::new().set_seed(seed);

        let cell_ids = graph_clone.cells.keys();
        let edge_cell_ids = find_edge_cell_ids(&graph_clone);
        for id in cell_ids {
            let cell = graph.cells.get_mut(id).unwrap();
            if edge_cell_ids.contains(&id) {
                cell.data.water = true;
                cell.data.ocean = true;
            } else {
                let x = (cell.center.0 + 1.0) / X_SCALE as f32;
                let y = (cell.center.1 + 1.0) / Y_SCALE as f32;
                let perlin_pos_value =
                    ((perlin.get([(x * NOISE_SCALE) as f64, (y * NOISE_SCALE) as f64])) + 1.0)
                        / 2.0;

                let edge_distances = vec![x - 1.0, x, y - 1.0, y];
                let min_edge_distance = edge_distances
                    .iter()
                    .fold(1.0, |acc, d| if d.abs() < acc { d.abs() } else { acc })
                    .abs();

                let edge_penalty = (min_edge_distance - 0.1).min(0.0) * 5.0 * -1.0;
                if perlin_pos_value < (0.5 + edge_penalty as f64) {
                    cell.data.water = true;
                }
            }

            drop(cell);
        }
        return graph;
    }

    pub fn assign_ocean_cells(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let mut queue: VecDeque<&Uuid> = VecDeque::from(find_edge_cell_ids(&graph_clone));
        let mut processed: HashSet<&Uuid> = HashSet::new();
        while let Some(id) = queue.pop_front() {
            let cell = graph_clone.cells.get(&id).unwrap();
            processed.insert(id);
            for id in &cell.neighbors {
                let n_cell = graph_clone.cells.get(&id).unwrap();
                if n_cell.data.water && !processed.contains(id) {
                    processed.insert(id);
                    queue.push_back(&id);
                }
            }

            let cell_mut = graph.cells.get_mut(id).unwrap();
            cell_mut.data.ocean = true;
            drop(cell_mut);
        }
        return graph;
    }
}
