#include <iostream>
#include <vector>
#include "voro/src/voro++.hh"

struct VoronoiCell {
    std::vector<double> vertices;
    std::vector<int> face_vertices;
    std::vector<double> normals;
    double x_pos;
    double y_pos;
    double z_pos;
    int particle_id;
};

struct RawVoronoiCell {
    double* vertices;
    size_t vertices_size;
    int* face_vertices;
    size_t face_vertices_size;
    double* normals;
    size_t normals_size;
    double x_pos;
    double y_pos;
    double z_pos;
    int particle_id;
};

static_assert(sizeof(double) == 8, "Error! : sizeof(double) != 8\n");
static_assert(sizeof(int) == 4, "Error! : sizeof(int) != 4\n");
static_assert(sizeof(bool) == 1, "Error! : sizeof(bool) != 1\n");
static_assert(sizeof(size_t) == 8, "Error! : sizeof(size_t) != 8\n");

extern "C" {
void* voro_orthogonal_container_new(double x_min,
                                    double x_max,
                                    double y_min,
                                    double y_max,
                                    double z_min,
                                    double z_max,
                                    int x_blocks,
                                    int y_blocks,
                                    int z_blocks,
                                    bool x_periodic,
                                    bool y_periodic,
                                    bool z_periodic,
                                    int init_mem_per_block) {
    voro::container* orthogonal_container = new voro::container(
        x_min, x_max, y_min, y_max, z_min, z_max, x_blocks, y_blocks, z_blocks,
        x_periodic, y_periodic, z_periodic, init_mem_per_block);

    return reinterpret_cast<void*>(orthogonal_container);
}

void voro_orthogonal_container_drop_container(void* container_handle) {
    voro::container* orthogonal_container =
        reinterpret_cast<voro::container*>(container_handle);
    delete orthogonal_container;
}

void voro_orthogonal_container_drop_cells(void* cells_handle) {
    std::vector<VoronoiCell>* computed_cells =
        reinterpret_cast<std::vector<VoronoiCell>*>(cells_handle);
    delete computed_cells;
}

void voro_orthogonal_container_add_particle(void* container_handle,
                                            int particle_id,
                                            double x_pos,
                                            double y_pos,
                                            double z_pos) {
    voro::container* orthogonal_container =
        reinterpret_cast<voro::container*>(container_handle);

    orthogonal_container->put(particle_id, x_pos, y_pos, z_pos);
}

int voro_orthogonal_container_particles_number(void* container_handle) {
    voro::container* orthogonal_container =
        reinterpret_cast<voro::container*>(container_handle);

    return orthogonal_container->total_particles();
}

void* voro_orthogonal_container_calculate_all_cells(void* container_handle) {
    voro::container* orthogonal_container =
        reinterpret_cast<voro::container*>(container_handle);

    voro::c_loop_all loop(*orthogonal_container);
    voro::voronoicell voronoi_cell;
    std::vector<VoronoiCell>* computed_cells = new std::vector<VoronoiCell>;

    if (loop.start()) {
        do {
            if (orthogonal_container->compute_cell(voronoi_cell, loop)) {
                VoronoiCell computed_cell;

                computed_cell.x_pos = loop.x();
                computed_cell.y_pos = loop.y();
                computed_cell.z_pos = loop.z();
                voronoi_cell.vertices(computed_cell.x_pos, computed_cell.y_pos,
                                      computed_cell.z_pos,
                                      computed_cell.vertices);
                voronoi_cell.face_vertices(computed_cell.face_vertices);
                voronoi_cell.normals(computed_cell.normals);
                computed_cell.particle_id = loop.pid();

                computed_cells->push_back(computed_cell);
            }
        } while (loop.inc());
    }

    return reinterpret_cast<void*>(computed_cells);
}

size_t voro_orthogonal_container_cells_number(void* cells_handle) {
    std::vector<VoronoiCell>* computed_cells =
        reinterpret_cast<std::vector<VoronoiCell>*>(cells_handle);

    return computed_cells->size();
}

RawVoronoiCell voro_orthogonal_container_get_raw_voronoi_cell(
    void* cells_handle,
    size_t index) {
    std::vector<VoronoiCell>* computed_cells =
        reinterpret_cast<std::vector<VoronoiCell>*>(cells_handle);

    return {
        .vertices = (*computed_cells)[index].vertices.data(),
        .vertices_size = (*computed_cells)[index].vertices.size(),
        .face_vertices = (*computed_cells)[index].face_vertices.data(),
        .face_vertices_size = (*computed_cells)[index].face_vertices.size(),
        .normals = (*computed_cells)[index].normals.data(),
        .normals_size = (*computed_cells)[index].normals.size(),
        .x_pos = (*computed_cells)[index].x_pos,
        .y_pos = (*computed_cells)[index].y_pos,
        .z_pos = (*computed_cells)[index].z_pos,
        .particle_id = (*computed_cells)[index].particle_id,
    };
}
}
