pub mod island;
pub mod terrain {
    use crate::graph::graph::Graph;

    use super::island::island::{
        assign_land_elevation, assign_ocean_cells, mark_coastal_cells, run_island_gen,
    };

    pub fn run_terrain_gen(graph: &mut Graph) -> &mut Graph {
        run_island_gen(graph);
        assign_ocean_cells(graph);
        mark_coastal_cells(graph);
        assign_land_elevation(graph);
        return graph;
    }
}
