import voronoi_diagram
from Bio.PDB import PDBParser

# Wczytanie struktury cząsteczki z pliku PDB
pdb_parser = PDBParser()
structure = pdb_parser.get_structure("ID", "7v39.pdb")
atoms = structure.get_atoms()
atoms_positions = [atom.get_coord() for atom in atoms]

# Wyznaczenie kresów przestrzeni w 3D
x_min = min(atoms_positions, key=lambda x: x[0])[0]
x_max = max(atoms_positions, key=lambda x: x[0])[0]

y_min = min(atoms_positions, key=lambda x: x[1])[1]
y_max = max(atoms_positions, key=lambda x: x[1])[1]

z_min = min(atoms_positions, key=lambda x: x[2])[2]
z_max = max(atoms_positions, key=lambda x: x[2])[2]

# Zbudowanie kontenera o odpowiedniej wielkości
container = voronoi_diagram.OrthogonalContainer(
    x_min, x_max, y_min, y_max, z_min, z_max, 16, 16, 16, False, False, False, 32)

# Dodanie atomów do kontenera
for index, atom in enumerate(atoms_positions):
    container.add_particle(index, *atom)

# Obliczenie komórek
cells = container.calculate_all_cells()

# Zapisanie wyników obliczeń (krawędzi komórek) do pliku
with open("diagram.txt", "w") as output_file:
    for cell in cells:
        for edge in cell.get_all_edges():
            output_file.write(f"{edge[0]},{edge[1]},{edge[2]},{edge[3]},{edge[4]},{edge[5]}\n")
