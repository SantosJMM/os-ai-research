# SASP y MCP — Análisis de interoperabilidad

> **Estado:** investigación técnica y recomendación preliminar, **no decisión normativa de SASP**.  
> **Referencia de comparación:** Model Context Protocol, revisión **2026-07-28**, y extensión **MCP Apps 2026-01-26**.  
> **Casos de estudio:** [Music Player](../examples/music-player/manifest.json) y [Backup Daemon](../examples/backup-daemon/manifest.json).

## 1. Pregunta de investigación

**¿Qué parte del contrato semántico de aplicaciones y servicios propuesto por SASP puede expresarse directamente en MCP, y qué convenciones adicionales, si las hubiera, justifican una especificación independiente?**

La comparación parte de dos necesidades diferentes:

- **MCP** define mecanismos de comunicación con servidores que ofrecen herramientas y recursos a clientes de IA, además de extensiones opcionales.
- **SASP** propone una descripción común de **aplicaciones y servicios como entidades**: identidad lógica, instancias, estados, métodos, eventos y contexto de interacción, sin exigir interfaz gráfica ni un sistema operativo concreto.

La posible diferencia es **semántica y de convenciones**, no una carencia general de MCP para exponer métodos, datos o notificaciones.

## 2. Hechos relevantes de MCP 2026-07-28

La revisión 2026-07-28 introdujo un núcleo **sin handshake ni sesión de protocolo obligatoria**: cada solicitud declara su versión y capacidades en `_meta`. El método `server/discover` permite conocer capacidades, pero no es una operación obligatoria para iniciar cada interacción. Los clientes antiguos pueden seguir empleando `initialize` cuando negocian revisiones previas.

Las capacidades principales para la investigación SASP son:

- **Tools:** `tools/list` y `tools/call`, con `inputSchema`, `outputSchema`, `structuredContent`, anotaciones y listas que pueden variar según la autorización de la solicitud.
- **Resources:** `resources/list` y `resources/read`, con URIs y contenidos dinámicos que pueden expresar manifiestos o instantáneas.
- **Notifications:** `subscriptions/listen` permite observar cambios de catálogos y actualizaciones de recursos, como `notifications/resources/updated`. **No equivale a un bus genérico de eventos de dominio con entrega durable garantizada.**
- **Tasks:** la extensión **`io.modelcontextprotocol/tasks`** ofrece ejecución asíncrona asociada, actualmente, a `tools/call`, con `tasks/get`, `tasks/update` y `tasks/cancel`. No debe confundirse con el Task Runtime general de un sistema operativo.
- **MCP Apps:** la extensión **`io.modelcontextprotocol/ui`** asocia herramientas a recursos `ui://` y presenta UI HTML interactiva dentro de hosts compatibles. No es obligatoria para el servidor MCP ni para un proveedor SASP.
- **Seguridad:** MCP contempla autorización y consentimiento, pero el proveedor conserva la obligación de validar permisos y efectos. Un campo informativo o una anotación de herramienta no otorgan autoridad.

La versión 2026-07-28 modifica decisiones previas de MCP; los ejemplos para versiones 2025 basados en `initialize`, `resources/subscribe` o notificaciones no solicitadas **no deben reutilizarse sin comprobar su compatibilidad**.

## 3. Matriz de equivalencias

