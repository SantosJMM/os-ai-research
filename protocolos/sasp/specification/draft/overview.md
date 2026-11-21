# SASP — Contrato mínimo experimental (draft)

> **Estado: borrador exploratorio no normativo.** Los nombres de campos y las convenciones de este documento son candidatos destinados a probarse mediante prototipos. No existe todavía una versión estable de SASP, un transporte oficial, pruebas de conformidad aprobadas ni garantía de compatibilidad entre implementaciones.

## 1. Alcance

Este borrador propone un **modelo mínimo común** para una aplicación gráfica y un daemon sin interfaz. El propósito es comprobar si ambos pueden describirse de forma uniforme, ser comprendidos por un agente y ofrecer información suficiente para una interfaz humana, sin imponer un transporte ni un toolkit visual.

Los requisitos de estudio son:

- Identificar un **proveedor** independientemente de sus instancias.
- Anunciar métodos y eventos con descripciones y contratos de datos.
- Describir la **forma** del estado sin exigir que el manifiesto contenga su valor actual.
- Obtener una **instantánea dinámica** vinculada a una instancia.
- Mantener el contexto de interacción **opcional**.
- Evitar que la descripción del proveedor se confunda con una concesión de autoridad.

Este borrador define **formas de datos candidatas**. No define handshake, discovery RPC, método de invocación, autenticación, sesiones ni semántica de entrega de eventos.

## 2. Separación de tres tipos de información

| Tipo | Propósito | Frecuencia típica |
| --- | --- | --- |
| **Provider Manifest** | Identidad lógica y capacidades anunciadas, incluidos esquemas. | Cambios de contrato o versión. |
| **State Snapshot** | Valores observados para una instancia concreta. | Actualizaciones en tiempo de ejecución. |
| **Event** | Cambio o hecho observable relativo a una instancia. | A medida que ocurren cambios. |

Las tres estructuras pueden viajar por un mismo canal o por mecanismos diferentes. Un cliente no debe asumir que un manifiesto contiene el estado actual ni que un evento reemplaza una instantánea.

## 3. Provider Manifest

Campos candidatos:

| Campo | Función | Condición en el borrador |
| --- | --- | --- |
| `sasp_version` | Identifica la variante experimental del contrato. | Presente en los ejemplos. |
| `provider.id` | Identificador semántico estable del proveedor. | Presente. No es autenticación. |
| `provider.name` | Nombre descriptivo legible. | Presente. |
| `provider.kind` | Tipo general: `desktop`, `daemon` u otra categoría. | Presente. Enumeración abierta. |
| `capabilities.state.schema` | Forma candidata de las propiedades observables. | Presente en los ejemplos. |
| `capabilities.methods` | Lista de métodos con contratos de entrada y salida. | Presente, puede estar vacía. |
| `capabilities.events` | Lista de eventos con contrato de datos. | Presente, puede estar vacía. |
| `interaction` | Contexto de presentación y acciones destacadas. | Opcional; no exigido a daemons. |

La estructura de métodos utiliza `name`, `description`, `inputSchema` y `outputSchema` como campos de ejemplo. Los esquemas ilustran **JSON Schema**, sin comprometerse todavía con una versión o subconjunto específicos de ese estándar.

El campo `effect` (por ejemplo, `observe` o `act`) es una **anotación orientativa**. No es una autorización, garantía de ausencia de efectos ni sustituto de una política.

Ejemplos completos: [Music Player](../../examples/music-player/manifest.json) y [Backup Daemon](../../examples/backup-daemon/manifest.json).

## 4. State Snapshot

Una instantánea se asocia a una **instancia de proveedor**:

```json
{
  "kind": "state.snapshot",
  "provider_id": "org.example.music",
  "instance_id": "music-01",
  "revision": 7,
  "state": {
    "playback": "paused",
    "track": "Demo Track",
    "volume": 45
  }
}
```

En este modelo experimental, el contenido de `state` debería respetar el esquema `capabilities.state.schema` de su manifiesto.

**Garantías abiertas:** alcance y estabilidad de `revision`, reinicio de contador tras caída, consultas consistentes, información parcial, sincronización entre instancias y detección de manifestos incompatibles. Un número creciente **no equivale automáticamente** a una garantía de consistencia distribuida.

