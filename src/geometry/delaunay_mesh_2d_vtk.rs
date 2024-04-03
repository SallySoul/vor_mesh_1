use crate::geometry::*;
use vtkio::model::*;

pub fn mesh_2d_to_vtk<
    P: AsRef<std::path::Path>,
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
>(
    mesh: &DelaunayMesh2d<TC, TT, TO>,
    triangles: &[usize],
    p: P,
) {
    let mut points = Vec::new();
    for p in &mesh.points {
        points.push(p.x);
        points.push(p.y);
        points.push(0.0);
    }
    let mut connectivity = Vec::new();
    let mut cell_types = Vec::new();
    let mut offsets = Vec::new();
    let mut offset = 3;
    for ti in triangles {
        let t = mesh.triangles[*ti];
        connectivity.push(t.a as u64);
        connectivity.push(t.b as u64);
        connectivity.push(t.c as u64);
        cell_types.push(CellType::Triangle);
        offsets.push(offset);
        offset += 3;
    }

    let model = Vtk {
        version: Version { major: 1, minor: 0 },
        title: String::new(),
        byte_order: ByteOrder::LittleEndian,
        file_path: None,
        data: DataSet::inline(UnstructuredGridPiece {
            points: IOBuffer::F64(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    connectivity,
                    offsets,
                },
                types: cell_types,
            },
            data: Attributes {
                point: vec![],
                cell: vec![],
            },
        }),
    };

    model.export(p).unwrap();
}
