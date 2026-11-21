# SASP — Semantic Application & Service Protocol

> **Identidad adoptada por OS AI Research · 21 de noviembre de 2026**  
> **Pronunciación propuesta:** «Sasp» (una sílaba; inglés /sæsp/, español /sasp/).  
> **Estado técnico:** investigación / borrador; **no** existe una especificación v1.0 ni se afirma interoperabilidad implementada.

**SASP** (*Semantic Application & Service Protocol*, «Protocolo Semántico de Aplicaciones y Servicios») es nuestra propuesta de un contrato abierto mediante el cual **aplicaciones gráficas, aplicaciones sin interfaz y servicios/daemons** exponen su **identidad, estado, capacidades, métodos, eventos y contexto de interacción** a agentes de IA y otros clientes.

Su objetivo es que el software pueda describirse semánticamente sin depender exclusivamente de pantallas, coordenadas, interfaces HTML o integraciones específicas por cada agente. **TUP (Thanos UI Protocol)** fue el nombre original de esta investigación; se reemplazó para que la propuesta pueda adoptarse independientemente de Thanos OS. Véase [registro de decisión](DECISION-0001-nombre-y-alcance.md).

## Principios arquitectónicos

1. **Aplicaciones y servicios por igual.** Una app GTK/Qt, un proceso sin ventana, un worker o un daemon pueden exponer SASP, directamente o mediante un adaptador.
2. **Contrato semántico independiente de la presentación.** Describir estado y acciones no requiere UI gráfica. Si existe contexto de interfaz, puede ser consumido por humanos o agentes.
3. **No sustituir mecanismos nativos.** SASP no reemplaza señales (`kill`, `pidfd_send_signal`), syscalls, procesos, supervisores, cgroups, protocolos de servicio (p. ej. PostgreSQL) ni APIs especializadas.
4. **Descubrir no es autorizar.** Publicar una capacidad no concede permisos. El proveedor y los mecanismos de seguridad aplicables deben aplicar autorización real; una IA no es una autoridad de seguridad.
5. **Transporte por decidir.** La semántica de SASP se estudia por separado de sockets Unix, registro/broker o puentes hacia otros protocolos.
6. **Aprovechar estándares existentes.** Antes de duplicar MCP, MCP Apps, recursos, herramientas o suscripciones, comparar y justificar cualquier extensión propia.
7. **Permitir fallos y recuperación.** Evaluar versiones, identidad de instancias, estado revisable, eventos duplicados o perdidos, cancelación y reconexión.

## Modelo conceptual (NO esquema normativo)

```text
     Cliente agente                    Interfaz humana
           |                                 |
           +---------- consumidores ---------+
                             |
                      Contrato SASP
            identidad / estado / capacidades
                 métodos / eventos / contexto
                             |
                  transporte o adaptador
                             |
            aplicación, servicio o daemon
                             |
                   APIs y mecanismos nativos
```

La aplicación es responsable de su lógica. Una operación privilegiada debe seguir las políticas y mecanismos pertinentes del sistema. En Thanos OS eso puede involucrar componentes de autorización y auditoría, pero SASP **no exige** usar Thanos OS.

### Dimensiones candidatas del contrato

| Dimensión | Pregunta que responde |
| --- | --- |
| Identidad | ¿Qué aplicación, servicio e instancia es? |
| Estado | ¿Qué sucede ahora? ¿Cómo se interpreta? |
| Capacidades y métodos | ¿Qué operaciones declara, con qué argumentos, efectos y errores? |
| Eventos | ¿Qué cambios pueden observarse y cómo se recuperan? |
| Contexto de interacción | ¿Cómo debería comprenderse o representarse para distintos clientes? |

**Pendientes de diseño:** formatos y esquemas, negociación de versiones, seguridad y permisos de observación, identidad verificable, descubrimiento, consistencia de estado, transporte, ciclos de vida y compatibilidad. La tabla no implica decisiones definitivas sobre campos ni métodos RPC.

## Qué queda fuera

- Ser un reemplazo de Linux, su API de procesos, señales o supervisor de servicios.
- Convertir cada daemon en una app gráfica.
- Imponer HTML/iframes o un toolkit de escritorio específico.
- Usar descripciones o prompts como sustituto de permisos y aislamiento.
- Reemplazar automáticamente MCP: **MCP/MCP Apps pueden ser un puente o una implementación posible** que debemos evaluar.

## Experimentos iniciales

1. **SASP-01 — App de música simulada:** identidad, estado, métodos y eventos con datos sintéticos.
2. **SASP-02 — Dos consumidores:** agente simulado y UI humana usando las mismas capacidades.
3. **SASP-03 — Transportes:** socket Unix directo vs registro/broker con igual contrato.
4. **SASP-04 — Autorización:** acción rechazada por política; verificar ausencia de efectos y registro auditable.
5. **SASP-05 — Recuperación:** reiniciar proveedor, descubrir estado y tratar suscripciones/eventos perdidos.
6. **SASP-06 — MCP:** implementar el mismo caso con MCP Resources/Tools y opcionalmente MCP Apps; comparar complejidad, semántica, comportamiento y reutilización.

## Relación con otros proyectos

- **OS AI Research:** laboratorio donde investigamos y validamos la propuesta.
- **Thanos OS:** posible implementador nativo, pero no requisito de adopción.
- **MCP y MCP Apps:** tecnologías para evaluar interoperabilidad; sus capacidades no se atribuyen automáticamente a SASP.

**Seguimiento:** [Issue #13 — SASP](https://github.com/SantosJMM/os-ai-research/issues/13) · [Roadmap](../../roadmap.md) · [Archivo técnico de TUP y ejemplos JSON](../tup/archivo/README.md).

Este documento es una **declaración de nombre, alcance y objetivos de investigación**, no un estándar ratificado ni un contrato de wire format listo para implementar.
