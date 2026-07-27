# Mis scripts de Niri para extender su funcionalidad

## Vista general
https://github.com/user-attachments/assets/fab73541-c683-49b3-80da-ae1d3a110348

## ¿Por qué?
Me gusta Niri y quería extender su funcionalidad para mis necesidades.


## Características

* **Sticky Floating**: Mueve cualquier ventana flotante al espacio de trabajo enfocado.
* **Fondo de pantalla por espacio de trabajo**: Asigna un fondo de pantalla para cada espacio de trabajo individual.
* **Captura de pantalla**: Toma una captura de pantalla y luego anótala.

## Dependencias

* **Scriptisto**: Requerido para ejecutar el script. [GitHub](https://github.com/igor-petruk/scriptisto)
* **Fondo de pantalla por espacio de trabajo**:
    - [swww](https://github.com/LGFae/swww)
* **Captura de pantalla**:
    - [grim](https://github.com/GrimAnticheat/Grim)
    - [satty](https://github.com/gabm/Satty)
    - [slurp](https://github.com/emersion/slurp)
    - [wl-clipboard](https://github.com/bugaevc/wl-clipboard) (solo se usa cuando no utilizas --annotate)

---

## Instalación

1. Clona el repositorio:

   `git clone https://github.com/0xwal/niri-scripts.git`

2. Haz que los scripts sean ejecutables:

   `chmod +x niri-scripts/support-sticky-floating`
   `chmod +x niri-scripts/toggle-sticky`

   `chmod +x niri-scripts/wallpaper-per-workspace`

   `chmod +x niri-scripts/screenshot`

## Ejecución

### Ejecutar los scripts:

`niri-scripts/support-sticky-floating &`

`niri-scripts/wallpaper-per-workspace <WALLPAPER_DIR> &`

`disown`

O dentro de tu configuración:

```kdl
spawn-sh-at-startup "niri-scripts/support-sticky-floating"

binds {
    // Para tomar una captura de pantalla, usa el atajo de teclado que quieras
    Super+S { spawn-sh "niri-scripts/screeenshot <PATH_TO_SAVE_SCREENSHOT>"; }

    // Para tomar una captura de pantalla y luego anotarla, usa el atajo de teclado que quieras
    Super+Ctrl+S { spawn-sh "niri-scripts/screeenshot <PATH_TO_SAVE_SCREENSHOT> --annotate"; }

    // Para hacer que la ventana enfocada sea "sticky", usa el atajo de teclado que quieras
    Super+Shift+S { spawn-sh "niri-scripts/toggle-sticky"; }
}

```

### ARGUMENTOS

* `WALLPAPER_DIR`: Este directorio contiene cada fondo de pantalla nombrado según el índice del espacio de trabajo. Por ejemplo:
  * `WALLPAPER_DIR/1` estará activo cuando el espacio de trabajo 1 esté enfocado.
  * `WALLPAPER_DIR/2` estará activo cuando el espacio de trabajo 2 esté enfocado, y así sucesivamente.
  * `WALLPAPER_DIR/FALLBACK` se utilizará al activar un espacio de trabajo que no tenga un archivo de fondo de pantalla.

- `PATH_TO_SAVE_SCREENSHOT`: La ruta del directorio para guardar tus capturas de pantalla.
