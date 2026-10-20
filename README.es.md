# Rodro Turbo Search

[English](README.md) | **Español**

Buscador y lanzador de aplicaciones ligero, desarrollado en **Rust y GTK4** para
**Linux con GNOME sobre Wayland**, con calculadora integrada y búsqueda opcional
por nombre de archivo.

**Estado:** MVP funcional, proyecto personal en desarrollo. No se presenta como
una versión estable ni como un producto validado en muchos escritorios. La
interfaz y los mensajes de la línea de comandos están actualmente en español.

## Motivación

Abrir una aplicación, calcular un valor o encontrar un documento puede interrumpir
la tarea actual con varios menús y ventanas. Este proyecto explora un acceso
compacto, pensado para el teclado: abrir una barra, escribir y activar un resultado.
También es un ejercicio práctico de Rust, integración nativa con el escritorio,
trabajo en segundo plano acotado y módulos mantenibles. «Ligero» es un objetivo
de diseño; no se afirman ventajas comparativas de rendimiento ni memoria.

## Funcionalidades actuales

- **Lanzador de aplicaciones:** descubre aplicaciones `.desktop` visibles mediante
  GIO, ordena coincidencias exactas, prefijos y coincidencias internas sin distinguir
  mayúsculas, y muestra hasta cinco aplicaciones. El catálogo se actualiza al abrir.
- **Modo de archivos opcional:** un clic en el logo suma hasta cinco coincidencias
  de nombres de archivos/carpetas debajo de las aplicaciones. Un llenado violeta
  translúcido identifica este modo; otro clic vuelve al modo negro de aplicaciones.
  La búsqueda recorre la carpeta personal.
- **Calculadora:** evalúa operaciones aritméticas, paréntesis, potencias, porcentajes,
  signos y notación científica. Enter o un clic copia el resultado al portapapeles
  y oculta la barra. Por ejemplo, `125 * 1.22` da `152.5`, `(2 + 3) * 4` da `20`
  y `200 * 15%` da `30`. `=42` fuerza el cálculo; un entero solo sigue buscando.
  El separador decimal puede ser punto o coma.
- **Flujo de escritorio:** una instancia residente, Alt + Espacio después de
  instalar, flechas para seleccionar, Enter para abrir y Escape o un cambio de
  foco externo para ocultar. Una activación exitosa limpia la consulta; ocultar
  una búsqueda incompleta la conserva en memoria. No se guarda historial en disco.
- **Interfaz movible:** se puede arrastrar desde cualquier parte salvo el logo,
  incluso sobre el texto o los resultados. Los clics simples conservan su función.
  Arrastrar texto mueve la ventana; para seleccionarlo se usa el teclado. Incluye
  fondos translúcidos redondeados, hover del logo y una transición breve de modo.
  La animación de llenado respeta la preferencia de animaciones de GTK.

## Tecnologías y requisitos

Rust edición 2024, `gtk4` 0.11.5, GIO/GLib/GDK, D-Bus de sesión y GSettings de GNOME.
El instalador por usuario utiliza Python 3 y PyGObject del sistema. La aplicación
no incorpora un servicio web, telemetría, base de datos, índice persistente ni
vigilancia del disco.

El entorno previsto es **GNOME sobre Wayland en Linux**. Las comprobaciones locales
se ejecutaron con GTK 4.14.5 y Rust 1.99.1; esto no constituye una matriz de
compatibilidad. No se validaron otros escritorios, X11, Windows ni macOS. Que GTK
sea multiplataforma no implica que esta aplicación los soporte. El instalador
requiere los esquemas de GNOME.

Para compilar se necesita Rust/Cargo estable actual (los bindings GTK fijados
requieren Rust 1.92 o superior), compilador C, `pkg-config` y bibliotecas de desarrollo
GTK4. Se usa el conjunto base de APIs 4.0, pero la combinación completa de
dependencias solo se comprobó en el entorno local. Un binario precompilado también
depende de la arquitectura, libc y bibliotecas compartidas del entorno de compilación.

En sistemas basados en Debian/Ubuntu, instalá los requisitos de compilación e instalación:

```sh
sudo apt-get install build-essential pkg-config libgtk-4-dev python3 python3-gi
```

Rust/Cargo se instala por separado. `rustfmt` y Clippy se necesitan para las
comprobaciones de contribución, no para ejecutar un binario instalado.

