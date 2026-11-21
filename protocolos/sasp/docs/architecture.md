# Arquitectura de SASP

> **Estado:** propuesta arquitectónica para investigación. Este documento describe componentes, responsabilidades y flujos candidatos; **no es una especificación normativa**, ni establece un formato de mensajes definitivo.

## Principio rector: contrato declarado por el proveedor

**La aplicación o servicio declara las capacidades; el agente las descubre y puede solicitar operaciones, pero no las inventa ni obtiene permisos por conocerlas.** El modelo propuesto abarca **estado, métodos, eventos y contexto opcional de interfaz**, además de la identidad del proveedor y su instancia. Este principio se desarrolla en el [ejemplo de referencia Thanos Music](../../tup/archivo/05-contrato-semantico-thanos-music.md).

## 1. Propósito y alcance

**SASP — Semantic Application & Service Protocol** estudia **cómo integrar aplicaciones y servicios en un sistema operativo reutilizando MCP y MCP Apps**. El contrato de identidad, estado, capacidades, métodos, eventos y contexto de interacción es solo una parte: también se investigan el registro de instalaciones e instancias, el descubrimiento, la conexión, la disponibilidad y la recuperación. Los consumidores pueden ser agentes de IA, interfaces humanas u otros programas.

El contrato debe poder aplicarse a una aplicación gráfica, un servicio sin interfaz, un worker o un daemon. No exige que la aplicación esté visible, que implemente un toolkit concreto, que use Thanos OS ni que se comunique por un transporte particular.

La cuestión principal de arquitectura es **cómo separar lo que el software significa y permite hacer de los mecanismos usados para encontrarlo, representarlo y ejecutar operaciones**.

### Alcance de los servicios de integración

La dirección actual se desarrolla en **[SASP: integración MCP con el sistema operativo](os-integration.md)**. Distingue **Registry** (catálogo instalado y directorio de instancias), **Discovery** (consulta por identidad/capacidad como servicio MCP candidato), **Connection** (stdio, socket Unix, HTTP o mediación), **Lifecycle & Events** (supervisión, recursos y suscripciones) y **Presentation** (MCP Apps opcional o GUI nativa).

Los componentes son **responsabilidades lógicas**; no se exige implementar cada uno en un daemon separado. El actual prototipo MCP en Rust cubre solo herramientas y recursos, no el registro ni el descubrimiento local.

## 2. Modelo conceptual

```text
                CONSUMIDORES
     +----------------+----------------+
     | Agente / CLI   | GUI / Shell    |
     +----------------+----------------+
             |                  |
             +---- Cliente SASP-+
                        |
         descubrimiento / conexión / sesión
                        |
              [adaptador opcional]
                        |
                Contrato SASP
       identidad | capacidades | estado
          métodos | eventos | contexto
                        |
                Proveedor SASP
               (app / servicio)
                        |
         API y lógica nativa de la aplicación
                        |
       SO / kernel / servicios / recursos

  Registro o broker: componente opcional de descubrimiento
  y/o intermediación; no confiere autorización por sí mismo.
  Autoridad de seguridad: externa a la descripción semántica;
  valida clientes y operaciones donde corresponda.
```

**SASP describe y coordina interacciones; no sustituye el sistema operativo.** Las operaciones nativas conservan sus mecanismos de ejecución, supervisión, aislamiento y control de acceso.

## 3. Roles y responsabilidades

