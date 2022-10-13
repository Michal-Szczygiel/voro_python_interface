from pymol import cgo

edges = []

# Wczytanie krawędzi z pliku
with open("diagram.txt", "r") as input_file:
    for line in input_file:
        x_b, y_b, z_b, x_e, y_e, z_e = line.split(",")

        edges.append((float(x_b), float(y_b), float(z_b), float(x_e), float(y_e), float(z_e)))


# Zbudowanie listy zawierającej wierzchołki definiujące krawędzie
shape = [cgo.BEGIN, cgo.LINES]

for edge in edges:
    shape += [cgo.VERTEX, *edge[0:3], cgo.VERTEX, *edge[3:]]

shape.append(cgo.END)

# Rysowanie przy pomocy protokołu CGO
cmd.load_cgo(shape, "voro_test")
