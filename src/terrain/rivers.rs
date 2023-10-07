pub mod rivers {
    use uuid::Uuid;

    use crate::graph::graph::Edge;
    use crate::graph::graph::Graph;
    use crate::helpers::helpers::corner_distance;

    const PEAK_RAINWATER_COLLECTION_RATIO: f32 = 0.25;

    const MID_RAINWATER_COLLECTION_RATION: f32 = 0.03;

    pub fn create_rivers<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();

        let peak_starting_edges: Vec<&Edge> = graph_clone
            .edges
            .values()
            .filter(|c| c.data.elevation < 0.9 && c.data.elevation > 0.75)
            .collect();

        let mid_starting_edges: Vec<&Edge> = graph_clone
            .edges
            .values()
            .filter(|c| c.data.elevation < 0.75 && c.data.elevation > 0.33)
            .collect();

        let peak_starting_edge_count = ((peak_starting_edges.len()) as f32
            * PEAK_RAINWATER_COLLECTION_RATIO)
            .max(1.0) as usize;

        let mid_starting_edge_count =
            ((mid_starting_edges.len()) as f32 * MID_RAINWATER_COLLECTION_RATION).max(1.0) as usize;

        let peak_edges: Vec<&&Edge> = peak_starting_edges
            .iter()
            .take(peak_starting_edge_count)
            .collect();
        let mid_edges: Vec<&&Edge> = mid_starting_edges
            .iter()
            .take(mid_starting_edge_count)
            .collect();

        let starting_edges: Vec<&&Edge> = vec![peak_edges, mid_edges].concat();
        for edge in starting_edges {
            let distance = corner_distance(
                graph_clone.corners.get(&edge.corners.0).unwrap(),
                graph_clone.corners.get(&edge.corners.1).unwrap(),
            );
            let mut working_edge = *edge;
            let mut new_volume = working_edge.data.river;
            let mut visited_corners: Vec<Uuid> = Vec::new();
            loop {
                new_volume += distance;
                let edge_mut = graph.edges.get_mut(&working_edge.corners).unwrap();
                edge_mut.data.water = true;
                edge_mut.data.river = edge_mut.data.river + new_volume;
                drop(edge_mut);
                let c1 = graph.corners.get_mut(&working_edge.corners.0).unwrap();
                c1.data.water = true;
                c1.data.river = new_volume;
                drop(c1);
                let c2 = graph.corners.get_mut(&working_edge.corners.1).unwrap();
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
