use crate::geometry::*;
use hdf5_metno::File;
use ndarray::{arr1, arr2};

pub struct Hdf5Builder<
    'a,
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
> {
    mesh: &'a DelaunayMesh2d<TC, TT, TO>,
    file: File,
    //vor_diagram: Option<VoronoiDiagram2d>,
}

impl<'a, TC: InCircleTest<Point = Vec2d>, TT: InTriangleTest, TO: TriangleOrientationTest>
    Hdf5Builder<'a, TC, TT, TO>
{
    pub fn new<P: AsRef<std::path::Path>>(
        mesh: &'a DelaunayMesh2d<TC, TT, TO>,
        filename: &P,
    ) -> Hdf5Builder<'a, TC, TT, TO> {
        Hdf5Builder {
            mesh,
            file: File::create(filename).unwrap(),
            //vor_diagram: None,
        }
    }

    pub fn finish(self) {
        println!("INFO: hdf5::finish, file groups:");
        for (i, name) in self.file.member_names().unwrap().iter().enumerate() {
            println!("  i: {}, n: {}", i, name);
        }
    }

    pub fn add_points(self) -> Self {
        println!("INFO: hdf5::add_points");
        // TODO: I think we should be able to pass
        // points directly to dataset builder
        let mut point_data = Vec::with_capacity(self.mesh.points.len());
        for pi in 4..self.mesh.points.len() {
            let p = &self.mesh.points[pi];
            point_data.push([p.x, p.y]);
        }

        let group = self.file.create_group("points").unwrap();
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&arr2(&point_data))
            .create("coords")
            .unwrap();

        self
    }

    pub fn add_delaunay_edges(self) -> Self {
        println!("INFO: hdf5::add_delaunay_points");
        // Make per triangle visit flag
        let n_triangles = self.mesh.triangles.len();
        let mut edge_data = Vec::with_capacity(self.mesh.points.len());
        let mut unvisited_flag = vec![true; n_triangles];
        for ti in 1..n_triangles {
            unvisited_flag[ti] = false;
            let t = &self.mesh.triangles[ti];

            // Exclude the mesh bounding points
            let include_a = t.a > 3;
            let include_b = t.b > 3;
            let include_c = t.c > 3;

            // edge ab
            if unvisited_flag[t.ab] && include_a && include_b {
                let a = self.mesh.points[t.a];
                let b = self.mesh.points[t.b];
                edge_data.push([[a.x, a.y], [b.x, b.y]]);
            }

            // edge bc
            if unvisited_flag[t.bc] && include_b && include_c {
                let b = self.mesh.points[t.b];
                let c = self.mesh.points[t.c];
                edge_data.push([[b.x, b.y], [c.x, c.y]]);
            }

            // edge ca
            if unvisited_flag[t.ca] && include_c && include_a {
                let c = self.mesh.points[t.c];
                let a = self.mesh.points[t.a];
                edge_data.push([[c.x, c.y], [a.x, a.y]]);
            }
        }

        let group = self.file.create_group("delaunay").unwrap();
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&arr2(&edge_data))
            .create("edges")
            .unwrap();

        self
    }

    pub fn add_vor_diagram(self) -> Self {
        println!("INFO: hdf5::add_vor_diagram");
        let n_triangles = self.mesh.triangles.len();
        let mut face_vertices = Vec::with_capacity(n_triangles);
        for ti in 0..n_triangles {
            face_vertices.push(self.mesh.triangle_center(ti));
        }

        let mut edge_data = Vec::with_capacity(self.mesh.points.len());
        let mut unvisited_flag = vec![true; n_triangles];
        for ti in 1..n_triangles {
            unvisited_flag[ti] = false;
            let t = &self.mesh.triangles[ti];

            // Exclude the mesh bounding points
            let include_a = t.a > 3;
            let include_b = t.b > 3;
            let include_c = t.c > 3;

            // edge ab
            if unvisited_flag[t.ab] && include_a && include_b {
                let a = face_vertices[ti];
                let b = face_vertices[t.ab];
                edge_data.push([[a.x, a.y], [b.x, b.y]]);
            }

            // edge bc
            if unvisited_flag[t.bc] && include_b && include_c {
                let b = face_vertices[ti];
                let c = face_vertices[t.bc];
                edge_data.push([[b.x, b.y], [c.x, c.y]]);
            }

            // edge ca
            if unvisited_flag[t.ca] && include_c && include_a {
                let c = face_vertices[ti];
                let a = face_vertices[t.ca];
                edge_data.push([[c.x, c.y], [a.x, a.y]]);
            }
        }

        let group = self.file.create_group("voronoi").unwrap();
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&arr2(&edge_data))
            .create("edges")
            .unwrap();

        self
    }

    pub fn add_circum_circles(self) -> Self {
        println!("INFO: hdf5::add_circum_circles");
        let n_triangles = self.mesh.triangles.len();
        let mut centers = Vec::with_capacity(n_triangles);
        let mut radi = Vec::with_capacity(n_triangles);

        for ti in 1..n_triangles {
            let c = circumcircle(self.mesh, ti);
            centers.push([c.center.x, c.center.y]);
            radi.push(c.radius);
        }

        let group = self.file.create_group("circumcircles").unwrap();
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&arr2(&centers))
            .create("centers")
            .unwrap();

        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&arr1(&radi))
            .create("radius")
            .unwrap();

        self
    }
}
