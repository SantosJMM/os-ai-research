# TUP histórico — Acciones, seguridad y Task Runtime

> **Reconstrucción de las conversaciones de 27–28 de octubre de 2026.** No es transcripción literal ni una decisión técnica oficial posterior. Los nombres de tipos y acciones anotados aquí se recuperaron del historial contextual; los detalles que no se pudieron comprobar quedan marcados como propuestas.

## 1. La pregunta arquitectónica de fondo

¿Cómo puede un agente de IA abrir un proyecto, inspeccionarlo y ejecutar pruebas sin conseguir privilegios directos, y cómo puede Thanos Shell mostrar ese proceso sin depender de una ventana específica?

La respuesta de trabajo separaba **intención**, **autorización**, **ejecución** y **presentación**.

~~~text
Usuario / IA
    |
    v
Acción semántica tipada
    |
    v
Validación de sesión, identidad, estado y argumentos
    |
    v
Política/autorización de Ship (cuando corresponda)
    |
    v
Task Runtime / worker aislado / API de aplicación
    |
    +----> progreso y eventos ----> Thanos Shell / Capsuleer
    |
    +----> resultado y auditoría -> Atlas
~~~

Es un flujo conceptual; una aplicación puede ejecutar una operación local no privilegiada sin pasar cada método por Ship, pero **ninguna operación protegida puede evitar la autoridad efectiva**.

## 2. Tipos Rust discutidos

En las propuestas aparecieron explícitamente los nombres:

- **Task**, **TaskState**: identidad y estado de un trabajo.
- **UiNode**, **View**: estructura declarativa de interfaz.
- **AppManifest**: identidad, acciones y capacidades de una aplicación.
- **ActionRequest**: solicitud de ejecutar una acción tipada.
- **ProjectAction**: variantes **Open**, **Explore**, **RunTests**.
- **ActionStatus**: resultado o estado de la solicitud.
- **ApprovalScope**: alcance de una aprobación.

La forma exacta de las declaraciones Rust originales **no está disponible en este archivo**. El siguiente código sirve como reconstrucción orientativa, no como cita textual ni API publicada:

~~~rust
// ILUSTRATIVO, NO COPIA LITERAL DE LA CONVERSACION
struct Task {
    id: String,
    kind: String,
    state: TaskState,
}

enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Denied,
    Cancelled,
    Interrupted,
}

enum ProjectAction {
    Open { path: String },
    Explore { project_id: String },
    RunTests { project_id: String, suite: Option<String> },
}

struct ActionRequest {
    request_id: String,
    session_id: String,
    action: ProjectAction,
    view_revision: Option<u64>,
}
~~~

Este código **no define** un esquema normativo, no incluye autorizaciones reales y tampoco establece transiciones de estado completas.

## 3. Caso práctico: proyecto → explorar → pruebas

### Paso A — Abrir

Solicitud de operación **project.open**. Se verifica el contexto de cliente, su identidad, el proyecto solicitado y las rutas permitidas.

### Paso B — Explorar

Solicitud **project.explore**. La exploración debería ejecutarse con permisos mínimos y devolver un resultado estructurado para Capsuleer y para Thanos Shell.

### Paso C — Ejecutar pruebas

Solicitud **project.test.run**. La operación puede necesitar aprobación según el perfil y debe ejecutarse en un **test-worker aislado**, con los recursos apropiadamente restringidos.

### Paso D — Observar y concluir

El Task Runtime registra el progreso y la terminación de la tarea. TUP puede transmitir las actualizaciones semánticas a la vista y a Capsuleer. Atlas almacena la evidencia de las operaciones auditables.

La conversación discutió **aprobación con alcance (ApprovalScope)** en vez de interpretar una aprobación como permiso global o permanente.

## 4. Verificación de identidad y validación

La propuesta v0.2 reforzó estos aspectos:

- Tomar identidad real del cliente desde credenciales del socket o de una sesión autenticada, no de un campo arbitrario del JSON.
- Comprobar que la acción esté registrada y sus parámetros cumplan su esquema.
- Vincular la solicitud a vista y revisión cuando la operación se origine en controles de UI, para evitar acciones sobre contexto obsoleto.
- Usar **request IDs / deduplicación** para limitar la repetición accidental de operaciones tras reintentos.
- Evitar que Capsuleer obtenga un canal lateral con permisos superiores a los definidos por Ship.

