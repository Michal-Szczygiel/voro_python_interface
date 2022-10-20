## Wymagane narzędzia:
- Narzędzia budowania Rust'a, do pobrania stąd: https://www.rust-lang.org/tools/install
- Kompilator c++ (projekt domyślnie używa clang++)
- CPython v3

## Kompilacja:
- Utworzyć środowisko wirtualne Pythona i je aktywować,
- Zainstalować w środowisku pythonowym menadżer budowania "maturin" (pip3 install maturin)
- Przejść do katalogu projektu (voronoi_diagram, tam gdzie znajduje się plik "Cargo.toml")
- Zbudować i zainstalować bibliotekę pythonową w aktywowanym środowisku pythonowym: maturin develop --release (więcej o maturin tutaj: https://github.com/PyO3/maturin)
- Teraz biblioteka voronoi_diagram powinna być dostępna z poziomu Pythona
