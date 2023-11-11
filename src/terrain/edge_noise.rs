pub mod edge_noise {
    use crate::{graph::graph::Graph, helpers::helpers::position_midpoint};
    use rand::Rng;

    pub fn add_noise_to_edges<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();
        for e_id in graph_clone.edges.keys() {
            let edge = graph.edges.get_mut(e_id).unwrap();

            let p_1 = graph_clone.corners.get(&e_id.0).unwrap();
            let p_2 = graph_clone.corners.get(&e_id.1).unwrap();

            // let points = split_edge_into(&p_1.pos, &p_2.pos, 3);

            edge.noisey_midpoints = diamond_square(p_1.pos, p_2.pos, 2, 0.1);
            // edge.noisey_midpoints = split_edge_into(&p_1.pos, &p_2.pos, 3);

            drop(edge);
        }
        return graph;
    }

    fn diamond_square(
        start: (f32, f32),
        end: (f32, f32),
        iterations: usize,
        limit_x: f32,
    ) -> Vec<(f32, f32)> {
        let mut output = vec![start, end];
        let grad = (end.0 - start.0) / (end.1 - start.1);
        for j in 0..iterations {
            let mut output_buffer = output.clone();
            let l = output.len();
            for i in 0..(l - 1) {
                let o_mid = position_midpoint(&output[i], &output[i + 1]);
                let mid = displace_point(
                    position_midpoint(&output[i], &output[i + 1]),
                    limit_x,
                    grad,
                    j,
                );

                output_buffer.push(mid);
            }
            output_buffer.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            output = output_buffer;
        }
        // println!("Output: {:?}", output);
        return output;
    }

    fn displace_point(point: (f32, f32), limit_x: f32, grad: f32, iteration: usize) -> (f32, f32) {
        let mut rng = rand::thread_rng();
        let iteration_multipler: f32 = (2.0 as f32).powf(-(iteration as f32));
        let x_distance = ((rng.gen::<f32>() * limit_x * 2.0) - limit_x) * iteration_multipler;
        return (point.0 + x_distance, point.1 + (x_distance * (1.0 / grad)));
    }

    fn displacement_amount(limit: f32, iteration: usize) -> f32 {
        let mut rng = rand::thread_rng();
        let iteration_multipler: f32 = (2.0 as f32).powf(-(iteration as f32));
        return ((rng.gen::<f32>() * limit * 2.0) - limit) * iteration_multipler;
    }

    fn split_edge_into(p_1: &(f32, f32), p_2: &(f32, f32), i: usize) -> Vec<(f32, f32)> {
        let mut output: Vec<(f32, f32)> = Vec::new();
        output.push(p_1.clone());
        let grad = (p_2.1 - p_1.1) / (p_2.0 - p_1.0);
        let x_increment = ((p_2.0) - (p_1.0)) / (i - 1) as f32;
        for j in 1..(i - 1) {
            let x_mod = x_increment * j as f32;
            let y_mod = x_mod * grad;
            output.push((p_1.0 + x_mod, p_1.1 + y_mod));
        }
        output.push(p_2.clone());
        return output;
    }

    #[test]
    fn test_edge_split() {
        println!(
            "Edge Split: {:?}",
            split_edge_into(&(0.1, 0.8), &(0.8, 0.1), 5)
        );
    }
}
