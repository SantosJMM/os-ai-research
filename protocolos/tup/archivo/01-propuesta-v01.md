# TUP v0.1 — Reconstrucción de la propuesta histórica

> **Tipo de documento:** reconstrucción histórica, no transcripción literal.  
> **Conversaciones de referencia:** 27 y 28 de octubre de 2026.  
> **Estado:** propuestas de diseño; no se afirma que estén implementadas ni aprobadas como estándar.

## 1. Visión original

**Thanos UI Protocol (TUP)** se propuso para que **Thanos Shell y la inteligencia artificial compartieran un lenguaje estructurado de tareas, vistas y aplicaciones**. No consistía en permitir que la IA pulsara coordenadas en una pantalla. Se buscaba hacer explícitas las funciones, el estado y las acciones accesibles al agente y a la interfaz humana.

El diseño temprano tenía tres entidades principales:

| Entidad | Responsabilidad conceptual |
| --- | --- |
| **Task** | Representar trabajo con identificador, ciclo de vida, progreso, resultado, cancelación y recuperación. |
| **View** | Representar una interfaz declarativa y estructurada, compuesta por nodos que el Shell podría renderizar. |
| **App** | Identificar una aplicación, su manifiesto, instancias, estado, eventos y acciones registradas. |

No todas las capacidades debían pertenecer definitivamente al protocolo: había decisiones abiertas sobre qué gestionarían TUP, Thanos Shell o el **Task Runtime**.

## 2. Componentes de Thanos OS involucrados

~~~text
Persona                           Capsuleer (agente aislado)
    |                                      |
    v                                      v
Thanos Shell <------ contrato TUP ------> clientes semánticos
    |                                      |
    +-------------- eventos --------------+
                        |
           Aplicaciones / servicios
                        |
                 Task Runtime
            trabajos y persistencia
                        |
            solicitud de operaciones
                        v
              Ship (política/exec)
                        |
                 Linux / recursos
                        |
                Atlas (auditoría)
~~~

**Alcance del esquema:** ilustra relaciones funcionales estudiadas, no las rutas reales de IPC de una implementación concreta. *Station* supervisa servicios como PID 1. *Capsuleer* no tiene privilegios directos de kernel. *Ship* es el ejecutor privilegiado sujeto a políticas. *Atlas* mantiene evidencia de auditoría.

En la fase original el nombre **TUP** estaba ligado a Thanos OS. Posteriormente se adoptó **SASP** para investigar el contrato de aplicaciones/servicios de forma independiente.

## 3. Task — Trabajo identificable

La entidad *Task* se estudió como un trabajo que puede seguir existiendo sin que una ventana esté abierta y cuyo progreso puede observarse por múltiples clientes.

Dimensiones discutidas:
- Identidad estable por tarea, origen y descripción.
- Estados de ejecución y transiciones definidas.
- Progreso, eventos parciales, error, resultado y referencia al historial.
- Solicitudes de cancelación y resultado terminal inequívoco.
- Persistencia y recuperación en caso de reinicio o caída.
- Distinción entre intención solicitada, aceptación del trabajo, ejecución y resultado.

**Estados mencionados en nuestras conversaciones:** *completed*, *failed*, *denied*, *cancelled*, *interrupted*. Otros estados intermedios (p. ej. *queued* o *running*) aparecían como parte del diseño general; **no se conserva aquí como definitivo un enumerado exacto**.

El Task Runtime se planteó después como servicio Rust **sin root**, dedicado a administrar el ciclo de vida de tareas. TUP podía transportar eventos de las tareas, pero **no debía absorber toda la lógica de ejecución**.

## 4. View — Presentación declarativa

La entidad *View* se concibió como una representación estructurada para humanos y clientes semánticos, no como un fragmento arbitrario de código que el agente ejecuta.

El concepto histórico **UiNode** representaba nodos de una vista. Entre los objetivos:
- Estructura y contenido reconocible por el agente.
- Identificadores estables para asociar controles con acciones registradas.
- Revisión o versión de vista para detectar acciones basadas en estados obsoletos.
- Datos de presentación desacoplados de la autorización y de la lógica del negocio.
- Renderizado económico a partir de información estructurada y no de capturas recurrentes.

**Pregunta abierta histórica:** qué variedad de nodos admite una View, dónde viven sus esquemas de validación y cómo se concilia una interfaz declarativa con aplicaciones GUI tradicionales.

