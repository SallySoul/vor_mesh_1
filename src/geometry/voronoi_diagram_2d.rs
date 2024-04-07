use crate::geometry::*;

pub struct VoronoiDiagram2d {
    pub face_vertices: Vec<Vec2d>,
    pub edges: Vec<(usize, usize)>,
    // At some point we'll want cell to edge mapping
}

impl VoronoiDiagram2d {
    pub fn new<TC: InCircleTest<Point = Vec2d>, TT: InTriangleTest, TO: TriangleOrientationTest>(
        mesh: &DelaunayMesh2d<TC, TT, TO>,
    ) -> VoronoiDiagram2d {
        // Calculate Centroids
        // Become face vertices
        let n_triangles = mesh.triangles.len();
        let mut face_vertices = Vec::with_capacity(n_triangles);
        for ti in 0..n_triangles {
            face_vertices.push(mesh.triangle_center(ti));
        }

        // Make per triangle visit flag
        let mut unvisited_flag = vec![true; n_triangles];
        let mut edges = Vec::with_capacity(n_triangles);
        for ti in 0..n_triangles {
            unvisited_flag[ti] = false;
            let t = &mesh.triangles[ti];

            // edge ab
            if unvisited_flag[t.ab] {
                edges.push((t, t.ab));
            }

            // edge bc
            if unvisited_flag[t.bc] {
                edges.push((t, t.bc));
            }

            // edge ca
            if unvisited_flag[t.ca] {
                edges.push((t, t.ca));
            }
        }

        VoronoiDiagram2d {
            face_vertices,
            edges: vec![],
        }
    }
}
