pub mod island {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    use nannou::noise::NoiseFn;
    use nannou::noise::Perlin;
    use nannou::noise::Seedable;
    use rand::RngCore;
    use uuid::Uuid;

    const NOISE_SCALE: f32 = 4.0;

    const WATER_COVERAGE_MODIFIER: f64 = 1.0;

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
                drop(cell);
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
                if (perlin_pos_value * WATER_COVERAGE_MODIFIER) < (0.5 + edge_penalty as f64) {
                    cell.data.water = true;
                    drop(cell);
                    let cell = graph_clone.cells.get(id).unwrap();
                    for edge_id in &cell.edges {
                        let edge = graph.edges.get_mut(&edge_id).unwrap();
                        edge.data.water = true;
                        drop(edge);
                    }
                } else {
                    drop(cell);
                }
            }
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
            for e_id in &cell.edges {
                let edge = graph.edges.get_mut(&e_id).unwrap();
                edge.data.ocean = true;
                edge.data.water = true;

                let p1_id = edge.corners.0.clone();
                let p2_id = edge.corners.1.clone();
                drop(edge);

                let p1 = graph.corners.get_mut(&p1_id).unwrap();
                p1.data.water = true;
                p1.data.ocean = true;
                drop(p1);
                let p2 = graph.corners.get_mut(&p2_id).unwrap();
                p2.data.water = true;
                p2.data.ocean = true;
                drop(p2);
            }
        }
        return graph;
    }

    pub fn mark_coastal_cells(graph: &mut Graph) -> &mut Graph {
        let graph_cells_clone = graph.cells.clone();
        let graph_corners_clone = graph.corners.clone();
        for id in graph_cells_clone.keys() {
            let mut cell = graph.cells.get_mut(id).unwrap();
            if !cell.data.water
                && !cell.data.ocean
                && cell
                    .neighbors
                    .iter()
                    .any(|n_id| graph_cells_clone.get(n_id).unwrap().data.ocean)
            {
                cell.data.coast = true;
                drop(cell);

                // edge handling
                let cell = graph_cells_clone.get(id).unwrap();
                for neighbor_id in &cell.neighbors {
                    let neighbor = graph_cells_clone.get(neighbor_id).unwrap();
                    if neighbor.data.ocean {
                        let shared_edges: Vec<&(Uuid, Uuid)> = cell
                            .edges
                            .iter()
                            .filter(|(c1, c2)| {
                                neighbor.edges.contains(&(c1.clone(), c2.clone()))
                                    || neighbor.edges.contains(&(c2.clone(), c1.clone()))
                            })
                            .collect();
                        for se in shared_edges {
                            let edge = graph.edges.get_mut(se).unwrap();
                            edge.data.coast = true;
                            drop(edge);
                            let c1 = graph.corners.get_mut(&se.0).unwrap();
                            c1.data.coast = true;
                            drop(c1);
                            let c2 = graph.corners.get_mut(&se.1).unwrap();
                            c2.data.coast = true;
                            drop(c2);
                        }
                    }
                }
            } else {
                drop(cell);
            }
            let cell = graph_cells_clone.get(id).unwrap();
        }
        return graph;
    }

    pub fn assign_land_elevation(graph: &mut Graph) -> &mut Graph {
        let graph_edge_clone = graph.edges.clone();
        let graph_corner_clone = graph.corners.clone();
        let coastal_corner_ids: Vec<(Uuid, f32)> = graph_corner_clone
            .iter()
            .filter(|(_k, v)| v.data.coast)
            .map(|(k, _v)| (k.clone(), 0.0))
            .collect();
        let mut queue: VecDeque<(Uuid, f32)> = VecDeque::from(coastal_corner_ids);
        let mut processed: HashSet<Uuid> = HashSet::new();

        while let Some((id, base_evelation)) = queue.pop_front() {
            let new_elev = base_evelation + 1.0;
            println!("{}", new_elev);
            processed.insert(id.clone());
            let corner = graph.corners.get_mut(&id).unwrap();
            let a_corners: Vec<&Uuid> = corner
                .edges
                .iter()
                .map(|(c1, c2)| vec![c1, c2])
                .collect::<Vec<Vec<&Uuid>>>()
                .concat();

            for a_corner_id in a_corners {
                let a_edge = graph_corner_clone.get(&a_corner_id).unwrap();
                if !a_corner_id.eq(&id)
                    && !processed.contains(&a_corner_id)
                    && !a_edge.data.coast
                    && !a_edge.data.ocean
                {
                    queue.push_back((a_corner_id.clone(), new_elev.clone()));
                    processed.insert(a_corner_id.clone());
                }
            }

            let corner_mut = graph.corners.get_mut(&id).unwrap();
            corner_mut.data.elevation = new_elev;
            drop(corner_mut);
        }
        return graph;
    }
}
