# De TUP a SASP: origen, hallazgos y evolución del protocolo

> **Alcance:** historia técnica y evolución de la propuesta. TUP es la denominación original; SASP es la denominación vigente. Los esquemas y mecanismos pendientes se mantienen como propuestas de investigación.

## 1. ¿Por qué nació TUP?

La idea inicial surgió de una pregunta de arquitectura de **Thanos OS**: **¿cómo puede un agente de IA comprender y utilizar aplicaciones de un sistema operativo sin limitarse a mirar píxeles, interpretar ventanas o ejecutar clics en coordenadas?**

Una aplicación tradicional proporciona una interfaz gráfica para personas y, a veces, una API para otros programas. Esas dos interfaces no necesariamente comparten un modelo de interacción comprensible para un agente. Cuando no existe una integración específica, la automatización puede depender de capturas de pantalla, posiciones, nombres visibles o secuencias frágiles de acciones.

TUP (*Thanos UI Protocol*) nació como la investigación de una alternativa: **permitir que una aplicación publique un contrato explícito sobre sí misma** para que el agente pueda identificarla, conocer su estado, descubrir operaciones y observar cambios. La misma información podría alimentar una interfaz humana en **Thanos Shell**.

El objetivo no era prohibir interfaces gráficas ni reemplazar las API existentes, sino crear una forma estructurada de describir la interacción con software dentro de un entorno donde la IA es un participante de primera clase.

### Las primeras piezas del modelo

La propuesta temprana se organizó alrededor de tres conceptos:

| Concepto | Propósito original |
| --- | --- |
| **App** | Identidad de aplicación, capacidades, instancias, acciones y estado. |
| **View** | Vista declarativa, interpretable por Thanos Shell y por clientes semánticos. |
| **Task** | Trabajo identificable con estado, progreso, cancelación y resultado. |

Se estudiaron manifiestos (`AppManifest`), nodos de interfaz (`UiNode`), acciones tipadas (`ActionRequest`) y eventos como `view.publish`, `task.progress` y `task.completed`.

La intención se aprecia en el escenario **abrir proyecto → explorar archivos → ejecutar pruebas**: el agente no debería necesitar inferir dónde se encuentra un botón de «Ejecutar pruebas» para solicitar una operación que una aplicación expone explícitamente. La solicitud, su autorización, el trabajo y la presentación del resultado son responsabilidades diferentes.

Para los componentes y ejemplos de mensajes, consultar [arquitectura inicial](01-propuesta-v01.md), [JSON/JSON-RPC](02-ejemplos-json.md) y [acciones y Task Runtime](03-acciones-y-runtime.md).

## 2. Qué reveló el análisis arquitectónico

### 2.1. «UI» era una descripción demasiado estrecha

El contrato propuesto —identidad, estado, métodos, eventos y capacidades— también resulta útil cuando **no hay ninguna ventana**.

Un reproductor gráfico podría publicar `playback`, `volume` y `play`; un servicio de copias de seguridad podría publicar `last_backup`, `health` y `verify_backup`. En ambos casos el cliente necesita saber **qué es el proveedor, qué ocurre y qué operaciones ofrece**, no necesariamente cómo dibujarlo.

De aquí surge una distinción fundamental:

- **Contrato semántico:** lo que la aplicación o servicio declara sobre sus capacidades y su estado.
- **Representación:** cómo una interfaz humana —gráfica, textual o conversacional— presenta ese contrato.

La representación es una opción de consumo; no debe convertirse en requisito para implementar el contrato.

### 2.2. MCP cubre parte del problema

**Model Context Protocol (MCP)** ya ofrece mecanismos para que clientes descubran y utilicen herramientas y recursos de un servidor. También contempla notificaciones y formas de observación. Por tanto, **no es correcto asumir que un protocolo nuevo es necesario solo porque queremos métodos, estado o eventos para agentes**.

El trabajo pendiente es determinar si hace falta especificar una convención adicional y consistente para representar una **aplicación o servicio como entidad**: identidad de instancia, estado observable, capacidades relacionadas, ciclo de vida y contexto de interacción. Esta necesidad debe probarse contra MCP real, no declararse como una limitación insalvable de MCP.

### 2.3. MCP Apps muestra una opción de presentación, no una obligación de usar HTML

**MCP Apps** permite que un servidor MCP proporcione recursos de interfaz interactiva que un host compatible puede mostrar, típicamente mediante HTML/JavaScript en un entorno aislado. Es una forma útil de presentar funcionalidades de servicios o aplicaciones dentro de clientes como chats o entornos de desarrollo compatibles.

El hecho de que una app exponga una interfaz a través de MCP Apps **no transforma automáticamente una aplicación GTK/Qt o un daemon en una aplicación web nativa**, ni sustituye su protocolo de negocio.

Esta separación abre una posibilidad de interoperabilidad: **una aplicación que exponga un contrato semántico podría ser consumida de forma nativa por Thanos Shell y, mediante un adaptador, representada también en un host MCP Apps**. Es una arquitectura candidata, no una integración ya verificada.

### 2.4. El transporte es otro problema

Se examinaron **sockets Unix directos** y un **registro/broker** para descubrir aplicaciones, gestionar conexiones y reflejar si una instancia está disponible. Estas estrategias presentan compromisos de latencia, complejidad, aislamiento y escalabilidad, pero **no deberían determinar por sí solas el significado de un estado o método**.

También es importante separar el estado del **proceso/servicio** del estado del **endpoint de interacción**. Una aplicación puede estar ejecutándose y tener temporalmente desconectado su endpoint semántico.