| Rol | Responsabilidad | Qué no debe asumir |
| --- | --- | --- |
| **Proveedor SASP (Provider)** | Expone identidad, capacidades, estado observable, eventos y puntos de operación asociados a una aplicación o servicio. | Que el cliente posee permisos por haber descubierto la operación. |
| **Cliente SASP (Client)** | Descubre proveedores, interpreta contratos, consulta estados y solicita operaciones. | Privilegios directos sobre el proveedor o el sistema. |
| **Registro (Registry, opcional)** | Localiza proveedores/instancias y su información de disponibilidad; permite distinguir instalado de activo. | Ejecutar acciones autorizadas solo por estar registradas. |
| **Broker (opcional)** | Media conexiones y, si se decide, encaminamiento, suscripciones y reconexión. | Ser obligatorio para todas las topologías. |
| **Adaptador / Bridge (opcional)** | Traduce capacidades hacia protocolos o clientes externos, por ejemplo MCP. | Omitir la identidad, los controles de acceso o las garantías del proveedor. |
| **Presentador (Shell / UI)** | Construye una experiencia humana a partir de información semántica, o utiliza una UI propia. | Que todos los proveedores tengan vistas gráficas. |
| **Autoridad de seguridad** | Autentica identidades, decide permisos, restringe efectos y registra operaciones relevantes según el sistema anfitrión. | Que un manifiesto o prompt sustituya las comprobaciones efectivas. |

Un mismo proceso puede implementar varios roles. **Los roles son separaciones lógicas**, no una obligación de desplegar un daemon por cada uno.

## 4. Modelo de información del proveedor

Se investigan seis dimensiones, deliberadamente independientes del transporte:

1. **Identidad:** identificador del producto/proveedor, versión del contrato, identificador de instancia, clase de software y metadatos necesarios para descubrimiento.
2. **Capacidades:** catálogo de funciones y restricciones declaradas por el proveedor; debe permitir diferenciar consulta de acción.
3. **Estado:** instantánea observable con tipos definidos y, cuando corresponda, una revisión o mecanismo equivalente para detectar obsolescencia.
4. **Métodos:** operaciones con esquemas de entrada/salida, errores documentados y consideraciones sobre sus efectos.
5. **Eventos:** cambios observables con semántica de suscripción, secuencia, entrega y recuperación por especificar.
6. **Contexto de interacción (opcional):** información que ayuda a presentar y comprender funciones del software sin forzar HTML, una vista declarativa ni una UI gráfica.

### Separación entre descripción estática y estado dinámico

Una decisión que debe evaluarse es **mantener el manifiesto de capacidades relativamente estable** y obtener el estado mediante consultas o suscripciones. Incluir el estado actual en una sola respuesta de descubrimiento simplifica casos pequeños, pero complica coherencia, cacheo y actualización de datos.

El protocolo deberá precisar si la entidad es la **aplicación instalada**, el **servicio lógico** o una **instancia en ejecución**. Esas identidades no son equivalentes.

### Ejemplo mínimo no normativo

```json
{
  "sasp_version": "draft",
  "provider": {
    "id": "example.backup",
    "instance_id": "backup-01",
    "kind": "daemon"
  },
  "capabilities": {
    "methods": [
      {
        "name": "backup.verify",
        "description": "Verify an existing backup"
      }
    ],
    "events": ["backup.completed", "backup.failed"]
  },
  "state": {
    "status": "idle",
    "revision": 12
  }
}
```

Este ejemplo ilustra la organización conceptual, **no define nombres normativos de campos, esquemas de métodos, garantías de evento ni una versión implementable**.

## 5. Flujo candidato de interacción

El siguiente flujo representa **operaciones lógicas**, no nombres de RPC aprobados.

```text
 Cliente                 Registro opcional          Proveedor
    |                            |                      |
    |-- descubrir proveedores -->|                      |
    |<-- ubicación / estado -----|                      |
    |                                                   |
    |------------- establecer sesión ------------------>|
    |<------------ versión / capacidades ---------------|
    |------------- consultar manifiesto --------------->|
    |<------------ identidad / métodos -----------------|
    |------------- consultar estado ------------------->|
    |<------------ snapshot / revisión -----------------|
    |------------- suscribirse a eventos -------------->|
    |<------------ actualización -----------------------|
    |------------- solicitar acción ------------------->|
    |            [validación y autorización]            |
    |<------------ resultado o rechazo -----------------|
    |                                                   |
    |       [desconexión, nuevo descubrimiento,          |
    |        resincronización según garantías]          |
```

### Descubrimiento

