# Decisiones

## 2026-10-18 — Alcance: validar la fase 0

La primera entrega contiene únicamente la ventana, comandos y proceso residente.
No se adelantan catálogo, motor de búsqueda, Windows ni infraestructura futura.
La validación de foco en GNOME/Wayland es condición para continuar.

## GTK4 como candidato inicial, no decisión definitiva

Iced permite una interfaz Rust multiplataforma y un ciclo residente, pero su
API `GainFocus` documenta que Wayland no está soportado. Sus capacidades de
posición también están restringidas en Wayland. GTK4 ofrece integración directa
con GApplication, D-Bus y activación del escritorio, y permite ocultar y reutilizar
la ventana con `set_visible` / `present`. Se elige GTK4 para el primer prototipo
por reducir la integración propia. No se hizo una comparación de consumo ni un
prototipo ejecutable de Iced; no se afirma superioridad de rendimiento.

La documentación de `GtkWindow.present` deja el foco a criterio del gestor de
ventanas. Se pasa el token de activación del proceso que ejecuta `toggle`, cuando
existe, antes de presentar la ventana. `grab_focus` enfoca el campo dentro de GTK;
no fuerza al compositor a transferir el foco. No se inventan tokens ni se reutiliza
el entorno inicial para futuras invocaciones.

No se garantiza posición absoluta ni monitor activo en GNOME/Wayland con una
ventana normal. Si la prueba real de foco falla, se detiene la siguiente fase y
se evalúa el flujo de activación antes de considerar otra biblioteca o una
extensión de GNOME (que agregaría dependencia de versiones de Shell).

Referencias oficiales consultadas:

- [Iced Window Action](https://docs.rs/iced/latest/iced/window/enum.Action.html)
- [GTK present](https://docs.gtk.org/gtk4/method.Window.present.html)
- [GTK startup ID](https://docs.gtk.org/gtk4/method.Window.set_startup_id.html)
- [GTK Wayland](https://docs.gtk.org/gtk4/wayland.html)
- [GApplication](https://docs.gtk.org/gio/class.Application.html)

## Ciclo de vida e instancia única

GApplication identifica la aplicación mediante `io.github.rodro.TurboSearch` y
remite comandos a la instancia principal en el bus de sesión. Un guard de `hold`
mantiene el proceso vivo aun oculto. Se conserva una ventana, sin trabajo periódico.
`quit` sale del bucle y libera los recursos. No se mantiene un servidor o socket
propio. Solo se reenvía el entorno para recuperar el token del invocador; no se
persiste ni registra su contenido.

## Estructura mínima

Un crate con módulos por responsabilidad. GTK queda en interfaz e integración;
no se añaden traits para un backend Windows todavía inexistente. Se prohíbe
`unsafe` en el código propio. El tema usa colores del sistema y tamaños lógicos.

## 2026-10-18 — Búsqueda e instalación por usuario

El usuario confirmó que las pruebas de la ventana funcionaron y pidió una
experiencia instalable con atajo y búsquedas, sin configuración manual. Esto
habilita avanzar al catálogo y la búsqueda, y configurar la integración desde
el instalador explícito. La ejecución normal del buscador no cambia GSettings.

GIO descubre las aplicaciones visibles y las lanza con el contexto de activación
de GDK. El motor de ranking recibe solamente nombres e identificadores. Se
actualiza el catálogo al abrir, sin agregar threads, servicios o watchers.

Se entrega un instalador por usuario y un archivo tar.gz con el binario release.
Esta primera distribución está dirigida a GNOME en Mint 22.3 / Ubuntu 24.04 x86_64;
no se presenta como paquete universal. El instalador configura el atajo y XDG
Autostart sin root, conserva la asignación original del menú de ventana y permite
desinstalar. La instalación repetida actualiza el binario sin duplicar accesos.
La desinstalación conserva otros atajos y modificaciones posteriores del usuario.

El instalador usa Python 3 y PyGObject del sistema para acceder a GSettings de
forma tipada. No se agregan dependencias Rust ni un gestor de paquetes propio.
La restauración completa ante fallos está implementada para una primera instalación;
una actualización usa reemplazos atómicos por archivo, sin transacción global.
