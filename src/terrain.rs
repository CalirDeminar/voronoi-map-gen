pub mod elevation;
pub mod island;
pub mod rivers;
pub mod terrain {
    use crate::{
        graph::graph::Graph,
        terrain::{elevation::elevation::assign_land_elevation, rivers::rivers::create_rivers},
    };

    use super::island::island::{assign_ocean_cells, mark_coastal_cells, run_island_gen};

    pub fn run_terrain_gen(graph: &mut Graph) -> &mut Graph {
        run_island_gen(graph);
        assign_ocean_cells(graph);
        mark_coastal_cells(graph);
        assign_land_elevation(graph);
        create_rivers(graph);
        println!("Genned");
        return graph;
    }
}
