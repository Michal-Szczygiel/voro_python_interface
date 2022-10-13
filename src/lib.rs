mod orthogonal_container;
mod voronoi_cell;

use pyo3::prelude::*;

#[pymodule]
fn voronoi_diagram(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<orthogonal_container::OrthogonalContainer>()?;
    module.add_class::<voronoi_cell::VoronoiCell>()?;

    Ok(())
}
