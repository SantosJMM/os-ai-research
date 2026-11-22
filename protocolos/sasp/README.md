# SASP — Semantic Application & Service Protocol

> **Nombre:** SASP (*Semantic Application & Service Protocol*; «Protocolo Semántico de Aplicaciones y Servicios»).  
> **Estado:** investigación y arquitectura en borrador, **sin especificación normativa v1.0**.  
> **Orientación:** **MCP primero; integración con el sistema operativo como objetivo principal**.

## Qué es SASP

**SASP estudia cómo un sistema operativo integra MCP y MCP Apps con el ciclo de vida de aplicaciones y servicios.** Busca que un software pueda **registrarse, ser descubierto, conectarse, ofrecer capacidades semánticas, publicar estado y eventos, y ser utilizado por agentes y personas** bajo los permisos reales del sistema.

**SASP no es un sustituto de MCP**, ni un transporte JSON-RPC nuevo por defecto. Se apoya en las herramientas, recursos, suscripciones y extensiones existentes de MCP. Un **contrato semántico de proveedor** (identidad, estado, métodos, eventos y contexto opcional de interfaz) es una parte de la arquitectura, **no su definición completa**.

Las aplicaciones y sus desarrolladores **declaran** sus capacidades. Los agentes y otros clientes pueden descubrirlas y solicitar operaciones, pero **no inventan capacidades ni reciben permisos por conocerlas**.

El nombre proviene de **TUP — Thanos UI Protocol**, cuya motivación era superar la dependencia de ventanas, capturas y coordenadas. SASP amplió esa idea a **aplicaciones gráficas, programas sin interfaz y daemons**, sin requerir Thanos OS para adoptar el modelo.

Las decisiones de alcance están en [DECISION-0001 (nombre)](DECISION-0001-nombre-y-alcance.md) y [DECISION-0002 (integración MCP con el SO)](DECISION-0002-integracion-mcp-en-so.md).

## Qué reutilizamos y qué estudia SASP

| Área | Tecnología existente | Responsabilidad de investigación SASP |
| --- | --- | --- |
| Métodos y herramientas | MCP `tools/list` y `tools/call` | Relacionar capacidades con aplicaciones e instancias del SO |
| Datos y estado | MCP `resources/list` y `resources/read` | Exponer estado coherente de una instancia; semántica de revisiones si hace falta |
| Notificaciones | MCP `subscriptions/listen` y recursos actualizados | Comprobar si bastan para eventos de dominio, recuperación e historial |
| Descubrimiento de un endpoint | MCP `server/discover` | Encontrar primero **qué aplicaciones y servidores existen en el host** |
| Publicación de servidores | MCP Registry y `server.json` | Mantener catálogo **local** y directorio **vivo** de instancias verificadas |
| Interfaz humana | MCP Apps y UI propia de cada aplicación | Integrar al **Presentador**, la UI semántica y la GUI nativa sin obligar a usar HTML |
| Trabajo asíncrono | Extensión MCP Tasks | Integración con procesos y tareas del SO; evitar duplicar un Task Runtime |
| Conexión y políticas | Transportes MCP y seguridad del SO | Elegir rutas autorizadas; comparar `stdio`, Unix, HTTP y mediación |

**MCP Registry no es SASP Registry.** El primero distribuye metadatos de servidores; el segundo, como componente de integración local, necesita conocer instalaciones e instancias activas verificadas. **`server/discover` tampoco busca por sí solo todo el software instalado en Linux**: describe las capacidades de un servidor MCP ya localizado.

## Arquitectura conceptual

```text
                 Gestor de paquetes / SO
                    instala proveedores
                           |
                     SASP Registry
             catálogo + instancias verificadas
                           |
                      SASP Discovery
                 interfaz MCP candidata
                           |
             búsqueda por identidad/capacidad
                           |
              +------------+-------------+
              |                          |
       Agente                     Presentador / UI
              |                          |
              +------- SASP Connection --+
                        |  MCP
               stdio / Unix / HTTP
                  o ruta mediada
                        |
                 App o daemon
             MCP Tools / Resources
             Notifications / Tasks
             UI opcional vía MCP Apps
                        |
          Autorización y operaciones nativas
```

El diagrama representa **roles lógicos** y no exige implementar un daemon independiente para cada uno. Los proveedores conservan sus APIs, procesos y mecanismos de seguridad nativos.

Los nombres generales para estas responsabilidades son **Supervisor**, **Autoridad de seguridad**, **Ejecutor privilegiado**, **Subsistema de auditoría**, **Agente** y **Presentador**. En **Thanos OS**, su implementación de referencia incluye **Station**, **Ship**, **Atlas**, el componente de agente **Capsuleer** y **Thanos Shell**, respectivamente. Estas correspondencias no son necesariamente uno a uno ni obligatorias para otras plataformas. Véase [Terminología](docs/terminologia.md).

