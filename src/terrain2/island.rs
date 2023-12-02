pub mod island2 {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    use crate::{graph2::graph2::Graph, X_SCALE, Y_SCALE};
    use nannou::noise::NoiseFn;
    use nannou::noise::Perlin;
    use nannou::noise::Seedable;
    use rand::RngCore;
    use uuid::Uuid;

    const NOISE_SCALE: f32 = 4.0;

    const WATER_COVERAGE_MODIFIER: f64 = 1.0;

    pub fn run_island_gen(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let mut rng = rand::thread_rng();
        return graph;
    }
}
