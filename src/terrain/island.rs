pub mod island {
    use uuid::Uuid;

    use crate::{
        graph::graph::{Cell, Corner, Graph},
        X_SCALE, Y_SCALE,
    };

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
        for id in find_edge_cell_ids(&graph.clone()) {
            let cell = graph.cells.get_mut(id).unwrap();
            cell.data.water = true;
            cell.data.ocean = true;
            drop(cell);
        }
        return graph;
    }
}