## Los seis ejes de SASP

1. **Application & Service Contract:** identidad y capacidades anunciadas, aprovechando Tools, Resources, JSON Schema y anotaciones MCP existentes.
2. **SASP Registry:** instalación y desinstalación, metadatos verificables, inventario local, instancias activas y caducidad de registros.
3. **SASP Discovery:** servicio de búsqueda por identidad o capacidad, potencialmente un **servidor MCP del SO**, que devuelve solo proveedores accesibles.
4. **SASP Connection:** resolución de destinos, identidad del cliente, compatibilidad MCP, `stdio`, sockets Unix, HTTP y mediación cuando sea necesaria.
5. **SASP Lifecycle & Events:** instalación, procesos, endpoints, tareas, recursos actualizados, suscripciones, fallos y recuperación.
6. **MCP Integration & Presentation:** clientes de IA y humanos, MCP Apps como opción y convivencia con UI nativa o ausencia de GUI.

**Criterio de diseño:** primero comprobar MCP y sus extensiones; después integrar esos mecanismos con el SO; **solo definir nuevas convenciones o extensiones SASP si los experimentos demuestran una necesidad no cubierta**.

## Documentación

### Dirección y decisiones

- **[SASP integrado con MCP en el SO](docs/os-integration.md):** misión, seis ejes y modelo operativo de instalación, registro, descubrimiento, conexión y ciclo de vida.
- **[Arquitectura](docs/architecture.md):** componentes, relaciones y fronteras de confianza.
- **[Terminología de SASP](docs/terminologia.md):** nombres genéricos de roles, diferencias con MCP y correspondencias de Thanos OS.
- **[Registro y descubrimiento locales](docs/registry-discovery.md):** fuentes de identidad y datos, separación entre proveedor instalado e instancia en ejecución, reconciliación y consulta autorizada.
- [Conceptos y vocabulario](docs/concepts.md).
- [Decisión de alcance MCP-first](DECISION-0002-integracion-mcp-en-so.md).

### Análisis y especificación en borrador

- [Análisis de interoperabilidad MCP](docs/interoperability-mcp.md): capacidades ya existentes y preguntas pendientes.
- [Contrato de proveedor experimental](specification/draft/overview.md): **formato de datos de prueba**, no protocolo alternativo.
- [Music Player — manifiesto](examples/music-player/manifest.json) y [estado](examples/music-player/state.snapshot.json).
- [Backup Daemon — manifiesto](examples/backup-daemon/manifest.json) y [estado](examples/backup-daemon/state.snapshot.json).

### Experimentos

- **[Plan OS-01 a OS-06](experiments/os-integration/README.md):** investigaciones de los seis ejes con evidencia y criterios de aceptación.
- **[Discovery como servicio MCP](experiments/os-integration/discovery-mcp.md):** herramientas candidatas de búsqueda y resolución.
- **[Ejemplos Registry y Discovery](examples/os-integration/README.md):** catálogo local e instancia activa frente a resultado de búsqueda autorizado.
- [Servidor de ejemplo MCP en Rust](experiments/mcp-rust-stdio/README.md): **línea base Tools/Resources**, todavía sin validación ejecutable ni Registry/Discovery.

### Antecedentes

- [De TUP a SASP](../tup/archivo/04-origen-hallazgos-evolucion.md).
- [Contrato de referencia Thanos Music](../tup/archivo/05-contrato-semantico-thanos-music.md).

## Prioridad experimental

La primera iteración debe **verificar el servidor MCP Rust existente (OS-01)** y desarrollar **Registry local (OS-02) + Discovery expuesto por MCP (OS-03)**. Solo entonces comparar rutas de conexión (OS-04), observar eventos y recuperación (OS-05) y estudiar MCP Apps/UI nativa (OS-06).

Ningún experimento se considera completado sin ejecución reproducible y resultados publicados. Las APIs de descubrimiento, los esquemas de manifiesto, el broker y las extensiones de UI o eventos siguen siendo propuestas.

## Fuera de alcance

- Sustituir MCP, MCP Apps, Linux, syscalls, señales, permisos o protocolos de negocio existentes.
- Exigir una GUI, HTML, un broker obligatorio o un transporte SASP separado.
- Convertir la visibilidad de una herramienta en autorización de invocación.
- Presentar un manifiesto o ejemplo JSON como una implementación completa del sistema de registro y descubrimiento.

**Seguimiento:** [Issue #13 — SASP](https://github.com/SantosJMM/os-ai-research/issues/13) · [Roadmap general](../../roadmap.md).  
**Fuentes principales:** [MCP](https://modelcontextprotocol.io/specification/2026-07-28) · [MCP Apps](https://github.com/modelcontextprotocol/ext-apps) · [MCP Registry](https://github.com/modelcontextprotocol/registry).
