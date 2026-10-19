# Rodro Turbo Search

Lanzador de aplicaciones para **GNOME sobre Wayland**, escrito en Rust y GTK4.
Alt + Espacio abre la búsqueda; escribí un nombre, seleccioná con ↑/↓ y pulsá
Enter. Escape oculta la ventana. También podés abrir resultados con un clic.

## Instalar y usar

Desde este proyecto:

```sh
./install.sh
```

El instalador compila una versión optimizada e instala **solo para tu usuario**,
sin sudo. Configura Alt + Espacio, un acceso en el menú de aplicaciones y el inicio
automático con la sesión. Después de instalar, simplemente presioná Alt + Espacio.
No hace falta dejar una terminal abierta. Si el proceso no está iniciado, el
atajo lo inicia; las siguientes aperturas reutilizan la misma instancia.

Si Alt + Espacio pertenece al menú de ventana de GNOME, el instalador guarda su
valor y libera esa combinación. Si pertenece a otro atajo personalizado, detiene
la instalación sin reemplazarlo. No modifica otros atajos personalizados.

Para instalar **sin Rust ni compilación**, extraé el paquete
`dist/rodro-turbo-search-linux-x86_64.tar.gz` y ejecutá su `./install.sh`.
El paquete incluye el ejecutable; necesita GTK4, Python 3 con PyGObject y una sesión
GNOME. Está preparado para Linux Mint 22.3 / Ubuntu 24.04 x86_64 y entornos
compatibles; no es un binario estático universal.

Para actualizar, ejecutá de nuevo el instalador. Para desinstalar:

```sh
/usr/bin/python3 ~/.local/share/rodro-turbo-search/setup.py uninstall
```

Si usás `XDG_DATA_HOME`, la ruta está dentro de ese directorio y se muestra al
instalar. La desinstalación elimina los archivos propios y restaura el atajo del
menú de ventana si no lo modificaste después de instalar.

Ubicaciones predeterminadas:

- Ejecutable: `~/.local/bin/rodro-turbo-search`.
- Acceso: `~/.local/share/applications/io.github.rodro.TurboSearch.desktop`.
- Inicio de sesión: `~/.config/autostart/io.github.rodro.TurboSearch.desktop`.
- Estado para desinstalar: `~/.local/share/rodro-turbo-search/install-state.json`.

## Qué incluye

Descubre aplicaciones con GIO, respeta la visibilidad de las entradas `.desktop`
y muestra sus nombres e iconos. Refresca el catálogo al abrir la ventana, sin
vigilancia continua del sistema de archivos. Las aplicaciones Flatpak aparecen
si sus entradas están exportadas en las rutas XDG de la sesión.

La búsqueda ignora mayúsculas, admite coincidencias parciales y ordena primero
coincidencias exactas, luego prefijos y finalmente coincidencias internas.
Muestra hasta siete resultados con orden determinista. El lanzamiento usa GIO;
los errores quedan visibles y no cierran el buscador.

GApplication mantiene una sola instancia por sesión, con comunicación D-Bus local.
No usa red, telemetría, base de datos, historial ni búsqueda de archivos.

## Desarrollo

Requisitos: Rust estable, compilador C, pkg-config y bibliotecas de desarrollo GTK4.
En Mint/Ubuntu, si faltan:

```sh
sudo apt-get install libgtk-4-dev build-essential pkg-config python3-gi
```

```sh
cargo build --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
/usr/bin/python3 tests/installer_test.py
```

Las pruebas del instalador usan directorios temporales y GSettings en memoria;
no alteran la configuración del usuario. Para validar los widgets y el lanzamiento
con aplicaciones de prueba dentro de una sesión gráfica:

```sh
GDK_BACKEND=wayland cargo test --locked ui::tests::search_selection_launch_and_failure -- --ignored --test-threads=1
```

Comandos de la aplicación:

```sh
./target/debug/rodro-turbo-search         # iniciar residente oculto
./target/debug/rodro-turbo-search toggle  # mostrar/ocultar
./target/debug/rodro-turbo-search quit    # finalizar
```

Para generar un paquete precompilado local: `./scripts/package.sh`.
No publica ni instala dependencias del sistema.

## Arquitectura

Un crate con módulos pequeños:

- `cli`: argumentos y validación.
- `platform`: instancia única, residencia y activación.
- `catalog`: descubrimiento Linux mediante GIO.
- `search`: coincidencias y ranking, independiente de GTK y Linux.
- `ui`: presentación, selección y apertura mediante GIO.
- `scripts/setup.py`: instalación por usuario y desinstalación de la integración GNOME.

Sin backend Windows ni interfaces especulativas. Ver [DECISIONS.md](DECISIONS.md)
y [DEPENDENCIES.md](DEPENDENCIES.md). Proyecto privado, sin licencia de distribución
asignada y con `publish = false`.

## Validación y límites

La prueba inicial de ventana fue confirmada por el usuario. GNOME sigue decidiendo
el posicionamiento, monitor y foco: no se promete una posición absoluta ni estar
siempre encima. Conviene comprobar Alt + Espacio desde distintas aplicaciones,
Escape y reapertura, y configuraciones de varios monitores o escalado fraccional.

`tests/session_smoke.py [ruta-al-binario]` comprueba residencia, instancia única,
20 pares de `toggle` y salida. Requiere que no haya una instancia existente y abre
brevemente ventanas; no comprueba foco real ni posición.

Para medir, usar el binario release, tomar RSS/PSS de `/proc/PID/smaps_rollup` y
CPU por intervalo con `pidstat -p PID 1 10` si está disponible. Medir oculto al
inicio, después de varios minutos, visible y tras abrir/cerrar 20 veces.
`time ... toggle` mide comando/IPC, no tiempo hasta el primer frame. Las muestras
de fase 0 no representan el consumo de esta nueva versión con catálogo.

Validación de esta entrega: formato y Clippy correctos; cinco pruebas de comandos
 y búsqueda, cuatro pruebas del instalador y una prueba de interfaz en Wayland
correctas. Esta última verifica resultados, selección, apertura de un proceso de
prueba, error de lanzamiento, ausencia de resultados y limpieza al reabrir.
