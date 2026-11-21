# Ejemplos de integración SASP en el sistema operativo

> **Estado:** datos sintéticos de investigación para OS-02 (Registry) y OS-03 (Discovery). No son mensajes normativos de SASP ni representan un sistema real.

## Archivos

- [`registry.snapshot.json`](registry.snapshot.json): un proveedor de música y un servicio de backup instalados; solamente la instancia de música aparece activa.
- [`discovery.search.result.json`](discovery.search.result.json): resultado filtrado de una consulta por capacidad semántica.

Los identificadores de proceso, handles, generaciones y revisiones son **datos de ejemplo**, sin significado de autenticación real.

## Lectura de los ejemplos

El registro mantiene por separado:

1. **Instalaciones:** inventario alimentado por gestor de paquetes o instalador confiable.
2. **Instancias:** procesos y endpoints asociados a identidad comprobada por mecanismos del SO.

Una instalación no equivale a una instancia activa. El daemon de backup está instalado pero no tiene instancia en la instantánea.

El índice `semantic_index` es una **convención de búsqueda experimental**. `music.playback` es una etiqueta de clasificación derivada del dominio; **no es automáticamente un método MCP ni un campo que MCP exija**. Los métodos anunciados en el contrato base del Music Player siguen siendo `player.play` y `player.set_volume`.

La respuesta Discovery devuelve un `connection_ref` opaco, no una ruta privilegiada ni un permiso permanente. El consumidor debe resolverlo a través de una ruta autorizada y el proveedor debe verificar la invocación.

## Pruebas requeridas

- Comprobar que `provider_id` del catálogo coincide con un manifiesto de proveedor instalado.
- Comprobar que las entradas en `instances` hacen referencia a proveedores del catálogo.
- Comprobar el caso de instalado/inactivo.
- Negar resoluciones revocadas o caducadas.
- Demostrar que un PID recibido de datos no basta para autenticar un proceso.
- Comparar resultados por identidad y alcance del solicitante.

## Referencias

- [Registry y Discovery](../../docs/registry-discovery.md)
- [API candidata de Discovery MCP](../../experiments/os-integration/discovery-mcp.md)
- [Plan OS-01 a OS-06](../../experiments/os-integration/README.md)
- [MCP Registry](https://github.com/modelcontextprotocol/registry)