| Dimensión SASP | Mecanismo existente en MCP | Evaluación | Qué podría requerir SASP |
| --- | --- | --- | --- |
| `provider.id`, nombre, versión | Información de servidor y recursos propios; metadatos de respuesta | **Se puede expresar** | Semántica de identificador estable, no necesariamente igual a un MCP Server |
| `provider.kind` (`desktop`, `daemon`) | Dato de dominio en un recurso o metadato de extensión | **Convención adicional** | Vocabulario y interoperabilidad entre consumidores |
| Instancias `instance_id` | Handles explícitos en herramientas y URIs de recursos | **Se puede modelar** | Identidad, alcance, caducidad y revocación por instancia |
| Catálogo de métodos | `tools/list`, esquemas de entrada/salida | **Cubierto por MCP** | Convención para asociar herramienta con proveedor e instancia |
| Invocación y resultados | `tools/call`, `structuredContent` | **Cubierto por MCP** | Mapeo de errores/efectos del dominio; sin alterar permisos |
| Esquema de estado | JSON Schema incluido en un recurso de manifiesto | **Se puede expresar** | Definir una convención de ubicación, tipos y evolución |
| Snapshot dinámico | `resources/read` o herramienta de consulta | **Cubierto como dato** | Semántica de revisiones, consistencia y caché |
| Cambio de estado | `subscriptions/listen` + recurso actualizado | **Parcial** | Definir si basta reconsultar snapshot y cómo detectar lagunas |
| Eventos de dominio tipados | Recursos de eventos, herramientas o extensión específica | **Convención o extensión** | Suscripción, payload, orden, retención y replay si se necesitan |
| Descubrir servidores | Configuración de host, mecanismos MCP y Registry público | **Existe descubrimiento** | No confundir catálogo de servidores con registro local de procesos e instancias |
| Vida del proceso / endpoint | Gestión externa al protocolo MCP | **No es función central de MCP** | Modelo interoperable de disponibilidad, salud y reinicio |
| Trabajo asíncrono | Extensión MCP Tasks | **Cubierto para ciertos flujos** | Distinguir tareas de herramientas y Task Runtime general |
| Presentación interactiva | MCP Apps (`ui://`, `_meta.ui.resourceUri`) | **Extensión existente** | Mantenerla opcional; no exigir HTML a todos los proveedores |
| Autorización | Transporte MCP y políticas del servidor | **Mecanismos existentes** | Garantías del sistema anfitrión, no confianza en manifiestos |

**Resultado preliminar:** en los dos ejemplos actuales **no aparece una necesidad demostrada de reemplazar la comunicación MCP**. Las áreas por validar son principalmente las **convenciones de entidad/instancia**, la **semántica de estado revisable** y los **eventos de dominio y ciclo de vida**.

## 4. Proyección del contrato SASP sobre MCP

La primera alternativa a evaluar es un **perfil semántico SASP sobre un servidor MCP**. El perfil definiría nombres, relaciones y estructuras de datos en recursos, sin modificar los métodos principales de MCP.

```text
                CLIENTE CON MCP
                      |
        +-------------+--------------+
        | tools/list - tools/call    |
        | resources/list - read      |
        | subscriptions/listen       |
        +-------------+--------------+
                      |
                 SERVIDOR MCP
                      |
            Perfil semántico SASP
          manifiesto / estado / eventos
                      |
         Aplicación o daemon existente
                      |
              API nativa y permisos

     [MCP Apps: presentación opcional del host]
     [Registry local: investigación separada]
```

### 4.1. Manifiesto como recurso

Un servidor MCP podría publicar el manifiesto de Music Player como recurso con un URI de ejemplo como:

`https://example.org/sasp/providers/org.example.music/manifest`

El contenido que devuelve `resources/read` correspondería al JSON del [manifiesto SASP](../examples/music-player/manifest.json). La estructura JSON se mantiene independiente de si se obtiene por MCP, Unix socket o por un adaptador.

**El URI anterior es ilustrativo, no un recurso publicado ni un esquema de URI aprobado.** El objetivo es probar que el contenido puede transportarse con MCP sin diseñar antes otro protocolo RPC.

### 4.2. Estado como recurso independiente

Otra URI de recurso, por ejemplo:

`https://example.org/sasp/providers/org.example.music/instances/music-01/state`

podría devolver la [instantánea dinámica](../examples/music-player/state.snapshot.json). Para el daemon de backups se utilizaría el mismo patrón lógico, adaptado a `org.example.backup` y su `instance_id`.

El cliente puede reconsultar el recurso cuando reciba `notifications/resources/updated` a través de `subscriptions/listen`, **si el servidor anuncia y soporta esta capacidad**.

