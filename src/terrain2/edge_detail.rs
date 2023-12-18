pub mod edge_detail {
    use rand::Rng;
    use uuid::Uuid;

    use crate::{
        graph2::graph2::{Edge, Graph},
        helpers::helpers::{corner_distance2, position_midpoint},
        terrain2::island::island2::find_border_cell_ids,
    };

    const I: usize = 2;

    fn divide_edge(p1: &(f32, f32), p2: &(f32, f32), i: usize) -> Vec<(f32, f32)> {
        let midpoint = position_midpoint(&p1, &p2);
        if i.eq(&1) {
            return vec![p1.clone(), midpoint, p2.clone()];
        } else {
            let mut head = divide_edge(p1, &midpoint, i - 1);
            head.remove(head.len() - 1);

            let tail = divide_edge(&midpoint, p2, i - 1);

            return vec![head, tail].concat();
        }
    }

    fn generate_edge_midpoints(graph: &Graph, edge_id: &Uuid) -> Vec<(f32, f32)> {
        let edge = graph.edges.get(edge_id).unwrap();

        let p1 = graph.corners.get(&edge.corners.0).unwrap();
        let p2 = graph.corners.get(&edge.corners.1).unwrap();

        return divide_edge(&p1.pos, &p2.pos, I);
    }

    fn displace_points(points: Vec<(f32, f32)>, limit: f32) -> Vec<(f32, f32)> {
        let mut output = points.clone();
        if points.len() > 3 {
            let center_index = (points.len() - 1) / 2;
            let v1: Vec<(f32, f32)> = points[0..=center_index].iter().map(|p| p.clone()).collect();
            let v2: Vec<(f32, f32)> = points[center_index..=points.len() - 1]
                .iter()
                .map(|p| p.clone())
                .collect();
            return vec![
                displace_points(v1, limit * 0.5),
                displace_points(v2, limit * 0.5),
            ]
            .concat();
        } else {
            let mut output = points.clone();
            let dx = points.last().unwrap().0 - points.first().unwrap().0;
            let dy = points.last().unwrap().1 - points.first().unwrap().1;
            let displace_grad = 1.0 / (dy / dx);
            let dis_x = 0.01;
            let dis_y = dis_x * displace_grad;
            output[1] = (output[1].0 + dis_x, output[1].1 + dis_y);
            return output;
        }
    }

    fn displace_midpoints_in_place(
        points: Vec<(f32, f32)>,
        limit: f32,
        start: usize,
        end: usize,
    ) -> Vec<(f32, f32)> {
        let mut rng = rand::thread_rng();
        let mut output = points.clone();
        // println!("start: {} end {}", start, end);
        let mid_len = (end - start) / 2;
        let mid_index = start + mid_len;
        if end - start > 3 {
            return displace_midpoints_in_place(
                displace_midpoints_in_place(output, limit * 0.5, start, mid_index),
                limit * 0.5,
                mid_index,
                end,
            );
        } else if !(mid_index.eq(&0) || mid_index.eq(&(points.len() - 1))) {
            let dx = points.last().unwrap().0 - points.first().unwrap().0;
            let dy = points.last().unwrap().1 - points.first().unwrap().1;
            let displace_grad = 1.0 / (dy / dx);
            let displace_x_scale = (limit / displace_grad) * rng.gen::<f32>();
            let displace_x = displace_x_scale - ((limit / displace_grad) / 2.0);
            let displace_y = displace_x * displace_grad;
            // println!(
            //     "Indexs: {} Before: {:?} After: {:?}",
            //     mid_index,
            //     output[mid_index],
            //     (
            //         output[mid_index].0 + displace_x,
            //         output[mid_index].1 + displace_y,
            //     )
            // );
            // WORK HERE
            // println!("Mid: {}", mid_index);
            output[mid_index] = (
                output[mid_index].0 + displace_x,
                output[mid_index].1 + displace_y,
            );
            return output;
        }
        return output;
    }

    pub fn add_edge_divisions(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let border_cells = find_border_cell_ids(&graph_clone);
        for edge_id in graph_clone.edges.keys() {
            let edge_mut = graph.edges.get_mut(edge_id).unwrap();
            let midpoints = generate_edge_midpoints(&graph_clone, &edge_id);
            // let do_not_displace = edge_mut.cells.iter().any(|id| border_cells.contains(id));
            // if !do_not_displace {
            //     let l = midpoints.len();
            //     let edge_len = corner_distance2(
            //         graph_clone.corners.get(&edge_mut.corners.0).unwrap(),
            //         graph_clone.corners.get(&edge_mut.corners.1).unwrap(),
            //     );
            //     edge_mut.corner_midpoints =
            //         displace_midpoints_in_place(midpoints, edge_len * 0.05, 0, l - 1);
            // } else {
            edge_mut.corner_midpoints = midpoints;
            // }

            drop(edge_mut);
        }
        println!("Displacement Done");
        return graph;
    }

    #[cfg(test)]
    mod tests {
        use crate::{graph2::graph2::generate_base_graph, X_SCALE, Y_SCALE};

        use super::*;

        #[test]
        fn gen_base_graph_test() {
            println!("{:?}", divide_edge(&(10.0, 0.0), &(0.0, 10.0), 2));
        }

        #[test]
        fn test_edge_division_inclusion() {
            let mut graph = generate_base_graph(500, X_SCALE, Y_SCALE);
            add_edge_divisions(&mut graph);
            for edge in graph.edges.values() {
                let c1 = graph.corners.get(&edge.corners.0).unwrap();
                let c2 = graph.corners.get(&edge.corners.1).unwrap();
                assert!(edge.corner_midpoints.first().unwrap().eq(&c1.pos));
                assert!(edge.corner_midpoints.last().unwrap().eq(&c2.pos));
            }
        }
    }
}