## 5. App — Manifiesto y acciones

La entidad *App*, junto con el concepto **AppManifest**, se propuso para que el Shell y Capsuleer pudieran reconocer una aplicación, localizar una instancia y descubrir las acciones que publicaba.

Se investigaron:
- Identidad de aplicación e instancia.
- Descripción de capacidades, métodos y acciones tipadas.
- Disponibilidad y salud de la interfaz.
- Estado y eventos observables.
- Relación entre la instancia de la app y su proceso o servicio real.
- Ciclo de vida de la aplicación separado del de su endpoint TUP.

Posteriormente se distinguieron dos dimensiones distintas:

| Proceso o aplicación | Disponibilidad de interfaz TUP |
| --- | --- |
| *starting*, *running*, *stopping*, *stopped*, *failed* | *initializing*, *ready*, *unresponsive*, *disconnected* |

Son **conjuntos de estados distintos**, no una correspondencia de filas obligatoria: un proceso puede continuar ejecutándose aunque su endpoint TUP esté desconectado.

## 6. Transporte temprano

Las conversaciones iniciales propusieron **JSON-RPC 2.0 sobre sockets Unix**, con **encuadre de longitud** para delimitar mensajes. Se discutían eventos asíncronos, respuestas a solicitudes y sesiones.

La elección de transporte **no estaba cerrada**: también quedó pendiente evaluar si cada aplicación publicaría su socket directamente o se registraría a través de un *broker/registry*.

El protocolo semántico no debía depender de una única estrategia de transporte ni otorgar privilegios por el simple hecho de establecer una conexión.

## 7. Eventos conocidos de la propuesta

Los nombres que sí se identifican de las conversaciones anteriores incluyen:

- **task.progress** — progreso de una tarea.
- **task.completed** — finalización de una tarea.
- **view.publish** — publicación o actualización de una vista.

También se discutieron snapshots, revisiones, suscripciones, identificación de instancias y mecanismos para recuperar el estado después de desconectarse.

**No se atribuye a estos nombres un esquema JSON definitivo**. Las formas que aparecen en el anexo de ejemplos son propuestas de reconstrucción para estudio.

## 8. Un caso de uso trabajado

El flujo histórico fue: «abrir un proyecto → explorar → ejecutar pruebas».

1. Una persona o cliente solicita abrir un proyecto.
2. La intención se representa con una acción tipada, por ejemplo **project.open**.
3. Se establece contexto de aplicación, vista y revisión.
4. La IA propone **project.explore** o **project.test.run** según el objetivo.
5. La solicitud se valida frente al estado y la identidad del cliente.
6. Las operaciones protegidas se someten a política de **Ship**.
7. La ejecución de pruebas ocurre en un **worker aislado**; no es un privilegio inherente a TUP.
8. El trabajo publica progreso, resultado y evidencias; **Atlas** audita operaciones relevantes.
9. El Shell y el agente pueden consultar o visualizar el mismo estado.

Se mencionaron tipos conceptuales **ActionRequest**, **ProjectAction** con variantes **Open / Explore / RunTests**, **ActionStatus**, y **ApprovalScope**. Sus campos exactos requieren consultar la transcripción literal para archivarlos palabra por palabra.

## 9. Distinción entre propuesta e implementación

En la investigación histórica se bosquejaron tipos Rust como **Task**, **TaskState**, **UiNode**, **View** y **AppManifest**. La existencia de esos nombres en el diseño **no acredita una implementación funcional de TUP**.

Para mensajes completos con JSON de ejemplo, véase [02 — Ejemplos JSON](02-ejemplos-json.md). Para permisos y ejecución, véase [03 — Acciones y Runtime](03-acciones-y-runtime.md).

## 10. Decisiones que quedaron abiertas

- Socket Unix directo por aplicación frente a broker/registro.
- Esquema y versionado normativos de entidades TUP.
- Qué partes son responsabilidad de TUP, Task Runtime y Thanos Shell.
- Cómo adaptar aplicaciones tradicionales sin imponerles una GUI nueva.
- Garantías de entrega, duplicación y recuperación de eventos.
- Relación entre acciones semánticas y mecanismos de seguridad efectivos.

Estas decisiones se preservan como **preguntas**, no como características ya resueltas.