Debe ser posible identificar un proveedor disponible, su instancia y un punto de conexión o ruta lógica. Se evaluarán estrategias de **socket Unix directo**, **registro de proveedores** y **broker**. Descubrimiento no es autorización: los metadatos publicados por un proveedor no son prueba suficiente de identidad.

### Negociación y consulta

Antes de interpretar un contrato, el cliente y el proveedor necesitan acordar **versión y capacidades compatibles**. Queda pendiente decidir la representación de esa negociación y los errores de incompatibilidad.

### Observación

Un cliente debe poder conocer un estado inicial y observar cambios sin asumir que los eventos llegan exactamente una vez o sobreviven a reinicios. Las garantías deben declararse de forma explícita: instantáneas, secuencias, retención, deduplicación y recuperación se estudiarán por separado.

### Invocación

Una acción debe identificar la operación y su entrada tipada. El proveedor o la autoridad correspondiente valida la identidad efectiva del cliente, las restricciones de la operación y el estado pertinente antes de ejecutarla. Una respuesta debe diferenciar, cuando corresponda, **aceptación de trabajo**, **resultado**, **rechazo** y **fallo**.

## 6. Ciclos de vida distintos

SASP debe distinguir:

- **Ciclo de vida del proceso o servicio:** arranque, ejecución, detención, error o reinicio administrados por mecanismos nativos.
- **Ciclo de vida del endpoint SASP:** inicialización, disponibilidad, desconexión, degradación y restablecimiento.
- **Ciclo de vida de una tarea:** pendiente, en curso, completada, fallida, cancelada o interrumpida, administrado por el proveedor o un Task Runtime según el caso.

Estos estados no deben confundirse. Un daemon puede seguir funcionando aunque pierda su conexión semántica; una tarea puede continuar después de cerrar la interfaz humana.

Los nombres y transiciones definitivos pertenecen a la futura especificación, no a este documento de arquitectura.

## 7. Seguridad y fronteras de confianza

La regla principal es **«capacidad anunciada» no equivale a «capacidad autorizada»**.

- La identidad confiable proviene de mecanismos de autenticación y de la conexión: por ejemplo, credenciales de pares en sockets Unix o un mecanismo verificable adecuado al transporte. **No** basta un `client_id` incluido en JSON.
- El catálogo de operaciones es **información**, no un sistema de permisos. Cada operación protegida requiere comprobaciones efectivas.
- Un registro o broker introduce nuevas fronteras de confianza; debe evaluarse suplantación de proveedores, acceso entre usuarios, pérdida de servicio y revocación.
- Los adaptadores no pueden convertir permisos de lectura en acciones con más privilegios ni abrir rutas laterales que eludan la autoridad del sistema.
- Los recursos, entradas y resultados se tratan como datos no confiables donde corresponda; los agentes no deben interpretar descripciones de proveedores como instrucciones con autoridad superior.
- Las operaciones de larga duración requieren identidad de tarea, cancelación controlada, límites de recursos y auditoría según sus efectos.
- El protocolo no aporta por sí mismo sandbox, aislamiento, control de procesos ni privilegios.

### Ejemplo de integración en Thanos OS

En un despliegue hipotético, **Capsuleer** consumiría contratos SASP mediante una ruta permitida y solicitaría operaciones; **Thanos Shell** presentaría información a personas; **Ship** conservaría la aplicación de políticas para operaciones protegidas y **Atlas** registraría evidencia relevante. **Station** seguiría supervisando servicios y procesos. Ninguno de esos componentes es requisito para otras implementaciones de SASP.

## 8. Transporte e interoperabilidad

**La semántica se define antes de elegir su canal.** Se consideran al menos tres rutas de experimentación:

| Ruta | Ventajas candidatas | Costes o riesgos por medir |
| --- | --- | --- |
| **Conexión directa por socket Unix** | Menos intermediación, límites de proceso claros en un mismo host | Descubrimiento, permisos, número de conexiones, reconexión |
| **Registro + broker** | Punto común de localización y mediación, posibilidad de concentrar suscripciones | Centralización, disponibilidad, superficie de ataque y complejidad |
| **Perfil o puente sobre MCP** | Aprovecha herramientas, recursos y ecosistema de clientes existentes | Correspondencia semántica, identidad de instancia, ciclo de vida, límites de compatibilidad |

