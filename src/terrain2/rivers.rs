pub mod rivers2 {
    use std::collections::HashSet;

    use uuid::Uuid;

    use crate::{
        graph2::graph2::{Cell, Corner, Edge, Graph},
        helpers::helpers::{corner_distance, corner_distance2},
    };

    const PEAK_RAINWATER_COLLECTION_RATIO: f32 = 0.25;
    const PEAK_ELEV_MAX: f32 = 0.9;
    const PEAK_ELEV_MIN: f32 = 0.75;

    const MID_RAINWATER_COLLECTION_RATION: f32 = 0.03;
    const MID_ELEV_MAX: f32 = 0.75;
    const MID_ELEV_MIN: f32 = 0.33;

    fn get_edges_with_elevations_between(graph: &Graph, min: f32, max: f32) -> Vec<(&Uuid, &Edge)> {
        let mut edge_ids: Vec<&Uuid> = graph.edges.keys().collect();
        edge_ids.retain(|id| {
            let elev = graph.get_edge_elevation(id);
            return elev > min && elev < max;
        });
        return edge_ids
            .iter()
            .map(|id| (id.clone(), graph.edges.get(id).unwrap()))
            .collect();
    }

    fn corner_is_river_end(graph: &Graph, corner: &Corner) -> bool {
        let corner_edges = corner.edges.iter().map(|id| graph.edges.get(&id).unwrap());
        let cell_ids: Vec<Vec<Uuid>> = corner_edges.map(|edge| edge.cells.clone()).collect();
        return cell_ids
            .concat()
            .iter()
            .any(|cell_id| graph.cells.get(cell_id).unwrap().ocean);
    }

    pub fn create_rivers(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();

        let peak_starting_edges =
            get_edges_with_elevations_between(&graph_clone, PEAK_ELEV_MIN, PEAK_ELEV_MAX);
        let mid_starting_edges =
            get_edges_with_elevations_between(&graph_clone, MID_ELEV_MIN, MID_ELEV_MAX);

        let peak_starting_edge_count = ((peak_starting_edges.len()) as f32
            * PEAK_RAINWATER_COLLECTION_RATIO)
            .max(1.0) as usize;

        let mid_starting_edge_count =
            ((mid_starting_edges.len()) as f32 * MID_RAINWATER_COLLECTION_RATION).max(1.0) as usize;

        let peak_edges: Vec<&(&Uuid, &Edge)> = peak_starting_edges
            .iter()
            .take(peak_starting_edge_count)
            .collect();
        let mid_edges: Vec<&(&Uuid, &Edge)> = mid_starting_edges
            .iter()
            .take(mid_starting_edge_count)
            .collect();

        let starting_edges: Vec<&(&Uuid, &Edge)> = vec![peak_edges, mid_edges].concat();

        for (edge_id, edge) in starting_edges {
            let distance = corner_distance2(
                graph_clone.corners.get(&edge.corners.0).unwrap(),
                graph_clone.corners.get(&edge.corners.1).unwrap(),
            );
            let working_edge = *edge;
            let mut working_edge_id = *edge_id.clone();
            let mut new_volume = working_edge.river;
            let mut visited_corners: HashSet<Uuid> = HashSet::new();
            loop {
                new_volume += distance;
                let edge_mut = graph.edges.get_mut(&working_edge_id).unwrap();
                edge_mut.river = edge_mut.river + new_volume;

                let (down_corner_id, down_corner) =
                    graph.get_edge_downwards_corner(&working_edge_id);

                if visited_corners.contains(&down_corner_id)
                    || corner_is_river_end(&graph_clone, down_corner)
                {
                    break;
                }
                visited_corners.insert(down_corner_id.clone());
                let mut down_corner_edges: Vec<Uuid> = down_corner.edges.clone();
                down_corner_edges.sort_by(|a_id, b_id| {
                    let a_elev = graph.get_edge_elevation(a_id);
                    let b_elev = graph.get_edge_elevation(b_id);
                    return a_elev.partial_cmp(&b_elev).unwrap();
                });
                let next_edge = down_corner_edges.first().unwrap();
                working_edge_id = (*next_edge).clone();
            }
        }

        return graph;
    }
}
