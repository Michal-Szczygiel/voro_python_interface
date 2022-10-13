use pyo3::prelude::*;

use crate::voronoi_cell::VoronoiCell;

#[repr(C)]
struct RawVoronoiCell {
    vertices: *mut f64,
    vertices_size: usize,
    face_vertices: *mut i32,
    face_vertices_size: usize,
    normals: *mut f64,
    normals_size: usize,
    x_pos: f64,
    y_pos: f64,
    z_pos: f64,
    particle_id: i32,
}

extern "C" {
    fn voro_orthogonal_container_new(
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x_blocks: i32,
        y_blocks: i32,
        z_blocks: i32,
        x_periodic: bool,
        y_periodic: bool,
        z_periodic: bool,
        init_mem_per_block: i32,
    ) -> *mut ();

    fn voro_orthogonal_container_drop_container(container_handle: *mut ());

    fn voro_orthogonal_container_drop_cells(cells_handle: *mut ());

    fn voro_orthogonal_container_add_particle(
        container_handle: *mut (),
        particle_id: i32,
        x_pos: f64,
        y_pos: f64,
        z_pos: f64,
    );

    fn voro_orthogonal_container_particles_number(container_handle: *mut ()) -> i32;

    fn voro_orthogonal_container_calculate_all_cells(container_handle: *mut ()) -> *mut ();

    fn voro_orthogonal_container_cells_number(cells_handle: *mut ()) -> usize;

    fn voro_orthogonal_container_get_raw_voronoi_cell(
        cells_handle: *mut (),
        index: usize,
    ) -> RawVoronoiCell;
}

#[pyclass(unsendable)]
pub struct OrthogonalContainer {
    container_handle: *mut (),
    cells_handle: *mut (),
}

#[pymethods]
impl OrthogonalContainer {
    #[new]
    pub fn new(
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x_blocks: i32,
        y_blocks: i32,
        z_blocks: i32,
        x_periodic: bool,
        y_periodic: bool,
        z_periodic: bool,
        init_mem_per_block: i32,
    ) -> OrthogonalContainer {
        //println!("Utworzono nowy kontener!");

        OrthogonalContainer {
            container_handle: unsafe {
                voro_orthogonal_container_new(
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    z_min,
                    z_max,
                    x_blocks,
                    y_blocks,
                    z_blocks,
                    x_periodic,
                    y_periodic,
                    z_periodic,
                    init_mem_per_block,
                )
            },
            cells_handle: std::ptr::null_mut(),
        }
    }

    pub fn add_particle(&mut self, particle_id: i32, x_pos: f64, y_pos: f64, z_pos: f64) {
        unsafe {
            voro_orthogonal_container_add_particle(
                self.container_handle,
                particle_id,
                x_pos,
                y_pos,
                z_pos,
            )
        }

        //println!("Dodano cząsteczkę!");
    }

    pub fn particles_number(&self) -> usize {
        unsafe { voro_orthogonal_container_particles_number(self.container_handle) as usize }
    }

    pub fn calculate_all_cells(&mut self) -> Vec<VoronoiCell> {
        self.cells_handle =
            unsafe { voro_orthogonal_container_calculate_all_cells(self.container_handle) };

        // Tutaj: konwersja na rustowy typ VoronoiCell
        let cells_number = unsafe { voro_orthogonal_container_cells_number(self.cells_handle) };
        let mut raw_voronoi_cell: RawVoronoiCell;
        let mut voronoi_cells = Vec::<VoronoiCell>::with_capacity(cells_number);

        for index in 0..cells_number {
            raw_voronoi_cell =
                unsafe { voro_orthogonal_container_get_raw_voronoi_cell(self.cells_handle, index) };

            let mut voronoi_cell = VoronoiCell {
                vertices: Vec::with_capacity(raw_voronoi_cell.vertices_size),
                face_vertices: Vec::with_capacity(raw_voronoi_cell.face_vertices_size),
                normals: Vec::with_capacity(raw_voronoi_cell.normals_size),
                x_pos: raw_voronoi_cell.x_pos,
                y_pos: raw_voronoi_cell.y_pos,
                z_pos: raw_voronoi_cell.z_pos,
                particle_id: raw_voronoi_cell.particle_id,
            };

            for offset in 0..raw_voronoi_cell.vertices_size {
                unsafe {
                    voronoi_cell
                        .vertices
                        .push(*raw_voronoi_cell.vertices.offset(offset as isize))
                }
            }

            for offset in 0..raw_voronoi_cell.face_vertices_size {
                unsafe {
                    voronoi_cell
                        .face_vertices
                        .push(*raw_voronoi_cell.face_vertices.offset(offset as isize))
                }
            }

            for offset in 0..raw_voronoi_cell.normals_size {
                unsafe {
                    voronoi_cell
                        .normals
                        .push(*raw_voronoi_cell.normals.offset(offset as isize))
                }
            }

            voronoi_cells.push(voronoi_cell);
        }

        unsafe { voro_orthogonal_container_drop_cells(self.cells_handle) }
        self.cells_handle = std::ptr::null_mut();

        voronoi_cells
    }

    pub fn __len__(&self) -> usize {
        unsafe { voro_orthogonal_container_particles_number(self.container_handle) as usize }
    }
}

impl std::ops::Drop for OrthogonalContainer {
    fn drop(&mut self) {
        unsafe { voro_orthogonal_container_drop_container(self.container_handle) }
        //println!("Usunięto kontener!");
    }
}
