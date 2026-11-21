# SASP — Experimento de integración con MCP en Rust

> **Estado:** primera prueba de concepto, no implementación normativa de SASP. Expone contratos de ejemplo con el SDK oficial de MCP para Rust (`rmcp`) y transporte `stdio`.

**Posición dentro del proyecto:** este servidor es la **línea base OS-01 de Tools/Resources**; no implementa SASP Registry, Discovery, un gestor de instalación, identidad de proceso ni conexión mediada. Esos elementos están en el [plan OS-01 a OS-06](../os-integration/README.md) y en el [modelo de Registry/Discovery](../../docs/registry-discovery.md).

## Objetivo

Validar el principio central de SASP: **una aplicación o servicio declara sus capacidades y estado para que los clientes los descubran**, sin depender de su interfaz gráfica y sin que el agente obtenga privilegios del sistema operativo.

Este prototipo utiliza los mismos datos de referencia de la investigación:

- [Music Player — manifiesto](../../examples/music-player/manifest.json) y [estado](../../examples/music-player/state.snapshot.json)
- [Backup Daemon — manifiesto](../../examples/backup-daemon/manifest.json) y [estado](../../examples/backup-daemon/state.snapshot.json)
- [Contrato histórico TUP — Thanos Music](../../../tup/archivo/05-contrato-semantico-thanos-music.md)
- [Análisis de interoperabilidad con MCP](../../docs/interoperability-mcp.md)

## Implementación inicial

Un único servidor MCP local ofrece cuatro recursos:

| URI | Función |
| --- | --- |
| `sasp://music/manifest` | Contrato semántico del reproductor |
| `sasp://music/instances/music-01/state` | Estado dinámico y revisión |
| `sasp://backup/manifest` | Contrato semántico del daemon |
| `sasp://backup/instances/backup-01/state` | Estado de ejemplo del daemon |

También expone tres herramientas MCP: `player.play`, `player.set_volume` y `backup.verify`. Las herramientas tienen esquemas derivados de tipos Rust y responden con estructuras tipadas. `backup.verify` opera únicamente con el identificador sintético `demo-backup-001`, sin acceder a discos ni ejecutar utilidades externas.

El estado de Music Player comienza con pista `Demo Track`, volumen `45`, reproducción `paused` y revisión `7`; una mutación aceptada incrementa la revisión.

## Requisitos y ejecución

Necesita Rust compatible con edición 2024 y acceso a dependencias de Cargo.

~~~sh
cd protocolos/sasp/experiments/mcp-rust-stdio
cargo test
cargo run
~~~

El proceso sirve **MCP sobre stdin/stdout** y debe ser lanzado por un cliente MCP. No escriba logs en stdout, porque se reserva para el protocolo.

Para explorar con MCP Inspector, compilado el binario:

~~~sh
cargo build
npx @modelcontextprotocol/inspector --cli target/debug/sasp-mcp-demo --method resources/list
~~~

## Modelo de permisos del experimento

Por defecto, `player.play` y `player.set_volume` **rechazan las modificaciones**. Para habilitar únicamente las mutaciones sintéticas durante una prueba local:

~~~sh
SASP_DEMO_ALLOW_MUTATIONS=1 cargo run
~~~

**Este interruptor no es autenticación, autorización por identidad ni una implementación de Ship.** El programa no posee controles de aislamiento adicionales ni debe exponerse a clientes no confiables. No hace cambios sobre servicios ni sobre archivos del sistema.

## Criterios de prueba

1. El cliente enumera y lee ambos manifiestos mediante `resources/list` y `resources/read`.
2. La misma estructura describe un reproductor y un daemon sin interfaz gráfica.
3. El cliente enumera métodos mediante `tools/list`.
4. Las mutaciones se rechazan por defecto.
5. Con la opción experimental habilitada, ajustar volumen a 70 cambia la instantánea y su revisión.
6. Los métodos se descubren por sus contratos de entrada y salida; el cliente no infiere capacidades inexistentes.

## Fuera de alcance de esta primera etapa

No implementa suscripciones, eventos push, replay, broker, registro local de procesos, MCP Apps, trabajo duradero, múltiples instancias, persistencia ni un modelo real de seguridad. Tampoco verifica interoperabilidad con versiones antiguas de MCP.

La siguiente iteración deberá comprobar primero **versión MCP soportada y compilación/ejecución real**, y después `subscriptions/listen`, notificaciones, reconexión y controles efectivos. **No se considera una implementación funcional de integración con el SO** hasta que Registry, Discovery y políticas se prueben separadamente.

## Referencias

- [SDK oficial de MCP para Rust](https://github.com/modelcontextprotocol/rust-sdk)
- [Especificación MCP 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28)
