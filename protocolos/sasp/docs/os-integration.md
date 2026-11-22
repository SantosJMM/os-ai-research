# SASP — Integración de MCP y MCP Apps con el sistema operativo

> **Estado:** dirección arquitectónica y programa de investigación. No define una versión normativa de SASP ni afirma que exista una implementación completa.

Antes de definir interfaces nuevas, SASP toma como base las capacidades disponibles de **MCP 2026-07-28** y estudia la integración con un sistema anfitrión. La [DECISION-0002](../DECISION-0002-integracion-mcp-en-so.md) establece esta orientación.

## 1. Identidad y objetivo

**SASP — Semantic Application & Service Protocol** investiga cómo integrar aplicaciones y servicios de un sistema operativo con los mecanismos de **Model Context Protocol (MCP)** y, cuando aporten valor, **MCP Apps**.

El propósito no es sustituir MCP, inventar un RPC paralelo, ni limitarse a agrupar herramientas en un documento JSON. SASP investiga **cómo se instala, registra, descubre, conecta, observa y gestiona software en un entorno operativo con agentes de IA y clientes humanos**.

**Principio rector:** las aplicaciones y servicios declaran sus capacidades; los clientes las descubren y solicitan operaciones. Un agente no inventa las capacidades publicadas ni adquiere permisos por descubrirlas. La autoridad efectiva permanece en el proveedor y en el sistema anfitrión.

La arquitectura debe funcionar con aplicaciones gráficas, aplicaciones sin ventana, daemons y servicios. **Thanos OS** es el primer contexto de referencia, no una dependencia obligatoria.

## 2. Qué existe en MCP y qué no debe duplicarse

La comparación de referencia usa **MCP 2026-07-28**, la extensión **MCP Apps 2026-01-26**, la extensión **MCP Tasks** y el **MCP Registry** oficial.

| Necesidad | Capacidad MCP existente | Pregunta específica para SASP |
| --- | --- | --- |
| Descubrir herramientas | `tools/list`, `tools/call` y sus JSON Schemas | ¿A qué aplicación e instancia corresponde cada herramienta? |
| Exponer estado | `resources/list` y `resources/read` | ¿Cómo describir y consultar el estado de aplicaciones y servicios del SO? |
| Observar cambios | `subscriptions/listen` para listas y recursos | ¿Basta notificar recursos modificados o necesitamos eventos tipados con historial? |
| Descubrir capacidades de servidor | `server/discover` | ¿Cómo encuentra el sistema operativo **qué servidores o aplicaciones hay**? |
| Distribuir servidores | MCP Registry y registros del ecosistema | ¿Cómo registrar instalaciones **locales** e instancias activas de un equipo? |
| Operaciones prolongadas | Extensión `io.modelcontextprotocol/tasks` | ¿Cómo relacionarla con tareas, procesos y recuperación a nivel de SO? |
| Presentación interactiva | MCP Apps: recursos `ui://`, metadatos de herramientas y HTML aislado | ¿Cómo conviven MCP Apps, GTK/Qt y los **Presentadores** o interfaces nativas del SO? |
| Transporte | MCP `stdio`, Streamable HTTP y transportes personalizados | ¿Cuál es el canal autorizado para cada tipo de proceso e instancia local? |
| Autorización | MCP define mecanismos de autorización, especialmente para HTTP | ¿Cómo imponer credenciales, privilegios y políticas nativas de cada SO? |

### Diferencia entre los tres tipos de descubrimiento

1. **MCP Registry (ecosistema):** catálogo de metadatos e instrucciones de distribución/configuración de servidores MCP, no inventario en vivo de aplicaciones ejecutándose en un host.
2. **`server/discover` (protocolo):** describe versiones y capacidades de **un endpoint MCP ya localizado**. No busca automáticamente servicios instalados en Linux.
3. **SASP Discovery (integración de SO, propuesta):** consulta el **catálogo local** y las **instancias activas** para encontrar aplicaciones por identidad o capacidad, aplicando filtros de visibilidad y autorización.

**SASP Registry** y **SASP Discovery** son roles diferenciados: uno mantiene la verdad operativa del host; el otro ofrece mecanismos seguros para consultarla. Pueden residir en el mismo servicio o en procesos distintos.

## 3. Seis ejes de diseño y experimentación