## Compilar y ejecutar

El remoto actual de GitHub tiene la escritura `rodro-turbo-seach`; se conserva.
La aplicación y el paquete Cargo se llaman `rodro-turbo-search`.

```sh
git clone https://github.com/RodrigoNaray/rodro-turbo-seach.git
cd rodro-turbo-seach
cargo build --locked
./target/debug/rodro-turbo-search toggle
```

Ejecutá desde una sesión gráfica con D-Bus de sesión. `toggle` inicia la instancia
residente si hace falta. Sin argumentos se inicia oculta:

```sh
./target/debug/rodro-turbo-search         # iniciar oculto
./target/debug/rodro-turbo-search toggle  # mostrar/ocultar
./target/debug/rodro-turbo-search quit    # finalizar la instancia residente
./target/debug/rodro-turbo-search --help
```

Después de cerrar la barra, el proceso sigue activo intencionalmente. El binario
de desarrollo y el instalado comparten el identificador de aplicación; cerrá la
instancia actual antes de comparar compilaciones.

## Instalar para tu usuario

Desde el código fuente, dentro de una sesión GNOME:

```sh
./install.sh
```

Compila una versión optimizada, instala en `~/.local`, agrega acceso al menú e inicio
automático con la sesión y configura **Alt + Espacio**. No requiere sudo; instalar
los requisitos del sistema sí. Si GNOME utiliza el atajo para el menú de ventana,
el instalador guarda el valor antes de liberarlo. Se detiene si otro atajo
personalizado lo utiliza. Para actualizar, ejecutalo de nuevo.

Usá Alt + Espacio, escribí una consulta, seleccioná con ↑/↓ y presioná Enter o hacé
clic en el resultado. El logo incluye archivos; la calculadora funciona en ambos
modos. Abrir una aplicación/archivo o copiar un cálculo oculta y limpia la barra.
Si falla la apertura, conserva la consulta para reintentar. La barra vacía no
muestra texto de ayuda ni resultados.

Ubicaciones predeterminadas:

| Elemento | Ruta |
| --- | --- |
| Ejecutable | `~/.local/bin/rodro-turbo-search` |
| Acceso del menú | `~/.local/share/applications/io.github.rodro.TurboSearch.desktop` |
| Inicio automático | `~/.config/autostart/io.github.rodro.TurboSearch.desktop` |
| Icono instalado | `~/.local/share/icons/hicolor/scalable/apps/io.github.rodro.TurboSearch.svg` |
| Estado de desinstalación | `~/.local/share/rodro-turbo-search/install-state.json` |

Para desinstalar:

```sh
/usr/bin/python3 ~/.local/share/rodro-turbo-search/setup.py uninstall
```

Con `XDG_DATA_HOME`/`XDG_CONFIG_HOME` personalizados, usá las ubicaciones que
muestra el instalador. La desinstalación restaura el atajo guardado del menú de
ventana si el usuario no lo modificó después.

## Limitaciones conocidas

- GNOME/Wayland controla activación, posición, monitores y foco. No se garantiza
  posición inicial absoluta, estar siempre encima ni recibir foco en todos los
  entornos. El ocultamiento externo sigue eventos de foco/puntero, sin interceptar
  globalmente el mouse. El arrastre y el foco necesitan comprobación manual en el
  escritorio; la prueba gráfica automatizada no simula un arrastre del compositor.
- Los archivos requieren al menos dos caracteres y una espera de 200 ms después
  de escribir. Un único hilo busca solo nombres; omite entradas ocultas, enlaces
  simbólicos, `node_modules`, `target` y `vendor`. Comprueba límites de 30.000
  entradas, 16 descensos de directorio, 2.048 carpetas pendientes y 700 ms entre
  operaciones de disco. Si alcanza un límite informa resultados parciales. Los
  árboles grandes pueden omitir coincidencias; se saltan carpetas sin acceso.
  Un disco lento puede demorar el hilo más allá del presupuesto, aunque el
  recorrido se realiza fuera del hilo GTK.
- Las aplicaciones preceden a los archivos. Las consultas que parecen cálculos
  tienen prioridad sobre ambas búsquedas. Los archivos usan el manejador predeterminado.
- La aritmética usa coma flotante y unas 12 cifras significativas al presentar el
  resultado; no es aritmética decimal exacta. No hay separadores de miles.