No basta con recibir la notificación para garantizar que se observó cada cambio intermedio.

### 4.3. Métodos como herramientas MCP

El método SASP `player.set_volume` ya utiliza una forma compatible con un catálogo de herramientas MCP:

```json
{
  "name": "player.set_volume",
  "description": "Set volume from 0 to 100",
  "inputSchema": {
    "type": "object",
    "properties": {
      "volume": { "type": "integer", "minimum": 0, "maximum": 100 }
    },
    "required": ["volume"],
    "additionalProperties": false
  },
  "outputSchema": {
    "type": "object",
    "properties": {
      "accepted": { "type": "boolean" }
    },
    "required": ["accepted"],
    "additionalProperties": false
  }
}
```

Es una **definición individual de herramienta**, tal como podría figurar dentro de `tools/list`, no una respuesta RPC completa. Para una invocación con MCP 2026-07-28 se utiliza `tools/call` con la versión y las capacidades de cliente en el `_meta` de la solicitud. El resultado completo usa `resultType: "complete"`; el valor validado puede ir en `structuredContent`.

El atributo SASP `effect: "act"` es una pista del dominio, no un permiso de ejecución ni un campo de herramienta MCP definido por esta comparación.

### 4.4. Asociar métodos a instancias

Los ejemplos SASP separan `provider.id` de `instance_id`. Una traducción MCP debe escoger entre:

- **Un servidor MCP por instancia**, manteniendo un catálogo de herramientas por conexión o URL.
- **Un servidor MCP para varias instancias**, con un parámetro tipado `instance_id` o identificador de destino incluido en las herramientas.
- **Un servidor intermediario**, que encaminase llamadas a los proveedores pertinentes con reglas de autorización explícitas.

Las tres opciones necesitan comparar identidad, descubrimiento, scopes, herramientas cacheables, eficiencia y comportamiento ante la caída de una instancia. **La especificación MCP no aporta automáticamente el registro local de procesos de un sistema operativo.**

### 4.5. Un daemon sin MCP Apps

El ejemplo [Backup Daemon](../examples/backup-daemon/manifest.json) no contiene `interaction`. Podría exponer `backup.verify` como herramienta y su estado como recurso **sin proporcionar HTML ni `ui://`**.

Esto es una condición importante para SASP: su utilidad no debe depender de tener una ventana o de poder renderizar una interfaz en un chat.

## 5. Cuestiones pendientes y limitaciones relevantes

### Identidad no equivale a autenticación

El `provider.id` declarado por una aplicación o el `serverInfo` de MCP son referencias semánticas; ninguno de ellos sustituye a credenciales de transporte o permisos efectivos. Se deben comprobar por separado identidades del proceso, usuario, instancia y consumidor.

### El estado requiere garantías explícitas

Aunque MCP transporte un JSON con `revision`, no establece automáticamente qué ocurre con las revisiones al reiniciar un proveedor, cómo detectar un cambio perdido ni cómo obtener consistencia entre recursos. SASP tendría que especificarlo **si resulta imprescindible para el caso de uso**.

### Notificación de recurso no es evento de dominio persistente

`notifications/resources/updated` indica que un recurso cambió; no garantiza que `volume.changed` o `backup.failed` se haya entregado como evento tipado con historial duradero. Debemos comprobar si basta una vista de estado, o si algún caso requiere un esquema de eventos y retención más fuerte.

### MCP Tasks no equivale al Task Runtime del SO

La extensión MCP Tasks maneja tareas asíncronas de solicitudes MCP, mientras que un Task Runtime de un sistema operativo puede gestionar trabajos entre aplicaciones, procesos y reinicios, con restricciones operativas independientes. Un adaptador puede reflejar tareas del SO como tareas MCP sin que ambos tengan la misma responsabilidad.

### Registro de servidores frente a registro local de instancias

El **MCP Registry** público distribuye información sobre servidores disponibles; no debe confundirse con un servicio del host que mantenga un inventario confiable y actualizado de procesos o proveedores ejecutándose. Si SASP necesitara esto último, deberá estudiar herramientas nativas del SO, registros locales y límites de confianza.

