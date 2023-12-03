pub mod island2 {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    use crate::graph2::graph2::Biome;
    use crate::{graph2::graph2::Graph, X_SCALE, Y_SCALE};
    use nannou::noise::NoiseFn;
    use nannou::noise::Perlin;
    use nannou::noise::Seedable;
    use rand::RngCore;
    use uuid::Uuid;

    const NOISE_SCALE: f32 = 4.0;

    const WATER_COVERAGE_MODIFIER: f64 = 1.0;

    fn find_border_corner_ids(graph: &Graph) -> HashSet<&Uuid> {
        let mut output: HashSet<&Uuid> = HashSet::new();
        for (corner_id, corner) in &graph.corners {
            if corner.pos.0.eq(&(X_SCALE as f32))
                || corner.pos.0.eq(&0.0)
                || corner.pos.1.eq(&(Y_SCALE as f32))
                || corner.pos.1.eq(&0.0)
            {
                output.insert(&corner_id);
            }
        }
        return output;
    }

    fn find_border_cell_ids(graph: &Graph) -> HashSet<&Uuid> {
        let mut output: HashSet<&Uuid> = HashSet::new();
        let border_corners = find_border_corner_ids(graph);
        for (cell_id, _cell) in &graph.cells {
            let cell_corners = graph.get_cell_corners_ids(cell_id);
            let is_border = cell_corners.iter().any(|id| border_corners.contains(id));
            if is_border {
                output.insert(&cell_id);
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
        let edge_cell_ids = find_border_cell_ids(&graph_clone);
        for id in cell_ids {
            let cell = graph.cells.get_mut(id).unwrap();
            if edge_cell_ids.contains(&id) {
                cell.water = true;
                cell.ocean = true;
                cell.biome = Biome::Lake;
                drop(cell);
            } else {
                let (x_b, y_b) = graph_clone.get_cell_center(id);
                let x = (x_b + 1.0) / X_SCALE as f32;
                let y = (y_b + 1.0) / Y_SCALE as f32;

                let perlin_pos_value =
                    ((perlin.get([(x * NOISE_SCALE) as f64, (y * NOISE_SCALE) as f64])) + 1.0)
                        / 2.0;

                let edge_distances = vec![x - 1.0, x, y - 1.0, y];
                let min_edge_distance = edge_distances
                    .iter()
                    .fold(1.0, |acc, d| if d.abs() < acc { d.abs() } else { acc })
                    .abs();

                let edge_penalty = (min_edge_distance - 0.1).min(0.0) * 5.0 * -1.0;
                if (perlin_pos_value * WATER_COVERAGE_MODIFIER) < (0.5 + edge_penalty as f64) {
                    cell.water = true;
                    cell.biome = Biome::Lake;
                    drop(cell);
                } else {
                    cell.water = false;
                    drop(cell);
                }
            }
        }
        return graph;
    }

    pub fn assign_ocean_cells(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let edge_cell_ids = find_border_cell_ids(&graph_clone);
        let mut queue: VecDeque<&Uuid> = VecDeque::from_iter(edge_cell_ids.iter().map(|i| *i));
        let mut processed: HashSet<&Uuid> = HashSet::new();
        while let Some(id) = queue.pop_front() {
            processed.insert(id);
            for n_cell_id in graph_clone.get_cell_adjacent_cells(&id) {
                let n_cell = graph_clone.cells.get(n_cell_id).unwrap();
                if n_cell.water && !processed.contains(&n_cell_id) {
                    processed.insert(n_cell_id);
                    queue.push_back(&n_cell_id);
                }
            }

            let cell_mut = graph.cells.get_mut(id).unwrap();
            cell_mut.ocean = true;
            cell_mut.biome = Biome::Ocean;
            drop(cell_mut);
        }
        return graph;
    }

    pub fn assign_coastal_cells(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        for cell_id in graph_clone.cells.keys() {
            let mut cell = graph.cells.get_mut(cell_id).unwrap();
            if !cell.water
                && !cell.ocean
                && graph_clone
                    .get_cell_adjacent_cells(cell_id)
                    .iter()
                    .any(|id| graph_clone.cells.get(id).unwrap().ocean)
            {
                cell.coast = true;
                cell.biome = Biome::Beach;
            }
            drop(cell);
        }
        return graph;
    }
}
