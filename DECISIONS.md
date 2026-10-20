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

## 2026-10-19 — Apertura compacta y panel negro translúcido

A pedido del usuario, una consulta vacía (incluidos espacios) no devuelve
aplicaciones. La interfaz oculta resultados y ayuda hasta que se escribe, y limita
la lista a cinco coincidencias. Borrar la consulta o reabrir restaura la barra sola.

El fondo usa negro con alfa 0,82 y esquinas de 24 píxeles lógicos. Se aplica
transparencia al fondo, conservando texto e iconos opacos. Los estados de selección
son neutros para evitar el acento azul del tema. No se agrega desenfoque ni una
dependencia del compositor.

## 2026-10-19 — Ocultar al perder foco y retomar la consulta

La ventana se oculta cuando GTK notifica que deja de estar activa después de
haber recibido foco. No se interceptan clics globales; GNOME/Wayland comunica
el cambio de activación. La primera apertura espera a recibir foco para evitar
ocultarse durante el mapeo asíncrono.

Alt + Espacio conserva la consulta y la posición del cursor. Al refrescar el
catálogo, la selección se restaura por identificador de aplicación; si desaparece,
se selecciona el primer resultado disponible. Escape y el lanzamiento también
conservan la consulta. Esto reemplaza la limpieza al reabrir de la fase inicial.
El estado se mantiene únicamente en memoria durante la sesión del proceso.

## 2026-10-19 — Calculadora integrada

El usuario solicitó guardar los cambios de foco y continuidad en un commit y
avanzar con la calculadora. Se agrega un módulo puro con un parser aritmético
pequeño, sin dependencias nuevas ni evaluación de código. Respeta precedencia,
asociatividad de potencias, signos y paréntesis. Se limita a 1.024 bytes y 64
niveles de anidamiento para no bloquear la interfaz con entradas patológicas.

La interfaz reconoce expresiones numéricas y ofrece un resultado copiable con
Enter o clic. Una expresión incompleta no se confunde con una búsqueda sin
coincidencias. Las consultas con letras ajenas a notación científica siguen
buscando aplicaciones; el prefijo `=` permite forzar la calculadora.

Se usa f64 con presentación de 12 cifras significativas para reducir artefactos
visuales habituales. Se detectan divisiones por cero y resultados no finitos.
El porcentaje es un operador posfijo que divide por 100. No se agrega historial
ni persistencia, y el portapapeles se modifica solo al activar el resultado.

## 2026-10-20 — Limpiar después de activar un resultado

Al abrir correctamente una aplicación o copiar un cálculo con Enter o clic, se
oculta la barra y se vacía la consulta. Escape y la pérdida de foco conservan el
trabajo pendiente. Un fallo de lanzamiento conserva la consulta y muestra el error.
Antes de limpiar se liberan los préstamos de estado: el cambio de texto vuelve
a renderizar la interfaz de forma síncrona.


## Modo de archivos opcional

El logo es un ToggleButton accesible por teclado. Activarlo suma resultados de
archivos al catálogo de aplicaciones y cambia el fondo mediante un DrawingArea
con un barrido de color recortado a los bordes redondeados. El callback de frames
se retira al finalizar la transición; se respeta gtk-enable-animations.

La búsqueda de nombres usa solo std: un worker y un buzón de última consulta,
con generación atómica para cancelar resultados obsoletos. Se espera 200 ms antes
de solicitar trabajo. La UI consulta respuestas solo mientras hay una solicitud;
al ocultarse cancela el trabajo y el temporizador. No se agrega una dependencia,
base de datos, indexador ni vigilancia permanente. El alcance inicial es HOME,
con exclusiones de ocultos/builds, sin symlinks y con límites explícitos de tiempo,
entradas, profundidad y carpetas pendientes. Los límites pueden producir resultados
parciales, que se indican en la interfaz. Las operaciones de disco no son
interrumpibles, por eso se ejecutan fuera del hilo GTK.


## Foco durante el arrastre

El ocultamiento por pérdida de foco se evalúa al terminar el procesamiento de
los eventos GTK. Se comparte el estado de pulsación y movimiento con el manejo
de arrastre: una interacción interna o un movimiento nativo no ocultan la barra.
El puntero dentro de la interfaz también evita interpretar una pérdida transitoria
como un clic externo. Al volver el foco o recibir eventos sin botón presionado,
se limpia el estado de movimiento, porque Wayland puede consumir la liberación.
La comprobación usa eventos e idle de una sola ejecución, sin sondeo permanente.
