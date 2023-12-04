pub mod biome;
pub mod elevation;
pub mod island;
pub mod rivers;
pub mod terrain2 {
    use std::time::Instant;

    use crate::{
        graph2::graph2::{generate_base_graph, Graph},
        helpers::helpers::create_benchmarker,
    };

    use super::{
        biome::biome::assign_biomes,
        elevation::elevation2::assign_land_elevation,
        island::island2::{assign_coastal_cells, assign_ocean_cells, run_island_gen},
        rivers::rivers2::create_rivers,
    };

    pub fn run_terrain_gen(graph: &mut Graph) -> &mut Graph {
        run_island_gen(graph);
        assign_ocean_cells(graph);
        assign_coastal_cells(graph);
        return graph;
    }

    pub fn full_terrain_gen(i: usize, x_scale: f64, y_scale: f64) -> Graph {
        let base_graph_gen = create_benchmarker(String::from("Base Graph Gen"));
        let mut graph = generate_base_graph(i, x_scale, y_scale);
        base_graph_gen();

        let island_gen = create_benchmarker(String::from("Island Gen"));
        run_island_gen(&mut graph);
        island_gen();

        let assign_ocean = create_benchmarker(String::from("Ocean Assign"));
        assign_ocean_cells(&mut graph);
        assign_ocean();

        let assign_coastal = create_benchmarker(String::from("Coastal Assign"));
        assign_coastal_cells(&mut graph);
        assign_coastal();

        let assign_elevation = create_benchmarker(String::from("Elevation Assign"));
        assign_land_elevation(&mut graph);
        assign_elevation();

        let assign_rivers = create_benchmarker(String::from("Create Rivers"));
        create_rivers(&mut graph);
        assign_rivers();

        let biome_assign = create_benchmarker(String::from("Assign Biomes"));
        assign_biomes(&mut graph);
        biome_assign();

        return graph;
    }
}
