pub mod helpers {
    use crate::graph::graph::Corner;

    pub fn corner_distance(a: &Corner, b: &Corner) -> f32 {
        let x_dist = (a.pos.0 - b.pos.0).abs();
        let y_dist = (a.pos.1 - b.pos.1).abs();
        return (x_dist.powi(2) + y_dist.powi(2)).sqrt();
    }

    pub fn position_midpoint(a: &(f32, f32), b: &(f32, f32)) -> (f32, f32) {
        let x = (a.0 + b.0) / 2.0;
        let y = (a.1 + b.1) / 2.0;
        return (x, y);
    }
}
