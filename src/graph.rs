pub mod graph {
    use std::collections::HashMap;

    use rand::Rng;
    use uuid::Uuid;
    use voronator::{delaunator, VoronoiDiagram};

    #[derive(Debug, Clone)]
    pub struct Cell {
        id: Uuid,
        neighbors: Vec<Uuid>,
        edges: Vec<(Uuid, Uuid)>,
        corners: Vec<Uuid>,
    }

    #[derive(Debug, Clone)]
    pub struct Edge {
        id: Uuid,
        corners: (Uuid, Uuid),
        cells: Vec<Uuid>,
    }

    #[derive(Debug, Clone)]
    pub struct Corner {
        id: Uuid,
        pos: (f32, f32),
        edges: Vec<Uuid>,
        cells: Vec<Uuid>,
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        cells: HashMap<Uuid, Cell>,
        edges: HashMap<(Uuid, Uuid), Edge>,
        corners: HashMap<Uuid, Corner>,
    }

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

    pub fn generate_base_diagram(
        i: usize,
        x_scale: f64,
        y_scale: f64,
    ) -> VoronoiDiagram<delaunator::Point> {
        let base = VoronoiDiagram::<delaunator::Point>::from_tuple(
            &(0.0, 0.0),
            &(x_scale, y_scale),
            &initialise(i, x_scale, y_scale),
        )
        .unwrap();

        let rtn = relax_diagram_n(base, x_scale, y_scale, 5);
        let mut graph = Graph {
            cells: HashMap::new(),
            edges: HashMap::new(),
            corners: HashMap::new(),
        };
        for cell in rtn.cells() {
            let cell_id = Uuid::new_v4();
            let mut graph_cell = Cell {
                id: cell_id,
                neighbors: Vec::new(),
                edges: Vec::new(),
                corners: Vec::new(),
            };
            // Corner Handling
            let points = cell.points();
            let mut cell_corner_ids: Vec<Uuid> = Vec::new();
            for point in points {
                let pos = (point.x as f32, point.y as f32);
                let existing_corner = graph.corners.values().find(|corner| corner.pos.eq(&pos));
                if existing_corner.is_some() {
                    let corner_id = existing_corner.unwrap().id.clone();
                    cell_corner_ids.push(corner_id.clone());
                    let corner_mut = graph.corners.get_mut(&corner_id).unwrap();
                    corner_mut.cells.push(cell_id.clone());
                    drop(corner_mut);
                    graph_cell.corners.push(corner_id.clone());
                } else {
                    let corner = Corner {
                        id: Uuid::new_v4(),
                        cells: vec![cell_id.clone()],
                        edges: Vec::new(),
                        pos: pos,
                    };
                    cell_corner_ids.push(corner.id.clone());
                    graph_cell.corners.push(corner.id);
                    graph.corners.insert(corner.id, corner);
                }
            }
            // Edge Handling
            let final_point: Option<&Uuid> = cell_corner_ids.last();
            let mut prev_point: Option<&Uuid> = None;
            for id in &cell_corner_ids {
                let new_edge = if prev_point.is_none() {
                    // first case
                    (final_point.unwrap(), id)
                } else {
                    // all other cases
                    (id, prev_point.unwrap())
                };
                let new_edge_corners = (
                    graph.corners.get(&new_edge.0).unwrap(),
                    graph.corners.get(&new_edge.1).unwrap(),
                );
                let existing_edge = graph
                    .edges
                    .get(&(new_edge_corners.0.id, new_edge_corners.1.id))
                    .or(graph
                        .edges
                        .get(&(new_edge_corners.1.id, new_edge_corners.0.id)));
                if existing_edge.is_some() {
                    let key = existing_edge.unwrap().corners.clone();
                    graph_cell.edges.push(key.clone());
                    drop(existing_edge);
                    let existing_edge_mut = graph.edges.get_mut(&key).unwrap();
                    existing_edge_mut.cells.push(cell_id.clone());
                    drop(existing_edge_mut);
                } else {
                    drop(existing_edge);
                    let key = (new_edge_corners.0.id.clone(), new_edge_corners.1.id.clone());
                    drop(new_edge_corners);
                    let edge = Edge {
                        id: Uuid::new_v4(),
                        corners: key.clone(),
                        cells: vec![cell_id.clone()],
                    };
                    graph_cell.edges.push(edge.corners.clone());
                    graph.edges.insert(
                        (new_edge_corners.0.id.clone(), new_edge_corners.1.id.clone()).clone(),
                        edge.clone(),
                    );

                    let corner_1 = graph.corners.get_mut(&key.0).unwrap();
                    corner_1.edges.push(edge.id.clone());
                    drop(corner_1);

                    let corner_2 = graph.corners.get_mut(&key.1).unwrap();
                    corner_2.edges.push(edge.id.clone());
                    drop(corner_2);
                }
                prev_point = Some(id);
            }

            graph.cells.insert(cell_id, graph_cell);
        }
        // Cell adjacency
        for edge in graph.edges.values() {
            for cell_id in &edge.cells {
                let cell_mut = graph.cells.get_mut(&cell_id).unwrap();
                for cell_id_2 in &edge.cells {
                    if !cell_id.eq(cell_id_2) {
                        cell_mut.neighbors.push(cell_id_2.clone());
                    }
                }
                drop(cell_mut);
            }
        }
        println!("{:#?}", graph.cells.iter().take(5));
        return rtn;
    }
}
