# Informe de preparación para publicación

Revisión local realizada el 20 de octubre de 2026 sobre la base del commit
`895ec87`, con los cambios de preparación todavía sin commit. No se cambió la
visibilidad, el nombre del remoto, el código funcional del MVP ni su arquitectura.
No se ejecutó push, no se publicó una release y no se reescribió ni purgó historial.

## Archivos

Creados:

- `LICENSE`: texto oficial íntegro de GNU GPLv3, descargado de GNU y sin modificar.
- `COPYRIGHT`: autoría, selección explícita de GPL-3.0-only y alcance frente a terceros.
- `README.es.md`: versión española equivalente al README principal en inglés.
- `CONTRIBUTING.md`: requisitos y flujo para reportes y contribuciones.
- `SECURITY.md`: reporte privado condicional y tratamiento de datos locales.
- `THIRD_PARTY_NOTICES.md`: licencias y avisos upstream conservados.
- `PUBLICATION.md`: lista manual de publicación, presentación y distribución.
- `PREPARATION_REPORT.md`: este informe.

Modificados:

- `README.md`: presentación profesional en inglés, uso, instalación, límites y licencia.
- `Cargo.toml`: metadatos `license = "GPL-3.0-only"` y autor; dependencias intactas.
- `DEPENDENCIES.md`: inventario actualizado, compatibilidad y obligaciones de distribución.
- `.gitignore`: exclusiones adicionales de logs, temporales y archivos comprimidos.
- `scripts/package.sh`: inclusión de ambos README, documentos, licencia y avisos en
  el archivo local; no modifica la aplicación ni publica el paquete.

El SVG alternativo `assets/rodro-turbo-search-v1.svg` ya estaba sin seguimiento y se
conservó tal cual; no se incorporó a Git ni al paquete. El logo utilizado sigue
siendo `v2-rayo`. `src/`, Cargo.lock y los tests no tienen cambios.

## Licencia y recursos

**Copyright © 2026 Rodrigo Naray. GPL-3.0-only**, no GPL-3.0-or-later.
La selección se realiza en COPYRIGHT, Cargo.toml y los README; el texto oficial
de LICENSE conserva incluso sus ejemplos genéricos originales.

LICENSE coincide byte a byte con la descarga oficial de GNU y con la copia GPL-3
del sistema. SHA-256:
`3972dc9744f6499f0f9b2dbf76696f2ae7ad8af9b23dde66d6af86c9dfb36986`.

Se revisaron los metadatos y archivos de licencia de **64 paquetes Cargo**. No se
detectó una licencia incompatible entre las declaradas. Se eligió MIT cuando era
una alternativa, se conservaron Apache-2.0 WITH LLVM-exception de target-lexicon y
los términos Unicode-3.0 de unicode-ident. Se comprobó la conservación literal de
76 archivos de licencia/avisos de crates y del archivo upstream DejaVu 2.37.

Las bibliotecas nativas permanecen enlazadas dinámicamente y no se incluyen en el
paquete. La revisión de metadatos/avisos no equivale a una auditoría completa del
código upstream. El SVG contiene un glifo de DejaVu Serif convertido a contornos:
se conservaron sus términos de fuente; no se lo relicenció como software GPL.
Antes de publicar, el autor debe confirmar derechos sobre la composición original
del logo y cualquier otro material original. No se encontró un payload externo de
imagen ni un binario de fuente en el SVG.

Para futuras releases binarias debe proporcionarse el código fuente correspondiente
completo del build exacto, incluidas dependencias Rust enlazadas y scripts. El tar.gz
local tiene licencias/avisos pero no es un archivo completo de fuentes. Ver
DEPENDENCIES.md y PUBLICATION.md antes de distribuirlo.