### Compatibilidad con múltiples revisiones de MCP

Un bridge deberá detectar si el cliente/servidor habla **2026-07-28** o una revisión anterior. Las reglas de `initialize`, sesiones y `resources/subscribe` cambiaron en 2026-07-28. El diseño no debe depender de una sola revisión sin documentar sus requisitos.

## 6. Recomendación técnica y criterio de decisión

### Recomendación inicial: perfil semántico SASP sobre MCP

**Primera vía de experimentación:** describir un proveedor SASP mediante recursos, herramientas y suscripciones de un servidor MCP 2026-07-28, aprovechando **MCP Apps solo para el caso de UI interactiva**.

La recomendación **no declara que SASP sea ya una extensión de MCP**, ni descarta la opción de sockets Unix directos o broker para implementaciones nativas.

### Experimentos antes de cerrar el diseño

| ID | Caso | Evidencia a obtener |
| --- | --- | --- |
| **MCP-01** | Publicar `manifest.json` y `state.snapshot.json` de ambos ejemplos con `resources/read` | Dos clientes consumen el mismo contrato |
| **MCP-02** | Exponer `player.set_volume` y `backup.verify` con `tools/list` + `tools/call` | Esquemas, resultados, errores y permisos verificados |
| **MCP-03** | Cambiar volumen, recibir `notifications/resources/updated` y reconsultar snapshot | Comportamiento de actualización y observación |
| **MCP-04** | Reiniciar instancia del daemon y conectar de nuevo | Diferencia entre instancia, proceso y endpoint; pérdida de eventos |
| **MCP-05** | Añadir MCP Apps únicamente al Music Player | UI opcional sin afectar a clientes semánticos o daemon |
| **MCP-06** | Contrastar mismo contrato con socket Unix o registro local | Costes, complejidad, aislamiento y requisitos no cubiertos |

**Condiciones para justificar una especificación adicional:** demostrar una necesidad verificable que las convenciones MCP no resuelvan de forma suficiente (por ejemplo, consistencia de estado, interoperabilidad entre instancias, descubrimiento confiable o semántica de eventos de dominio) y cuantificar el coste de mantenerla.

**Condiciones para descartar un transporte propio:** comprobar que un perfil SASP sobre MCP cubre los casos representativos con autorizaciones efectivas, rendimiento aceptable y complejidad razonable.

## 7. Fuentes primarias

1. [MCP — especificación 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28)
2. [MCP — Tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
3. [MCP — Resources y suscripciones](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
4. [MCP — Versioning and Compatibility](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning)
5. [MCP — Authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
6. [MCP Tasks — extensión 2026-07-28](https://tasks.extensions.modelcontextprotocol.io/specification/2026-07-28/tasks)
7. [MCP Apps — especificación de extensión](https://github.com/modelcontextprotocol/ext-apps/blob/main/specification/2026-01-26/apps.mdx)
8. [MCP Registry — repositorio público de servidores](https://registry.modelcontextprotocol.io/)

## 8. Implementación de referencia en Rust

La prueba de concepto [SASP sobre MCP en Rust](../experiments/mcp-rust-stdio/README.md) expone los manifiestos y snapshots de los ejemplos como recursos MCP y sus métodos como herramientas tipadas. La [referencia inicial de Thanos Music](../../tup/archivo/05-contrato-semantico-thanos-music.md) conserva el objetivo semántico original: estado, métodos, eventos y contexto opcional de interfaz.

La implementación inicial no incorpora aún notificaciones de actualización, suscripciones ni seguridad de producción. Las pruebas de integración y la compilación deben verificarse antes de considerarla un resultado experimental reproducible.

## 9. Alcance del resultado

Esta comparación identifica **mecanismos disponibles en estándares existentes** y **preguntas de diseño SASP**. No sustituye prototipos, pruebas de conformidad ni decisiones de implementación; el valor diferencial del perfil SASP permanece como hipótesis por validar.
