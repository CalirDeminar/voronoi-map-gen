pub mod rivers {
    use uuid::Uuid;

    use crate::graph::graph::Corner;
    use crate::graph::graph::Edge;
    use crate::graph::graph::Graph;

    const PEAK_RAINWATER_COLLECTION_RATIO: f32 = 0.2;

    pub fn create_rivers<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();
        let max_elev = graph_clone
            .corners
            .values()
            .fold(0.0, |acc: f32, c: &Corner| {
                if c.data.elevation > acc {
                    c.data.elevation
                } else {
                    acc
                }
            });

        let possible_starting_edges: Vec<&Edge> = graph_clone
            .edges
            .values()
            .filter(|c| c.data.elevation > max_elev * 0.75)
            .collect();

        let starting_edge_count = ((possible_starting_edges.len()) as f32
            * PEAK_RAINWATER_COLLECTION_RATIO)
            .max(1.0) as usize;

        let starting_edges = possible_starting_edges.iter().take(starting_edge_count);
        for edge in starting_edges {
            let mut working_edge = *edge;
            let mut new_volume = working_edge.data.river;
            let mut visited_corners: Vec<Uuid> = Vec::new();
            loop {
                new_volume += 1;
                println!("New Volume: {}", new_volume);
                let edge_mut = graph.edges.get_mut(&working_edge.corners).unwrap();
                edge_mut.data.water = true;
                edge_mut.data.river = edge_mut.data.river + new_volume;
                drop(edge_mut);
                let c1 = graph.corners.get_mut(&working_edge.corners.0).unwrap();
                c1.data.water = true;
                c1.data.river = new_volume;
                drop(c1);
                let c2 = graph.corners.get_mut(&working_edge.corners.0).unwrap();
                c2.data.water = true;
                c2.data.river = new_volume;
                drop(c2);

                let down_corner = graph_clone.corners.get(&working_edge.down_corner).unwrap();

                if down_corner.data.water || visited_corners.contains(&down_corner.id) {
                    break;
                }
                visited_corners.push(down_corner.id);
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