### Eje 1 — Application & Service Contract

**Pregunta:** ¿qué necesita declarar una aplicación para que el SO, un agente y una interfaz humana la comprendan?

- Identidad lógica, clase de proveedor y versiones.
- Métodos y recursos publicados por MCP; sus esquemas.
- Estado observable y, opcionalmente, revisiones.
- Eventos disponibles y contexto de presentación opcional.
- Relación entre proveedor lógico e instancia de ejecución.
- Efectos y permisos como información explicativa, **nunca** como garantía de autorización.

**Primero reutilizar:** MCP Tools, Resources, JSON Schema, metadatos y MCP Apps.

**Añadir únicamente si es necesario:** convenciones portables para identidad de entidad, estado revisable, contexto y eventos de dominio.

### Eje 2 — SASP Registry

**Pregunta:** ¿cómo se integra un proveedor MCP con el registro e instalación del sistema operativo?

Se propone separar **dos inventarios**, aunque una implementación podría almacenarlos conjuntamente:

| Inventario | Fuente de verdad candidata | Datos representativos |
| --- | --- | --- |
| **Instalaciones** | Gestor de paquetes / instalador / integración autorizada | package ID, proveedor, versión, origen, mecanismo de arranque, capacidades declaradas |
| **Instancias activas** | Supervisor del SO / runtime local / proceso autenticado | identidad de instancia, estado operativo, PID o handle interno, endpoint, identidad verificada, fecha de activación |

El SO **no debe confiar automáticamente** en un `provider.id` o en un PID escrito en JSON. El registro de ejecución requiere vincular la instancia con una identidad comprobada y evitar reutilización accidental de PIDs; las políticas del host deciden quién puede anunciar o modificar registros.

Un proveedor instalado puede estar inactivo; un endpoint desconectado puede pertenecer a un proceso aún vivo. No se debe confundir esos estados.

**No es MCP Registry:** el catálogo público puede contribuir información de instalación, pero no sustituye el registro local confiable del host.

Las reglas de verificación, reconciliación, identidades y casos de prueba se desarrollan en [Registry y Discovery](registry-discovery.md).

