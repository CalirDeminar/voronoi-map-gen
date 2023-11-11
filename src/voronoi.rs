pub mod voronoi {
    use rand::Rng;
    use voronator::{delaunator, VoronoiDiagram};

    fn initialise(i: usize, x_scale: f64, y_scale: f64) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<(f64, f64)> = Vec::new();
        for _i in 0..i {
            output.push((rng.gen::<f64>() * x_scale, rng.gen::<f64>() * y_scale));
        }
        return output;
    }

    fn relax_diagram(
        diagram: VoronoiDiagram<delaunator::Point>,
        x_scale: f64,
        y_scale: f64,
    ) -> VoronoiDiagram<delaunator::Point> {
        let i_1_points: Vec<(f64, f64)> = diagram
            .cells()
            .iter()
            .map(|cell| {
                let l = cell.points().len() as f64;
                cell.points().iter().fold((0.0, 0.0), |(x, y), p| {
                    return (x + (p.x / l), y + (p.y / l));
                })
            })
            .collect();
        return VoronoiDiagram::<delaunator::Point>::from_tuple(
            &(0.0, 0.0),
            &(x_scale, y_scale),
            &i_1_points,
        )
        .unwrap();
    }

    fn relax_diagram_n(
        diagram: VoronoiDiagram<delaunator::Point>,
        x_scale: f64,
        y_scale: f64,
        i: usize,
    ) -> VoronoiDiagram<delaunator::Point> {
        let mut base = diagram;
        for _i in 0..i {
            base = relax_diagram(base, x_scale, y_scale);
        }
        return base;
    }

    pub fn initialise_voronoi(
        i: usize,
        x_scale: f64,
        y_scale: f64,
        relax_factor: usize,
    ) -> VoronoiDiagram<delaunator::Point> {
        let base = VoronoiDiagram::<delaunator::Point>::from_tuple(
            &(0.0, 0.0),
            &(x_scale, y_scale),
            &initialise(i, x_scale, y_scale),
        )
        .unwrap();

        return relax_diagram_n(base, x_scale, y_scale, relax_factor);
    }
}
