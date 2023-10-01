pub mod island;
pub mod terrain {
    use crate::graph::graph::Graph;

    use super::island::island::{assign_ocean_cells, run_island_gen};

    pub fn run_terrain_gen(graph: &mut Graph) -> &mut Graph {
        run_island_gen(graph);
        return assign_ocean_cells(graph);
    }
}
