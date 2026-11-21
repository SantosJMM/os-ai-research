# SASP — Experimentos de integración MCP en el sistema operativo

> **Estado:** plan experimental. Las pruebas descritas aquí no se consideran ejecutadas hasta que se publiquen resultados y evidencias. No requiere que SASP disponga de un transporte RPC propio.

## Propósito

Determinar si las primitivas existentes de MCP y MCP Apps son suficientes para incorporar aplicaciones y servicios a un sistema operativo: **instalación, registro, descubrimiento, conexión, ciclo de vida, estado, eventos y representación**.

El experimento base utiliza dos proveedores sintéticos existentes:
- [Music Player](../../examples/music-player/manifest.json), con estado y controles de volumen.
- [Backup Daemon](../../examples/backup-daemon/manifest.json), sin necesidad de interfaz.
- [Servidor MCP experimental en Rust](../mcp-rust-stdio/README.md), todavía pendiente de compilación y pruebas de integración.

**No conectar los ensayos a servicios de producción ni abrir acceso a recursos del anfitrión no requeridos.** Cada experimento debe anotar versión de MCP, implementación, sistema, comandos, resultados, fallos y mediciones.

## Matriz de seis experimentos

| ID | Eje SASP | Hipótesis verificable | Prueba | Evidencia mínima |
| --- | --- | --- | --- | --- |
| **OS-01** | Application Contract | Ambos proveedores pueden describirse con MCP Tools/Resources, sin nuevos RPC | Descubrir herramientas y leer manifiestos/snapshots | Capturas JSON, validación de esquemas y correspondencia entre capacidades |
| **OS-02** | SASP Registry | El SO puede mantener instalaciones e instancias independientes, con identidades verificables | Simular instalación, inicio, cierre y reinicio de dos instancias | Tabla de registros antes/después; detección de registros obsoletos |
| **OS-03** | SASP Discovery | Un servidor MCP del SO puede buscar proveedores por capacidad y devolver solo destinos accesibles | Consultar por `music`, `volume`, `backup`; comparar usuarios/perfiles | Resultados y denegaciones, trazabilidad de identidad |
| **OS-04** | SASP Connection | `stdio`, Unix directo y conexión mediada pueden preservar la semántica MCP | Probar invocación, desconexión, dos clientes y revocación | Métricas de latencia/RSS/FD, fallos y política efectiva |
| **OS-05** | Lifecycle & Events | Los recursos y `subscriptions/listen` cubren cambios de estado simples | Cambiar volumen, recibir notificación, releer recurso; reiniciar instancia | Orden observado, pérdidas, duplicados, capacidad de resincronización |
| **OS-06** | MCP Integration & Presentation | Una UI humana y un agente pueden consumir el mismo proveedor | Exponer Music Player vía MCP Apps opcional; usar Backup Daemon sin UI | Contrato compartido, permisos verificados y límites de UI nativa documentados |

Estos seis ejes pueden desarrollarse en paralelo donde no haya dependencias, pero **OS-01** proporciona la línea base de compatibilidad.

## OS-01 — Contrato MCP nativo

- Iniciar el servidor experimental de Rust con datos sintéticos.
- Consultar `server/discover` cuando el SDK soporta MCP 2026-07-28, o registrar la negociación de versión que efectivamente admita el servidor.
- Enumerar `tools/list` y `resources/list`; leer dos manifiestos y dos estados.
- Comprobar que un cliente encuentra `player.set_volume` y `backup.verify` por declaración explícita.
- Comprobar validación de argumentos y rechazo de mutaciones por defecto.
- Documentar diferencias entre el esquema de la herramienta y el manifiesto SASP para evitar dos fuentes de verdad contradictorias.

**Criterio:** el contrato semántico se transmite íntegro por MCP sin inventar mensajes de transporte SASP.

## OS-02 — Registro local (instalado vs activo)

- Crear un registro experimental en un proceso **sin root** con dos conjuntos: catálogo de paquetes y directorio de instancias.
- Registrar un proveedor instalado sin proceso ejecutándose.
- Iniciar una instancia local y publicar un endpoint asociado a un proceso comprobado.
- Reiniciar una instancia y confirmar que su identidad efímera o su generación cambia de forma controlada.
- Introducir un registro caducado o un PID reutilizado y comprobar que no se toma como autenticación.
- Simular desinstalación: el catálogo se limpia sin omitir el cierre de instancias dependientes.

**Criterio:** ninguna identidad autodeclarada por JSON basta para hacerse pasar por una instancia confiable.

## OS-03 — Servicio Discovery expuesto por MCP

