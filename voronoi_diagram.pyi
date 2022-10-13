class OrthogonalContainer:
    def __init__(self, x_min: float,
                 x_max: float,
                 y_min: float,
                 y_max: float,
                 z_min: float,
                 z_max: float,
                 x_blocks: int,
                 y_blocks: int,
                 z_blocks: int,
                 x_periodic: bool,
                 y_periodic: bool,
                 z_periodic: bool,
                 init_mem_per_block: int) -> OrthogonalContainer:
        """Inicjalizator klasy kontener"""

    def add_particle(self, particle_id: int, x_pos: float, y_pos: float, z_pos: float):
        """Dodaje cząsteczkę do kontenera"""

    def particles_number(self):
        """Zwraca liczbę cząsteczek w kontenerze"""

    def calculate_all_cells(self) -> list[VoronoiCell]:
        """Zwraca listę komórek diagramu Voronoia"""

    def __len__(self) -> int:
        """Zwraca liczbę cząsteczek w kontenerze"""


class VoronoiCell:
    def get_all_edges(self) -> list[tuple[float, float, float, float, float, float]]:
        """Zwraca listę wierzchołków budujących kolejne krawędzie"""