Ninguno de estos controles puede resolverse solo con una descripción textual para el modelo.

## 5. Task Runtime como servicio separado

En una conversación posterior se propuso un **Task Runtime en Rust, sin root**, con responsabilidades propias:

- Crear, consultar y listar tareas.
- Administrar estados y transiciones de trabajo.
- Persistir estado, eventos y resultados.
- Gestionar cancelación, reintentos y recuperación.
- Representar dependencias entre tareas.
- Entregar progreso y eventos a clientes suscritos.
- Distinguir un trabajo cancelado de uno que falló, fue denegado o quedó interrumpido.

API discutida para investigación:

| Método | Intención |
| --- | --- |
| **task.create** | Registrar un trabajo |
| **task.get** | Consultar el estado y resultado |
| **task.list** | Enumerar trabajos |
| **task.cancel** | Solicitar cancelación |
| **task.subscribe** | Observar actualizaciones |
| **task.complete** | Registrar finalización desde un ejecutor autorizado |

**Advertencia:** que estos nombres aparezcan en la propuesta no implica que el acceso a **task.complete** fuese seguro desde cualquier cliente. Las transiciones deben estar controladas por identidad y por la propiedad de la tarea.

### TUP no es un scheduler de procesos

TUP se propuso como lenguaje semántico entre aplicaciones, clientes y el Shell. **Task Runtime** sería quien mantuviera la máquina de estados del trabajo. **Station** y **Linux** seguirían gestionando servicios, procesos y señales. **Ship** evaluaría acciones privilegiadas. Los *workers* serían ejecutores concretos de tareas.

## 6. Ciclo de vida y recuperación

Se plantearon estados terminales **completed**, **failed**, **denied**, **cancelled**, **interrupted**. También se discutió que:
- Una desconexión de cliente no debe hacer que toda tarea desaparezca, si la tarea fue diseñada para persistir.
- Una tarea reiniciada o interrumpida debe conservar evidencia del estado anterior.
- Eventos perdidos, duplicados o retrasados requieren revisión/secuencia y una política explícita.
- Los consumidores necesitan poder volver a obtener una instantánea coherente.
- La cancelación puede ser cooperativa y no equivale a revertir automáticamente efectos ya aplicados.

**No estaba cerrado** si el registro TUP sería persistente, cómo se garantizaría entrega de eventos ni cómo se implementarían reintentos transaccionales.

## 7. Daemons, procesos y mecanismos existentes

La evolución del planteamiento incluyó aplicaciones gráficas y **servicios en segundo plano**. Un daemon puede exponer estado, métodos y eventos sin nueva ventana ni UI obligatoria.

TUP **no reemplaza**:
- Las señales de Linux como SIGTERM o SIGKILL.
- Las llamadas del kernel para inspeccionar o terminar procesos.
- Los supervisores y mecanismos de arranque de servicios.
- Los mecanismos de aislamiento como cgroups, namespaces o seccomp.
- Los protocolos propios de PostgreSQL u otros servicios.

La utilidad buscada era ofrecer a la IA **comprensión semántica e interacción** con esas aplicaciones y servicios, utilizando siempre mecanismos autorizados de ejecución.

## 8. Decisiones abiertas y transición posterior

Algunas preguntas que la investigación debía resolver:
1. ¿Endpoint TUP directo o registro/broker?
2. ¿Qué campos, métodos y errores son obligatorios?
3. ¿Qué modelos de consistencia y suscripción pueden prometerse?
4. ¿Cuál es la relación exacta de TUP con Task Runtime y Thanos Shell?
5. ¿Qué capacidades deberían integrarse con MCP y MCP Apps en lugar de duplicarse?

La investigación posterior pasó a llamarse **[SASP — Semantic Application & Service Protocol](../../sasp/README.md)**. La etapa TUP sigue siendo importante como registro del pensamiento original de Thanos OS, pero no debe presentarse retroactivamente como una implementación terminada.