- No hay interfaz de configuración, historial guardado ni persistencia del modo
  elegido después de finalizar el proceso. Las actualizaciones de instalación
  son atómicas por archivo, no una transacción sobre todos los archivos instalados.

## Comprobaciones y empaquetado local

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
/usr/bin/python3 tests/installer_test.py
```

Las pruebas del instalador utilizan directorios temporales y GSettings en memoria.
Necesitan los esquemas de GNOME instalados, pero no modifican los atajos reales.
La prueba gráfica opcional abre brevemente una ventana y ejecuta `/usr/bin/true`:

```sh
GDK_BACKEND=wayland cargo test --locked ui::tests::search_selection_launch_and_failure -- --ignored --test-threads=1
```

Dentro de una sesión gráfica, sin una instancia residente existente, la prueba
del ciclo de vida abre y oculta brevemente la barra real:

```sh
python3 tests/session_smoke.py target/debug/rodro-turbo-search
```

Comprueba IPC, reutilización de instancia, alternancia y salida; no la apariencia
ni el arrastre real. El repositorio actual no tiene flujos de GitHub Actions.

Para crear un paquete precompilado local de la arquitectura Linux actual:

```sh
./scripts/package.sh
```

Genera `dist/rodro-turbo-search-linux-<arquitectura>.tar.gz` sin publicar nada.
Extraelo y ejecutá su `./install.sh` para instalar sin Rust. No se promete la
existencia de releases precompiladas. Para compartir binarios, usá los scripts de
release: reemplazan las rutas locales de compilación y rechazan ejecutables que
contengan rutas del desarrollador. Antes de distribuir binarios, también se debe
proporcionar su código fuente correspondiente completo, incluidas dependencias
Rust enlazadas y scripts de compilación, como exige GPLv3; el paquete binario local
por sí solo no es una distribución completa de fuentes. Ver
[DEPENDENCIES.md](DEPENDENCIES.md) y [PUBLICATION.md](PUBLICATION.md).

## Estructura del repositorio

| Ubicación | Responsabilidad |
| --- | --- |
| `src/cli.rs`, `src/platform.rs` | Comandos, integración de sesión e instancia única |
| `src/catalog.rs`, `src/search.rs` | Descubrimiento y ranking de aplicaciones |
| `src/calculator.rs`, `src/files.rs` | Aritmética y búsqueda acotada de nombres |
| `src/ui.rs`, `src/background.rs` | Interfaz GTK y transición de color |
| `src/window_drag.rs`, `src/dismissal.rs` | Arrastre y manejo de foco |
| `scripts/` | Instalación por usuario, compilación release y empaquetado local |
| `tests/` | Comprobaciones del instalador y ciclo de vida gráfico |
| `assets/` | Recursos SVG |

[DECISIONS.md](DECISIONS.md) registra las decisiones de diseño en español.
[DEPENDENCIES.md](DEPENDENCIES.md) y [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
registran licencias y avisos conservados de las dependencias.

## Contribuir y reportar problemas

Utilizá los [Issues](https://github.com/RodrigoNaray/rodro-turbo-seach/issues)
del repositorio para errores reproducibles y propuestas. Incluí escritorio/sesión,
distribución, versiones GTK/Rust, pasos, resultado esperado y resultado observado.
Quitá rutas privadas, nombres de archivos y contenido del portapapeles de los
reportes y capturas. Para vulnerabilidades, seguí [SECURITY.md](SECURITY.md).

Se aceptan contribuciones pequeñas y enfocadas y correcciones de documentación.
Leé [CONTRIBUTING.md](CONTRIBUTING.md) antes de enviar un pull request. El mantenimiento
es personal y según disponibilidad; no hay calendario de releases ni compromiso
de tiempos de respuesta. Se aceptan reportes y contribuciones en inglés o español.

## Licencia y autor

Copyright © 2026 Rodrigo Naray.

El trabajo original del proyecto se distribuye bajo **GNU GPL versión 3 únicamente**
(`GPL-3.0-only`), sin autorización automática de versiones posteriores. Ver el texto
completo sin modificaciones en [LICENSE](LICENSE) y [COPYRIGHT](COPYRIGHT). Los
avisos de terceros y el arte derivado de fuentes conservan sus términos respectivos
en [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). No se ofrece garantía.