**Reutilización a investigar:** el [formato `server.json` del MCP Registry](https://github.com/modelcontextprotocol/registry/blob/main/docs/reference/server-json/generic-server-json.md) ya define metadatos de distribución y configuración de servidores. El catálogo de instalaciones SASP **debería evaluar su reutilización** antes de crear otro manifiesto de paquetes. La información del **runtime local** (instancia, estado del proceso, identidad autenticada y endpoint disponible) sigue necesitando mecanismos del SO, aunque se utilicen componentes de un registro compatible.

### Eje 3 — SASP Discovery

**Pregunta:** ¿cómo descubre un cliente las aplicaciones y servicios disponibles para él?

El servicio de descubrimiento podría presentarse **como servidor MCP del sistema operativo**, con herramientas y recursos propios para:

- Buscar proveedores por nombre, clase o capacidad.
- Obtener instancias disponibles y su estado de conexión.
- Resolver una instancia hacia una ruta lógica de comunicación autorizada.
- Consultar versiones y capacidades soportadas.
- Filtrar resultados por identidad y permisos del solicitante.

Los nombres de operaciones como `applications.search` o `applications.resolve` son **ejemplos de API de investigación**, no nuevas llamadas obligatorias de MCP.

**Decisión importante:** el resultado de descubrimiento puede ser un handle opaco o una ruta mediada, no necesariamente la dirección física de un socket a la que un agente pueda conectarse sin controles.

### Eje 4 — SASP Connection

**Pregunta:** ¿cómo pasa un consumidor del descubrimiento a una sesión operativa, y quién mantiene la autoridad?

Se investigan las siguientes vías:

1. **`stdio` administrado:** el host lanza un servidor MCP que trabaja como subproceso; útil para herramientas efímeras y apps con ejecución controlada.
2. **Socket Unix directo:** un proveedor activo expone un endpoint local. MCP 2026-07-28 permite transportes personalizados y recomienda reutilizar el framing de `stdio` sobre streams fiables como Unix domain sockets.
3. **Conexión mediada:** un broker o gateway autorizado verifica identidad y encamina al proveedor; útil si el cliente está aislado, no debe recibir rutas directas o hay múltiples clientes.
4. **Streamable HTTP:** para proveedores remotos o acceso controlado; se aplican sus mecanismos correspondientes de autorización.

Antes de invocar métodos, el cliente necesita compatibilidad MCP (en 2026-07-28, `server/discover` puede ayudar) y el sistema debe aplicar comprobaciones efectivas de identidad y política.

No se debe confundir el concepto de **sesión funcional del proveedor** con una sesión de protocolo MCP 2026-07-28: esa revisión eliminó las sesiones de protocolo obligatorias. El contexto de una aplicación puede representarse con handles explícitos si hace falta.

### Eje 5 — SASP Lifecycle & Events

**Pregunta:** ¿cómo observa el SO cambios en apps, servicios, instancias y tareas, y qué garantías necesitan realmente los consumidores?

Debe diferenciarse entre:

- **Instalación:** instalada, actualizada, desinstalada.
- **Proceso:** iniciando, ejecutándose, detenido, fallido; responsabilidad del supervisor y el SO.
- **Endpoint MCP:** anunciado, disponible, inaccesible, reconectado.
- **Estado funcional:** cambio de pista, backup iniciado/finalizado, datos propios del dominio.
- **Tarea:** ejecución y resultados asíncronos; MCP Tasks puede ser suficiente en operaciones MCP, sin asumir que reemplaza un Task Runtime general.

**MCP ya aporta observación:** `subscriptions/listen` notifica cambios de catálogos y recursos; MCP Tasks admite `notifications/tasks`. Esto permite implementar un caso inicial en el que el cliente observa la actualización de un recurso y vuelve a consultar el estado.

**Límite a medir:** `notifications/resources/updated` notifica un cambio de URI, pero no es por sí misma un evento de dominio `playback.changed` con carga tipada y replay persistente. Una extensión SASP para eventos de dominio solo se justifica si una prueba requiere orden, retención, recuperación o historial que no pueda expresarse suficientemente reutilizando recursos/herramientas y extensiones existentes.

Se deben probar pérdidas, duplicados, reconexiones y cambios de instancia sin prometer entrega *exactly once* de manera implícita.

### Eje 6 — MCP Integration & Presentation

**Pregunta:** ¿cómo utilizar MCP y MCP Apps como base de interacción dentro de un SO, sin forzar que toda aplicación sea HTML?

- El **núcleo** consume MCP Tools, Resources, suscripciones y extensiones.
- **MCP Apps** es una posibilidad de presentación interactiva para hosts compatibles, con recursos `ui://` y sandbox HTML; **no reemplaza** por sí misma una ventana nativa GTK/Qt.
- El **Presentador** puede mostrar estado y acciones mediante una UI propia a partir de recursos y contratos de las aplicaciones.
- Los daemons y servicios sin ventana pueden participar **sin publicar ningún recurso `ui://`**.
- Si las necesidades de presentación nativa no se cubren con MCP Apps, definir una **extensión SASP de contexto de interfaz**, separada del transporte y del registro, después de comprobar con prototipos su necesidad.
- Los adaptadores deben mantener identidad y permisos, y no convertir metadatos no confiables de servidores en instrucciones privilegiadas.

## 4. Flujo completo propuesto: instalar → registrar → descubrir → conectar

```text
       Gestor de paquetes / Instalador
                    |
           instalación validada
                    v
        +-----------------------+
        | Catálogo local SASP   |
        | provider/package      |
        +-----------------------+
                    |
           supervisor inicia app
                    v
        +-----------------------+
        | Instancia activa      |
        | proceso + endpoint    |
        +-----------------------+
                    |
         registro con identidad
               verificada
                    v
        +-----------------------+
        | SASP Registry         |
        | instalado / activo    |
        +-----------------------+
                    |
          búsqueda por capacidad
                    v
        +-----------------------+
        | SASP Discovery        |<-- Agente / Presentador
        | (interfaz MCP posible)|
        +-----------------------+
                    |
        handle/ruta bajo política
                    v
        +-----------------------+
        | Conexión MCP          |
        | stdio / Unix / bridge |
        +-----------------------+
                    |
              herramientas
             recursos / eventos
                    v
        +-----------------------+
        | Proveedor             |
        | app / servicio        |
        +-----------------------+
```

El **registro local** puede ser una fuente común de datos para Discovery y el gestor de tareas. El **broker es opcional**: introducirlo debe responder a necesidades de aislamiento, auditoría o multiplexación demostradas, no ser requisito nominal de SASP.

## 5. Ejemplo de descubrimiento semántico

Una persona pide: «Busca un reproductor musical instalado y dime si está reproduciendo».

1. El cliente consulta a **SASP Discovery** mediante una herramienta MCP del SO.
2. Discovery encuentra el proveedor instalado y, si procede, su instancia activa, **filtrando los datos por permisos del cliente**.
3. El cliente obtiene una ruta lógica permitida hacia el endpoint; si no existe instancia, puede solicitar al SO iniciar una, **con autorización aparte**.
4. El cliente MCP lee el manifiesto o catálogo de herramientas, consulta el recurso de estado y obtiene `playback`.
5. Si se requiere observar cambios, se suscribe al recurso; si recibe actualización, vuelve a leerlo.
6. Si se requiere cambiar volumen, invoca la herramienta con argumentos tipados y el proveedor o la autoridad aplica la política.

**Descubrimiento no equivale a arranque, conexión, autorización ni ejecución**. Cada etapa tiene un propietario y controles diferentes.

## 6. Seguridad e integración en Thanos OS

En Thanos OS, una implementación candidata sería:

- **Station (Supervisor):** supervisión y ciclo de vida real del proceso/servicio.
- **SASP Registry:** catálogo de instalaciones e instancias, abastecido por mecanismos confiables del SO.
- **SASP Discovery:** interfaz de consulta, potencialmente un servidor MCP del propio SO.
- **Agente:** cliente restringido que descubre capacidades y solicita operaciones.
- **Thanos Shell (Presentador):** cliente humano de estado y acciones; puede renderizar MCP Apps o UI nativa según el caso.
- **Ship (Autoridad de seguridad / Ejecutor privilegiado):** aplica políticas y operaciones protegidas según el perfil; el agente no obtiene acceso root ni rutas de bypass.
- **Atlas (Subsistema de auditoría):** registro de operaciones y eventos relevantes.

Es una asignación **candidata**, no una implementación ya aprobada del núcleo de Thanos OS. Un SO distinto puede aportar otros supervisores y políticas sin cambiar el objetivo de SASP. La [terminología general y sus correspondencias](terminologia.md) evitan que los nombres específicos de Thanos OS se conviertan en requisitos.

La separación entre identidad de proveedor, identidad autenticada del proceso y permiso de invocación es obligatoria como principio de seguridad.

## 7. Criterio para definir extensiones SASP

Una función nueva solo entra al núcleo o a una extensión después de responder:

1. ¿Existe ya en MCP 2026-07-28 o en una extensión oficial?
2. ¿Puede lograrse con una convención de recursos/herramientas existente?
3. ¿Es responsabilidad del protocolo o de un gestor/supervisor del sistema operativo?
4. ¿Qué requisito de interoperabilidad o seguridad demuestra que falta?
5. ¿Cuál es la prueba reproducible y el costo de mantener la extensión?

**Regla de investigación:** priorizar **MCP como mecanismo de comunicación**, **SASP como integración con aplicaciones/servicios del SO**, y **extensiones SASP solo para capacidades verificablemente ausentes**.

## 8. Programa de validación

Ver [plan de seis experimentos de integración con el SO](../experiments/os-integration/README.md). El actual [servidor Rust MCP de prueba](../experiments/mcp-rust-stdio/README.md) sirve como **línea base de Tools/Resources**; no implementa registro local, descubrimiento de instancias, gestión de servicios ni políticas reales.

## 9. Fuentes primarias

- [MCP 2026-07-28 — changelog](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/changelog.mdx)
- [MCP — Tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
- [MCP — Resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
- [MCP — Subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)
- [MCP — Transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- [MCP — Authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- [MCP Tasks](https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md)
- [MCP Apps](https://github.com/modelcontextprotocol/ext-apps/blob/main/specification/2026-01-26/apps.mdx)
- [MCP Registry](https://github.com/modelcontextprotocol/registry/blob/main/docs/modelcontextprotocol-io/about.mdx)
- [Arquitectura SASP](architecture.md), [Terminología](terminologia.md), [Registry y Discovery](registry-discovery.md) y [contrato auxiliar experimental](../specification/draft/overview.md)
