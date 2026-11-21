# Thanos UI Protocol (TUP) — línea de investigación

[Issue #13: Thanos UI Protocol](https://github.com/SantosJMM/os-research/issues/13)

## Definición

**TUP** significa **Thanos UI Protocol** (*Protocolo de Interfaz de Usuario de Thanos*).

Es el protocolo mediante el cual las aplicaciones exponen su **estado, métodos, eventos y contexto de interfaz** para comunicarse con la IA.

Esta es la definición central del proyecto. Las capacidades adicionales, los transportes, permisos, esquemas y garantías son aspectos de diseño e investigación, no parte obligatoria de la expansión del acrónimo.

## Visión

Investigar un **contrato nativo entre aplicaciones, humanos y agentes de IA** para un sistema operativo en el que la IA sea un participante de primera clase.

En la propuesta actual, las aplicaciones exponen de forma estructurada su **estado, métodos, eventos y contexto de interfaz**; se investiga además cómo representar capacidades y acciones mediante contratos tipados. El agente no debe depender únicamente de capturas de pantalla, coordenadas o controles visuales.

**TUP es una idea de protocolo en diseño**, no una especificación estable ni una funcionalidad que se presuma implementada en todas las aplicaciones de Thanos OS.

## Qué problema queremos resolver

Las interfaces gráficas tradicionales están optimizadas para personas. Los agentes suelen interactuar mediante APIs ad hoc, automatización de pantalla o herramientas con integraciones específicas. El objetivo es estudiar si una interfaz semántica común permite control más robusto, explícito, observable y auditable.

La hipótesis **debe comprobarse** contra enfoques existentes: APIs convencionales, MCP, accesibilidad, modelos de recursos y Schemes de Redox. Un protocolo nuevo tiene costes de adopción y no garantiza por sí mismo seguridad ni buenas interfaces.

## Capas conceptuales (propuesta de estudio)

```text
Persona ──────────> UI gráfica
                       │
Agente de IA ─────> Cliente TUP
                       │
               Modelo semántico TUP
          estado / acciones / eventos / UI
                       │
             Transporte (intercambiable)
            Unix socket / broker / otros
                       │
           App o servicio que publica TUP
                       │
              Recursos y operaciones
```

La autorización y aislamiento de recursos **no están garantizados por TUP**: requieren controles efectivos, por ejemplo Ship y mecanismos del kernel. Los caminos de conexión deben diseñarse de modo que Capsuleer no eluda esas políticas.

## Ejes de especificación

1. **Descubrimiento y ciclo de vida:** aplicación, instancia, versión, salud, desconexión.
2. **Estado:** recursos, propiedades, tipos, instantáneas y revisiones.
3. **Métodos:** parámetros, resultados, errores, efectos e idempotencia.
4. **Eventos:** suscripción, secuencia, pérdida, reanudación y retención.
5. **Presentación:** representación humana gráfica y representación semántica para agentes.
6. **Seguridad:** identidad verificada, permisos, scopes, revocación y auditoría.
7. **Transporte:** conexión directa mediante Unix sockets frente a broker; el modelo TUP permanece separado del transporte.
8. **Compatibilidad:** evolución del protocolo, extensiones, versiones y adaptadores.

## Experimentos iniciales

### TUP-01 — Aplicación ficticia de música

Usar datos sintéticos para publicar estado (reproducción, volumen), métodos (reproducir, pausar, ajustar volumen) y eventos (cambio de pista o volumen). Definir primero el contrato, **sin confundir estos ejemplos con el esquema definitivo de TUP**.

### TUP-02 — Dos consumidores

Conectar una UI mínima para humanos y un agente simulado que utilicen el **mismo estado semántico**. Evaluar si ambos pueden operar sin lógica duplicada de negocio.

### TUP-03 — Comparación de transporte

Usar el mismo contrato con Unix socket directo y con un registro/broker. Evaluar descubrimiento, consumo de recursos, latencia, fallos y complejidad.

### TUP-04 — Autorización real

El agente solicita una operación que la política deniega. Comprobar que no ocurre el efecto, que el cliente recibe un error estructurado y que queda evidencia auditable.

### TUP-05 — Recuperación

Reiniciar la aplicación y evaluar reconexión, estado y comportamiento de eventos perdidos. Definir las garantías explícitamente antes de medirlas.

## Criterios de éxito

- Contrato comprensible y compatible con clientes distintos.
- Acciones y cambios de estado reproducibles y observables.
- Permisos aplicados por una autoridad independiente del agente.
- Comportamiento definido ante caída, reconexión y eventos duplicados.
- Comparativa con alternativas documentada sin asumir superioridad.

## Frontera con Thanos OS

Esta carpeta contiene **investigación y experimentación**. Las decisiones oficiales, implementación y especificación aprobada del protocolo, si existen, pertenecen al repositorio principal de Thanos OS. Mantener enlaces bidireccionales; no duplicar una especificación oficial mutable aquí.

## Relaciones

- [Agentes y sistemas operativos — Issue #12](https://github.com/SantosJMM/os-research/issues/12)
- [Hipótesis Thanos OS — Issue #9](https://github.com/SantosJMM/os-research/issues/9)
- [Fundamentos de IPC — Issue #3](https://github.com/SantosJMM/os-research/issues/3)