Referencias primarias: [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt),
[compatibilidad GNU](https://www.gnu.org/licenses/license-list.html),
[licencia DejaVu](https://dejavu-fonts.github.io/License.html).

## Seguridad e historial

- Revisados **8 commits alcanzables**, 54 blobs históricos y sus nombres de archivo.
  Ambos campos de correo de todos los commits utilizan GitHub noreply.
- También examinados los **74 blobs y 14 commits almacenados localmente**, incluidos
  objetos no alcanzables, y el contenido de archivos actuales y del SVG no seguido.
- Sin coincidencias de los patrones revisados para claves privadas, tokens de GitHub,
  proveedores cloud/servicios, JWT, URLs con credenciales, secretos literales,
  rutas personales de home ni direcciones IP privadas en el contenido examinado.
- No hay binarios, temporales, configuración privada ni directorios de credenciales
  versionados. Se comprobaron exclusiones de `.env`, `.aws`, `.codex`, target, dist,
  logs y archivos comprimidos. No hay submódulos.
- **Hallazgo pendiente:** quedan 6 commits antiguos no alcanzables con el correo
  personal anterior. Se informó antes de continuar. No forman parte de las ramas
  que se envían con un push normal. No se modificaron reflogs, respaldos ni objetos.
  La solución opcional es una limpieza local autorizada tras preservar un respaldo
  privado; copias anteriores en otros servidores/clones requieren revisión separada.
- El paquete regenerado no contiene rutas personales detectadas ni metadatos de
  propietario/grupo, e incluye LICENSE y los documentos indicados.

La búsqueda de secretos se basó en inspección y patrones locales; no garantiza
la ausencia absoluta de secretos ni constituye una auditoría formal de seguridad.
Los nombres/correos de terceros que aparecen en licencias son atribuciones públicas
intencionales y se conservaron.

## Verificaciones ejecutadas

| Comprobación | Resultado |
| --- | --- |
| `cargo build --locked` | Correcto |
| `cargo fmt --check` | Correcto |
| `cargo clippy --locked --all-targets -- -D warnings` | Correcto |
| `cargo test --locked` | 12 correctas, 1 gráfica ignorada por defecto |
| Prueba gráfica opt-in en Wayland | 1 correcta |
| `/usr/bin/python3 tests/installer_test.py` | 4 correctas; temporales y GSettings en memoria |
| Prueba `tests/session_smoke.py` en bus aislado sin activación de servicios | Correcta: residente, instancia única, 40 toggles y salida |
| `./scripts/package.sh` | Archivo local generado; sin publicación |
| Sintaxis de scripts shell y `git diff --check` | Correctos |
| Enlaces Markdown locales de documentación | Sin destinos faltantes |
| Identidad del texto de GPL y avisos upstream | Correcta |
| GitHub Actions | No existen workflows en este repositorio |

La primera ejecución de la prueba de ciclo de vida en un bus aislado estándar
falló antes de validar el residente: `quit` agotó su timeout mientras se intentaban
activar portales/servicios de sesión, con errores FUSE. Se repitió la prueba existente
sin modificarla, con una configuración temporal D-Bus sin servicios autoactivables
y GTK_A11Y=none, GIO_USE_VFS=local y GTK_USE_PORTAL=0; pasó. No se detuvo la instancia
instalada del usuario. Esto no convierte esa prueba en una validación completa de
portales ni de otros escritorios.

Entorno de compilación: Linux x86_64, Rust 1.99.1 y GTK 4.14.5. La prueba gráfica
usó Wayland. No se verificaron otras distribuciones, escritorios, X11, arquitecturas,
ni versiones mínimas del compilador. La prueba gráfica no simula arrastre real ni
comprueba apariencia; foco, monitores y arrastre conservan esa limitación de cobertura.
Las muestras diagnósticas de memoria/CPU del smoke test no se publican como benchmarks.

## Configuración manual y recomendaciones

Consultar PUBLICATION.md para la descripción y los topics propuestos.

Pendientes: revisar/confirmar autoría, decidir sobre el SVG alternativo, revisar y
hacer commit de esta preparación, comprobar Issues/canal privado de vulnerabilidades,
y después decidir la publicación manual. No es necesario publicar binarios para
presentar el repositorio como proyecto open source.

Opcionales, sin implementar: capturas reales de ambos modos y calculadora,
demostración breve, imagen social, CI y mediciones reproducibles de consumo.