El uso de **MCP Apps** se estudia específicamente como **representación interactiva opcional en hosts compatibles**, no como el transporte obligatorio de SASP ni como sustituto de interfaces desktop o daemons.

Se debe comparar un caso real implementado con **MCP existente** frente a cualquier propuesta de SASP, para distinguir necesidades genuinas de duplicación de mecanismos ya disponibles.

## 9. Qué pertenece a SASP y qué pertenece a otros componentes

| Responsabilidad | Límite de SASP |
| --- | --- |
| Exponer identidad, estado y capacidades | Dentro de la investigación del contrato |
| Describir métodos/eventos y sus esquemas | Dentro de la investigación del contrato |
| Dibujar ventanas o páginas HTML | Fuera del núcleo; integración opcional |
| Ejecutar trabajos y conservar su ciclo de vida | Del proveedor o Task Runtime; SASP podría exponer su estado |
| Terminar procesos, gestionar señales y cgroups | Del sistema operativo y sus interfaces nativas |
| Autorizar efectos privilegiados | De la autoridad de seguridad correspondiente |
| Decidir qué aplicación está instalada o desplegada | Del gestor de aplicaciones; SASP puede describir instancias disponibles |
| Interpretar conversaciones y planificar acciones | Del agente/cliente, no del contrato |

## 10. Contratos pendientes de especificación

Para avanzar hacia `specification/draft/`, se requieren decisiones y pruebas sobre:

1. **Vocabulario:** `provider`, `application`, `service`, `instance`, `capability`, `method`, `event`, `view/context`.
2. **Descubrimiento e identidad:** instalación, registro, instancias efímeras, autenticación y revocación.
3. **Tipos de estado:** snapshots, revisiones, coherencia y observación sin pérdida silenciosa de contexto.
4. **Métodos:** entrada/salida tipadas, efectos, errores, idempotencia y cancelación.
5. **Eventos:** suscripción, orden, secuencias, retención y resincronización.
6. **Versionado:** negociación, compatibilidad hacia atrás y extensiones.
7. **Transporte:** framing, límites, timeouts, backpressure y ciclo de conexión, si se crea un transporte propio.
8. **Seguridad:** amenazas, autorización y auditoría verificables.
9. **Conformidad:** casos de prueba independientes de la implementación.
10. **Interoperabilidad:** evaluación de compatibilidad con MCP y MCP Apps.

La primera validación recomendada es una **aplicación de música simulada** y un **servicio de backup sin GUI**, consumidos por dos clientes. Solo después conviene fijar un esquema normativo.

## 11. Documentos relacionados

- [Conceptos y vocabulario SASP](concepts.md)
- [Comparación e interoperabilidad con MCP](interoperability-mcp.md)
- [Contrato mínimo experimental — draft](../specification/draft/overview.md)
- [Ejemplo: manifiesto de Music Player](../examples/music-player/manifest.json) y [su estado](../examples/music-player/state.snapshot.json)
- [Ejemplo: manifiesto de Backup Daemon](../examples/backup-daemon/manifest.json) y [su estado](../examples/backup-daemon/state.snapshot.json)

- [SASP — descripción y principios](../README.md)
- [Decisión sobre nombre y alcance](../DECISION-0001-nombre-y-alcance.md)
- [Antecedentes de TUP y evolución a SASP](../../tup/archivo/04-origen-hallazgos-evolucion.md)
- [MCP — especificación oficial](https://modelcontextprotocol.io/specification)
- [MCP Apps — extensión y ejemplos](https://github.com/modelcontextprotocol/ext-apps)

**Objetivo de esta arquitectura:** conservar un núcleo semántico útil para aplicaciones y servicios, con representación opcional, seguridad verificable y posibilidad de interoperar con protocolos existentes sin comprometer prematuramente el diseño.
