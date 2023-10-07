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
        }
        return graph;
    }

    pub fn assign_land_elevation(graph: &mut Graph) -> &mut Graph {
        let mut graph_clone = graph.clone();
        let coastal_corner_ids: Vec<(Uuid, f32)> = graph_clone
            .corners
            .iter()
            .filter(|(_k, v)| v.data.coast)
            .map(|(k, _v)| (k.clone(), 0.0))
            .collect();
        let mut queue: VecDeque<(Uuid, f32)> = VecDeque::from(coastal_corner_ids);
        let mut processed: HashSet<Uuid> = HashSet::new();

        while let Some((id, base_evelation)) = queue.pop_front() {
            processed.insert(id.clone());
            let corner = graph.corners.get_mut(&id).unwrap();
            let has_adjacent_lake = corner
                .cells
                .iter()
                .map(|c_id| graph.cells.get(c_id).unwrap())
                .any(|cell| cell.data.water);
            let new_elev = base_evelation + if has_adjacent_lake { 0.0 } else { 1.0 };
            let a_corners: Vec<&Uuid> = corner
                .edges
                .iter()
                .map(|(c1, c2)| vec![c1, c2])
                .collect::<Vec<Vec<&Uuid>>>()
                .concat();

            for a_corner_id in a_corners {
                let a_edge = graph_clone.corners.get(&a_corner_id).unwrap();
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
        graph_clone = graph.clone();
        for edge_id in graph_clone.edges.keys() {
            let edge = graph.edges.get_mut(edge_id).unwrap();
            let c1 = graph_clone.corners.get(&edge_id.0).unwrap();
            let c2 = graph_clone.corners.get(&edge_id.1).unwrap();
            edge.data.elevation = (c1.data.elevation + c2.data.elevation) / 2.0;
            edge.down_corner = if c1.data.elevation < c2.data.elevation {
                c1.id.clone()
            } else {
                c2.id.clone()
            };
            drop(edge);
        }
        return graph;
    }

    pub fn create_rivers<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();
        let possible_starting_edges = graph_clone
            .edges
            .values()
            .filter(|c| c.data.elevation > 0.0);
        let starting_edges = possible_starting_edges.take(20);
        for edge in starting_edges {
            let mut working_edge = edge;
            loop {
                let edge_mut = graph.edges.get_mut(&working_edge.corners).unwrap();
                edge_mut.data.water = true;
                drop(edge_mut);

                let down_corner = graph_clone.corners.get(&working_edge.down_corner).unwrap();

                if down_corner.data.water {
                    break;
                }

                let mut down_corner_edges = down_corner.edges.clone();
                down_corner_edges.sort_by(|a_id, b_id| {
                    graph
                        .edges
                        .get(a_id)
                        .unwrap()
                        .data
                        .elevation
                        .partial_cmp(&graph.edges.get(b_id).unwrap().data.elevation)
                        .unwrap()
                });
                let next_edge = down_corner_edges.first().unwrap();
                working_edge = graph_clone.edges.get(next_edge).unwrap();
            }
        }
        return graph;
    }
}
