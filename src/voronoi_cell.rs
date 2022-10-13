use pyo3::prelude::*;

#[pyclass]
pub struct VoronoiCell {
    #[pyo3(get)]
    pub vertices: Vec<f64>,

    #[pyo3(get)]
    pub face_vertices: Vec<i32>,

    #[pyo3(get)]
    pub normals: Vec<f64>,

    #[pyo3(get)]
    pub x_pos: f64,

    #[pyo3(get)]
    pub y_pos: f64,

    #[pyo3(get)]
    pub z_pos: f64,

    #[pyo3(get)]
    pub particle_id: i32,
}

#[pymethods]
impl VoronoiCell {
    pub fn get_all_edges(&self) -> Vec<(f64, f64, f64, f64, f64, f64)> {
        let mut edges = Vec::<(f64, f64, f64, f64, f64, f64)>::new();
        let mut offset: usize = 0;
        let mut vertices_to_read: usize;

        while offset < self.face_vertices.len() {
            vertices_to_read = self.face_vertices[offset] as usize;
            offset += 1;

            for index in offset..offset + vertices_to_read - 1 {
                edges.push((
                    self.vertices[self.face_vertices[index] as usize * 3],
                    self.vertices[self.face_vertices[index] as usize * 3 + 1],
                    self.vertices[self.face_vertices[index] as usize * 3 + 2],
                    self.vertices[self.face_vertices[index + 1] as usize * 3],
                    self.vertices[self.face_vertices[index + 1] as usize * 3 + 1],
                    self.vertices[self.face_vertices[index + 1] as usize * 3 + 2],
                ))
            }

            edges.push((
                self.vertices[self.face_vertices[offset + vertices_to_read - 1] as usize * 3],
                self.vertices[self.face_vertices[offset + vertices_to_read - 1] as usize * 3 + 1],
                self.vertices[self.face_vertices[offset + vertices_to_read - 1] as usize * 3 + 2],
                self.vertices[self.face_vertices[offset] as usize * 3],
                self.vertices[self.face_vertices[offset] as usize * 3 + 1],
                self.vertices[self.face_vertices[offset] as usize * 3 + 2],
            ));

            offset += vertices_to_read;
        }

        edges
    }
}
