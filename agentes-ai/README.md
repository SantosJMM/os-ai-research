# Agentes de IA y sistemas operativos

Esta línea de OS AI Research estudia **agentes de IA como software que interactúa con sistemas operativos**, no únicamente modelos de lenguaje.

## Objetivos
- Entender modelos, inferencia, herramientas, bucles de agente y runtimes.
- Diseñar interfaces seguras entre agentes y recursos de un SO.
- Estudiar identidad, autoridad, cancelación, recuperación, eventos, persistencia y auditoría.
- Comparar diseños convencionales con un sistema operativo que incorpore a la IA como participante de primera clase.

## Rutas
- [Fundamentos de agentes](fundamentos/README.md) — [Issue #11](https://github.com/SantosJMM/os-research/issues/11)
- [Agentes e integración con sistemas operativos](os-integration/README.md) — [Issue #12](https://github.com/SantosJMM/os-research/issues/12)
- [Hipótesis específica de Thanos OS](../thanos-os/hipotesis.md) — [Issue #9](https://github.com/SantosJMM/os-research/issues/9)

## Reglas de investigación
1. Distinguir comportamiento del modelo de garantías comprobables del sistema.
2. No usar prompts como sustituto de controles de permisos, aislamiento o políticas.
3. Los experimentos con herramientas se desarrollan en entornos aislados y con datos sintéticos.
4. Documentar versión, coste, límites, escenarios de fallo y resultados observables.
5. Mantener separado el código principal de Thanos OS.

Esta línea puede avanzar en paralelo con la arquitectura de computadores y los fundamentos de kernels.