Los ejemplos de instantánea están en [Music Player](../../examples/music-player/state.snapshot.json) y [Backup Daemon](../../examples/backup-daemon/state.snapshot.json).

## 5. Event

Estructura candidata de evento:

```json
{
  "kind": "event",
  "provider_id": "org.example.music",
  "instance_id": "music-01",
  "name": "playback.changed",
  "sequence": 8,
  "data": {
    "playback": "playing"
  }
}
```

`sequence` es una opción de estudio para ordenar o detectar lagunas en un flujo de eventos; **no implica entrega exactamente una vez, retención, confirmación ni recuperación**. Esas garantías requieren un diseño propio y pruebas.

Los eventos pueden contener solo cambios parciales. Un cliente que pierde eventos debería poder consultar una instantánea actualizada, cuando ese mecanismo esté definido.

## 6. Invocación de un método

Un método anunciado define la **interfaz semántica**, no la ruta de ejecución. La secuencia lógica es:

```text
Cliente -> descubre proveedor -> obtiene manifiesto
        -> consulta estado, si es relevante
        -> solicita método con entrada validada
        -> proveedor/autoridad verifica identidad y permisos
        -> respuesta: resultado / rechazo / error / tarea aceptada
```

Se investigarán errores distinguibles de **entrada inválida**, **capacidad ausente**, **acceso denegado**, **estado incompatible**, **proveedor no disponible** y **fallo de ejecución**. Los identificadores de error, el tratamiento de trabajos asíncronos y la deduplicación **no están fijados**.

Una solicitud de cliente nunca se considera autorizada por estar descrita en el manifiesto. El valor de `effect` puede orientar a las interfaces, pero la autorización debe residir en mecanismos efectivos del proveedor o del sistema.

## 7. Contrato opcional de interacción

Un manifiesto de aplicación gráfica puede incluir:

```json
{
  "interaction": {
    "title": "Music Player",
    "primary_methods": ["player.play", "player.set_volume"]
  }
}
```

Es **contexto**, no código ejecutable ni un modelo completo de ventanas. Los clientes pueden usarlo para seleccionar controles, sugerir acciones o agrupar funciones. Un daemon puede omitir `interaction` por completo sin perder su condición de proveedor SASP.

Las vistas gráficas declarativas, si se desean, deberán investigarse como **extensión de presentación**, incluida su posible relación con MCP Apps.

## 8. Compatibilidad de los ejemplos

Los manifiestos de ejemplo comparten la misma forma de datos. Las instantáneas se pueden asociar a los proveedores mediante `provider_id`, y el estado ejemplifica sus propiedades publicadas.

**Criterios de revisión para el siguiente prototipo:**

1. Un consumidor puede leer ambos manifiestos sin reglas especiales para una app gráfica o un daemon.
2. Puede enumerar métodos y eventos, así como inspeccionar la forma esperada de las entradas, salidas y estado.
3. Puede distinguir identidad del proveedor de identidad de la instancia.
4. Puede consultar un snapshot coherente con el manifiesto para cada proveedor.
5. Puede identificar cuándo la interacción humana es opcional.
6. No puede deducir privilegios de la mera presencia de métodos.
7. La equivalencia con mecanismos de MCP se evalúa explícitamente antes de fijar un transporte propio.

## 9. Requisitos y decisiones todavía abiertos

**No se ha elegido:** sintaxis de nombres y URIs, autoridad sobre la identidad de proveedor, origen y estabilidad de `instance_id`, negociación de versiones, mecanismo de discovery, contrato de transporte y seguridad, representación de errores, consistencia de estado, entregabilidad de eventos, suscripciones, cancelación, reintentos, idempotencia, extensión de UI o relación definitiva con MCP.

Este contrato experimental es una base para **formular pruebas y comparar alternativas**, no un estándar de wire format.

## 10. Documentación relacionada

- [Vocabulario](../../docs/concepts.md)
- [Arquitectura](../../docs/architecture.md)
- [SASP — descripción principal](../../README.md)
- [MCP — especificación](https://modelcontextprotocol.io/specification)