- Publicar un **servidor MCP de descubrimiento del SO** sobre el registro local de OS-02.
- Exponer herramientas de búsqueda/obtención de instancias y recursos de catálogo usando MCP.
- Hacer consultas por capacidad, no solo por nombre.
- Mostrar para cada resultado identidad, versión, disponibilidad y una **referencia lógica autorizada**.
- Filtrar el resultado según credenciales de dos perfiles simulados.
- Verificar que descubrir una aplicación no la inicia ni autoriza sus operaciones.

**Criterio:** un agente encuentra aplicaciones sin aprender rutas locales arbitrarias y sin recibir acceso directo a proveedores prohibidos.

## OS-04 — Conexión y aislamiento

- Comparar `stdio` para servidores lanzados por el cliente, Unix socket para instancias persistentes y una ruta mediada.
- En Unix socket, reutilizar el framing recomendado por MCP (JSON-RPC delimitado por salto de línea) si se implementa un transporte personalizado.
- Verificar identidad del cliente por mecanismos adecuados del SO.
- Probar dos clientes, pérdida del endpoint, cancelación y reconexión.
- Medir FD abiertos, RSS, tiempo de descubrimiento y latencia de una invocación.
- Aislar al cliente del endpoint si el perfil de seguridad lo exige.

**Criterio:** la decisión sobre broker se toma con datos y pruebas de seguridad, no por preferencia estética o topológica.

## OS-05 — Eventos, suscripciones y continuidad

- El proveedor Music Player publica una instantánea de estado como recurso MCP.
- El cliente abre `subscriptions/listen` solicitando actualizaciones de ese recurso.
- Modificar el volumen; verificar `notifications/resources/updated` y la relectura del snapshot.
- Desconectar el cliente durante múltiples cambios y reconectarlo; determinar qué puede reconstruirse únicamente desde la instantánea.
- Probar evento de dominio `volume.changed`: comparar un recurso/event log tipado frente a la notificación de cambio de recurso.
- Repetir con caída y reinicio de proveedor, además de una tarea MCP Tasks si el SDK la admite.

**Criterio:** documentar exactamente qué garantiza MCP y qué requisitos de eventos **no** cubre. Proponer una extensión SASP solo si un requisito real queda sin resolver.

## OS-06 — Presentación y clientes

- Construir una vista humana para Music Player sin alterar su contrato de herramientas.
- Probar un host compatible con **MCP Apps** mediante `ui://`, siempre que el SDK y el host soporten la extensión.
- Probar un cliente de agente que no necesite ni solicite la UI.
- Comprobar que Backup Daemon funciona sin `interaction` ni `ui://`.
- Documentar qué casos de ventanas GTK/Qt, composición nativa, accesibilidad y estado de la ventana siguen fuera de MCP Apps.
- Verificar que la vista no puede saltarse las autorizaciones de las herramientas.

**Criterio:** una misma lógica de aplicación sirve a consumidor humano y a agente, y la UI permanece opcional.

## Dependencias y resultados

```text
OS-01 (contrato MCP)
    |
    +----> OS-02 (catálogo + instancias)
    |            |
    |            +----> OS-03 (Discovery como MCP)
    |                       |
    |                       +----> OS-04 (transporte y política)
    |
    +----> OS-05 (estado, eventos y continuidad)
    |
    +----> OS-06 (MCP Apps + UI propia + daemon)
```

Para cada prueba registrar:
- ID y commit de código, versión de MCP y SDK;
- escenario, identidad del consumidor y política aplicable;
- comandos reproducibles y datos sintéticos;
- resultado observado y evidencia de error/denegación;
- mediciones y límites;
- conclusión: **reutilizar MCP**, **convención SASP**, **extensión SASP justificada** o **responsabilidad nativa del SO**.

## Decisiones que no se deben adelantar

- No presentar el actual servidor de prueba `stdio` como implementación de SASP Registry o Discovery.
- No exigir broker hasta medir conectividad directa, aislamiento y necesidades de concurrencia.
- No inventar un sistema de eventos paralelo mientras `subscriptions/listen` y recursos no estén evaluados.
- No obligar MCP Apps ni HTML a aplicaciones que ya ofrecen UI nativa.
- No confundir registrarse en el catálogo local con el MCP Registry público.
- No utilizar `initialize` ni antiguas llamadas de suscripción como si fueran el contrato moderno de MCP 2026-07-28.
- No tratar la presencia de una herramienta como permiso de invocación.

## Relación con la arquitectura

- [SASP: integración de MCP con el SO](../../docs/os-integration.md)
- [Arquitectura general](../../docs/architecture.md)
- [Análisis previo de interoperabilidad](../../docs/interoperability-mcp.md)
- [Contrato semántico experimental](../../specification/draft/overview.md)
