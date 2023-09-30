pub mod island;
pub mod terrain {
    use crate::graph::graph::Graph;

    use super::island::island::run_island_gen;

    pub fn run_terrain_gen(graph: &mut Graph) -> &mut Graph {
        return run_island_gen(graph);
    }
}
