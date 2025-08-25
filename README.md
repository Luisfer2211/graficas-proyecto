# Proyecto 1 – Gráficas por Computadora
# Shrek Find and Rescue

Este repositorio contiene el **Proyecto 1** del curso **Gráficas por Computadora**, desarrollado en **Rust** utilizando la librería [macroquad](https://github.com/not-fl3/macroquad).  
El proyecto consiste en un **juego en primera persona (FPS) basado en raycasting**, que incluye un menú principal y un nivel jugable con elementos interactivos inspirados en el universo de Shrek.

---

## 📂 Estructura del proyecto
```
C:.
│   .gitignore
│   README.md
│
└───brawl_fps
    │   Cargo.lock
    │   Cargo.toml
    │
    ├───img
    │       bosque.png
    │       burro.png
    │       castillo.png
    │       final.wav
    │       fiona.png
    │       fondo.wav
    │       gato.png
    │       menu.png
    │       moneda.wav
    │       moneda1.wav
    │       planicie.png
    │
    └───src
            main.rs
            nivel1.rs
            nivel2.rs
```


## ▶️ Ejecución

Para ejecutar el proyecto es necesario tener **Rust** y **cargo** instalados.  
Se recomienda instalarlos desde [rustup.rs](https://rustup.rs).

1. Clonar este repositorio o descargarlo en su máquina local.
   ```bash
   git clone https://github.com/Luisfer2211/graficas-proyecto
3. Acceder al directorio del proyecto:
   ```bash
   cd brawl_fps
4. Ejecutar el juego mediante:
    ```bash
   cargo run

🎮 Controles del juego
Movimiento: W, A, S, D o flechas direccionales.

Pausa: ESC.

✨ Características principales:

- Menú principal con diseño personalizado.

- Selección de niveles.


Nivel 1:

- Movimiento en primera persona con raycasting.

- Interacción con amigo: la salida se desbloquea únicamente tras encontrarlo.

- Implementación de audio en formato .wav.

- Uso de texturas y sprites personalizados (burro, fiona, gato, entre otros).

Nivel 2: 

- Misma lógica y mapa del primer nivel pero la salida se desbloquea únicamente tras encontrar a los dos amigos.

🎥 Video demostrativo
Se puede visualizar una explicación detallada y demostración del juego en el siguiente enlace:
👉 https://youtu.be/YBN-Pizhl6o

👨‍💻 Auto

Luis Palacios

Proyecto 1 – Gráficas por Computadora