### 2.5. Publicar una acción no concede autoridad

Una descripción de capacidades permite al agente **solicitar** operaciones, no obtener privilegios.

En Thanos OS, **Capsuleer** permanece aislado; **Ship** aplica políticas a operaciones protegidas y **Atlas** registra evidencia auditable. **Station** y el kernel conservan sus responsabilidades sobre procesos, señales y supervisión.

El protocolo no sustituye `kill`, syscalls, cgroups, sockets existentes, ni los protocolos especializados de servicios como PostgreSQL. Tampoco un manifiesto JSON puede sustituir la comprobación de identidad, permisos, validación de argumentos y control de efectos.

### 2.6. Task Runtime debe poder evolucionar por separado

Las tareas de larga duración requieren ciclo de vida, estados terminales, progreso, cancelación, persistencia y recuperación. Es razonable que su gestión corresponda a un **Task Runtime**, y que el protocolo permita consultar y presentar la información correspondiente.

Esta separación evita convertir el contrato de interacción con aplicaciones en un planificador general de procesos o en un ejecutor privilegiado.

## 3. ¿En qué se transformó TUP?

La evolución condujo a adoptar el nombre **SASP — Semantic Application & Service Protocol** (*Protocolo Semántico de Aplicaciones y Servicios*).

El cambio responde a dos ampliaciones de alcance:

1. **De interfaces a software:** no solo vistas gráficas, sino aplicaciones y daemons con capacidades semánticas propias.
2. **De Thanos OS a un contrato independiente:** no solo Capsuleer y Thanos Shell; cualquier cliente o plataforma podría estudiar o implementar el contrato.

La **identidad pública del protocolo** cambió, pero se mantuvo su intuición principal: **hacer que el software pueda describirse de forma estructurada para ser comprendido e interactuado por agentes y otros consumidores**.

| Dimensión | TUP: punto de partida | SASP: enfoque vigente |
| --- | --- | --- |
| Nombre | Thanos UI Protocol | Semantic Application & Service Protocol |
| Motivación | Interacción agente–UI en Thanos OS | Contrato semántico para aplicaciones y servicios |
| Proveedores | Aplicaciones y contexto de vistas | Aplicaciones gráficas, headless y daemons |
| Consumidores | Capsuleer y Thanos Shell como primeros casos | Agentes y clientes diversos; Thanos OS como posible implementador |
| Presentación | `View` como concepto temprano central | Contexto de interacción opcional, separado del contrato semántico |
| Transporte | JSON-RPC/sockets Unix y broker en estudio | Transporte y adaptadores por definir |
| Ecosistema | Investigación enfocada en Thanos OS | Comparación e interoperabilidad con MCP y MCP Apps |

**SASP no sustituye MCP, Linux ni las API nativas por decisión de nombre.** La arquitectura exacta —protocolo propio, perfil sobre MCP, o contrato interno con puentes— continúa abierta.

## 4. Qué se preserva y qué queda por validar

### Principios preservados

- Descubrir aplicaciones y servicios como entidades reconocibles.
- Exponer identidad, estado, capacidades, métodos y eventos.
- Permitir clientes humanos y agentes sobre información semántica compartida.
- Mantener el contrato separado de la presentación y del transporte.
- Conservar la autoridad efectiva fuera del agente y registrar operaciones relevantes.
- Definir explícitamente recuperación, consistencia, versiones y errores antes de prometer garantías.

### Cuestiones abiertas

1. ¿Qué campos debe declarar obligatoriamente cualquier proveedor SASP?
2. ¿Qué modelo de estado, revisiones y eventos necesita una instancia?
3. ¿Cómo se autentica y descubre una aplicación sin confiar en identidades autodeclaradas?
4. ¿Qué mecanismos de MCP bastan y cuáles requerirían un perfil o extensión?
5. ¿Cuál es el coste real de conectar aplicaciones mediante sockets Unix directos, un broker o un adaptador MCP?
6. ¿Qué responsabilidades pertenecen al proveedor, al Task Runtime, al Shell y a la autoridad de seguridad?
7. ¿Cómo se conserva interoperabilidad entre implementaciones independientes?

Estas preguntas definen una **agenda de investigación**, no un protocolo normativo ya implementado.

## 5. Relación entre el archivo TUP y SASP

La documentación de **TUP** se conserva para entender el origen, la arquitectura propuesta y las decisiones que motivaron el cambio de alcance. La investigación **actual** se publica bajo SASP:

- [SASP — definición, alcance y experimentos](../../sasp/README.md)
- [DECISION-0001 — Nombre y alcance de SASP](../../sasp/DECISION-0001-nombre-y-alcance.md)
- [Issue #13 — Línea de investigación SASP](https://github.com/SantosJMM/os-ai-research/issues/13)

## 6. Referencias técnicas

- [Model Context Protocol — especificación](https://modelcontextprotocol.io/specification)
- [MCP Apps — extensión oficial y ejemplos](https://github.com/modelcontextprotocol/ext-apps)
- [MCP Apps — presentación de la extensión](https://blog.modelcontextprotocol.io/posts/2026-01-26-mcp-apps/)

**Conclusión:** TUP comenzó como una respuesta al problema de interactuar con interfaces gráficas desde un agente. El análisis llevó a un problema de mayor alcance: la capacidad de cualquier aplicación o servicio de describir semánticamente **qué es, qué hace, en qué estado está y cómo puede interactuarse con él**. Esa línea de investigación continúa como **SASP**.
