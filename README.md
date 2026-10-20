# Rodro Turbo Search

Lanzador de aplicaciones para **GNOME sobre Wayland**, escrito en Rust y GTK4.
Alt + Espacio abre la búsqueda; escribí un nombre, seleccioná con ↑/↓ y pulsá
Enter. Escape o hacer clic fuera ocultan la ventana. Al volver con Alt + Espacio
se conserva la consulta, el cursor y la aplicación seleccionada (si sigue disponible).
Después de abrir una aplicación o copiar un cálculo, la barra se limpia para la
siguiente búsqueda. Si falla la apertura, conserva la consulta para poder reintentar.
La consulta se mantiene solo mientras el proceso está abierto, sin guardarla en disco.
También podés abrir resultados con un clic. La barra vacía no muestra texto de
ayuda en ninguno de los modos. Para moverla, hacé clic y arrastrá desde cualquier
parte de la interfaz, incluso el campo de texto o los resultados. La única
excepción es el logo: cambia de modo y se ilumina al pasar el mouse. Los clics
sin arrastre siguen enfocando el campo y abriendo resultados; arrastrar el texto
mueve la ventana (para seleccionarlo podés usar el teclado). La barra permanece
abierta durante la interacción y el arrastre, aunque el compositor cambie el foco
transitoriamente; al hacer clic afuera vuelve a ocultarse.

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
- Icono: `~/.local/share/icons/hicolor/scalable/apps/io.github.rodro.TurboSearch.svg`.
- Inicio de sesión: `~/.config/autostart/io.github.rodro.TurboSearch.desktop`.
- Estado para desinstalar: `~/.local/share/rodro-turbo-search/install-state.json`.

## Qué incluye

Descubre aplicaciones con GIO, respeta la visibilidad de las entradas `.desktop`
y muestra sus nombres e iconos. Refresca el catálogo al abrir la ventana, sin
vigilancia continua del sistema de archivos. Las aplicaciones Flatpak aparecen
si sus entradas están exportadas en las rutas XDG de la sesión.

La búsqueda ignora mayúsculas, admite coincidencias parciales y ordena primero
coincidencias exactas, luego prefijos y finalmente coincidencias internas.
Al iniciar por primera vez muestra solo la barra; al escribir aparecen hasta cinco resultados con
orden determinista. Al borrar la consulta vuelve a quedar compacta. El panel tiene
fondo negro al 82% de opacidad, bordes redondeados y texto opaco para mantener
la legibilidad. El lanzamiento usa GIO;
los errores quedan visibles y no cierran el buscador.

GApplication mantiene una sola instancia por sesión, con comunicación D-Bus local.
No usa red, telemetría, base de datos ni historial.

## Archivos y modo violeta

Hacé clic en el logo de la barra para incluir archivos y carpetas de tu carpeta
personal. El fondo se llena de violeta translúcido; otro clic vuelve al negro y
busca solo aplicaciones. También podés enfocar el botón con Tab y activarlo con
Espacio. La animación dura 280 ms, admite invertirla con otro clic y respeta la
preferencia de GNOME de desactivar animaciones. No se anima cuando está en reposo.

Con al menos dos caracteres aparecen hasta cinco archivos debajo de las aplicaciones.
Se busca por nombre, sin distinguir mayúsculas, y se muestra la ruta para distinguir
nombres repetidos. Enter o clic abre el resultado con la aplicación predeterminada
mediante GIO y limpia la barra. La calculadora sigue funcionando en ambos modos.
El modo elegido se conserva durante la sesión del proceso.

Para mantener el consumo bajo, hay una espera de 200 ms después de escribir y un
único hilo de búsqueda, sin índice ni vigilancia del disco. Cambiar la consulta,
desactivar el modo u ocultar la ventana cancela el recorrido. No se leen contenidos,
no se siguen enlaces simbólicos y se omiten nombres ocultos, `node_modules`, `target`
y `vendor`. Esta versión busca dentro de tu carpeta personal, no en todo `/`.
El recorrido se limita a 30.000 entradas, 16 niveles, 2.048 carpetas pendientes y
700 ms entre operaciones de disco; si alcanza un límite, indica que los resultados
son parciales. Un disco o montaje lento puede demorar una operación del sistema,
pero nunca bloquea el hilo de la interfaz. No hay garantías de búsqueda exhaustiva
para árboles grandes; los resultados se ordenan entre los archivos recorridos.

## Calculadora

Escribí una operación en la misma barra y el resultado aparece inmediatamente.
**Enter** o un clic en el resultado lo copia al portapapeles y oculta la barra.
No se copia nada mientras escribís. Al copiar, la consulta se limpia; la próxima
apertura con Alt + Espacio muestra la barra vacía.

| Expresión | Resultado |
| --- | --- |
| `125 * 1.22` | `152.5` |
| `(2 + 3) * 4` | `20` |
| `2^8` | `256` |
| `200 * 15%` | `30` |
| `1,5 + 0,5` | `2` |

Admite `+`, `-`, `*`, `/`, `^`, paréntesis, signos, porcentajes y notación
científica (`1e-3`). También acepta `×`, `÷` y `−`. La coma y el punto se interpretan
como separadores decimales, sin separadores de miles. `%` convierte el valor
anterior en una fracción de cien: `15% = 0.15`.

Un número entero solo sigue buscando aplicaciones; `=42` fuerza el modo de
calculadora. Nombres como `7-Zip` y `1Password` siguen siendo búsquedas normales.
Una operación incompleta muestra una indicación discreta; una división por cero o
un resultado fuera de rango muestra un error sin cerrar la aplicación.

El cálculo usa coma flotante y presenta hasta unas 12 cifras significativas;
no es aritmética decimal exacta. La expresión se conserva al ocultar/reabrir igual
que una búsqueda, únicamente en memoria.

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
No publica ni instala dependencias del sistema. La compilación del instalador y
del paquete reemplaza las rutas locales del desarrollador por rutas genéricas y
verifica que no aparezcan en el ejecutable. Para compartir binarios, usar estos
scripts en lugar de empaquetar una compilación manual de Cargo.

El logo de la aplicación es `assets/rodro-turbo-search-v2-rayo.svg`. La barra
lo incluye dentro del ejecutable para funcionar también sin instalación. El
instalador lo registra como icono SVG en el tema `hicolor` del usuario. El acceso
del menú referencia directamente ese archivo para evitar resoluciones antiguas
por nombre del tema.

## Arquitectura

Un crate con módulos pequeños:

- `cli`: argumentos y validación.
- `platform`: instancia única, residencia y activación.
- `catalog`: descubrimiento Linux mediante GIO.
- `search`: coincidencias y ranking, independiente de GTK y Linux.
- `calculator`: evaluación aritmética acotada, independiente de GTK y Linux.
- `files`: búsqueda de nombres en un hilo, cancelable y con recorrido acotado.
- `background`: transición de color dibujada solo mientras se anima.
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

Validación de esta entrega: formato y Clippy correctos; doce pruebas unitarias y
la prueba de interfaz en Wayland correctas. Cubren cálculo, comandos, ranking,
búsqueda de archivos, exclusiones, cancelación y reemplazo de consultas. La prueba
de interfaz verifica selección, apertura y error de lanzamiento, conservación de
consulta, limpieza al activar y cancelación al cambiar de modo u ocultar la barra.
