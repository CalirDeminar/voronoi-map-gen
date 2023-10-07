pub mod elevation {
    use crate::graph::graph::Graph;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use uuid::Uuid;

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
        normalise_elevation(graph);
        return graph;
    }

    fn normalise_elevation<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();
        let max_elev = graph_clone
            .corners
            .values()
            .map(|c| c.data.elevation)
            .fold(0.0, |acc, e| if e > acc { e } else { acc });
        for c_id in graph_clone.corners.keys() {
            let corner = graph.corners.get_mut(c_id).unwrap();
            corner.data.elevation /= max_elev;
            drop(corner);
        }
        for e_id in graph_clone.edges.keys() {
            let edge = graph.edges.get_mut(e_id).unwrap();
            edge.data.elevation /= max_elev;
            drop(edge);
        }
        for c_id in graph_clone.cells.keys() {
            let cell = graph.cells.get_mut(c_id).unwrap();
            cell.data.elevation /= max_elev;
            drop(cell);
        }
        return graph;
    }
}
