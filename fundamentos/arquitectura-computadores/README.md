# Unidad 00 — Arquitectura de computadores

**Seguimiento:** [Issue #10](https://github.com/SantosJMM/os-ai-research/issues/10)

## Propósito

Comprender las propiedades del hardware que los sistemas operativos utilizan para gestionar procesos, memoria, interrupciones y aislamiento. Esta unidad precede a la Unidad 01 (syscalls), pero no impide realizar experimentos de kernels en paralelo.

## Conceptos esenciales

| Concepto | Pregunta de estudio |
| --- | --- |
| ISA | ¿Qué instrucciones y registros promete una arquitectura? |
| Microarquitectura | ¿Cómo implementa físicamente una CPU esa ISA? |
| ABI | ¿Cómo acuerdan programas y sistema el paso de argumentos y resultados? |
| Registros y pila | ¿Dónde viven los valores y cómo se llaman las funciones? |
| Privilegios | ¿Por qué una aplicación no puede ejecutar cualquier instrucción? |
| Interrupciones y excepciones | ¿Qué causa una transición asíncrona o síncrona al kernel? |
| MMU y TLB | ¿Cómo se traduce y protege la memoria virtual? |
| Memoria multicore | ¿Qué implica la concurrencia para la visibilidad de los datos? |

## Orden y alcance

1. **x86-64 — principal:** instrucciones, registros, ABI, privilegios y entrada a syscalls en Linux.
2. **RISC-V — secundario:** comparar ISA abierta, modos de privilegio y traps mediante QEMU.
3. **ARM64 — posterior:** comparar registros, excepciones, EL0/EL1 y ABI.

No es necesario estudiar diseño de circuitos, pipelines y ejecución especulativa antes de continuar con los sistemas operativos; estos temas se retoman cuando sean útiles.

## LAB-00A: C a Assembly

En Linux x86-64 con GCC o Clang:

```c
// sumar.c
int sumar(int a, int b) {
    return a + b;
}

int main(void) {
    return sumar(2, 3) == 5 ? 0 : 1;
}
```

```bash
uname -m
cc --version
cc -O0 -S -fno-omit-frame-pointer sumar.c -o sumar.s
cc -O0 -g -fno-omit-frame-pointer sumar.c -o sumar
objdump -d -M intel ./sumar | less
./sumar
echo $?
```

Explorar la función `sumar`, la convención de registros del ABI System V AMD64 y el papel de `call`, `ret`, `rsp` y `rbp`. La forma concreta del Assembly depende del compilador y sus opciones. Comparar después con `-O2` y observar posibles optimizaciones.

## LAB-00B: registros en el depurador

```bash
gdb ./sumar
```

En GDB: `break sumar`, `run`, `info registers`, `disassemble`, `stepi`. Anotar qué registros cambian y cómo se representa la pila.

## Puente a Unidad 01

Comparar una llamada ordinaria entre funciones con una syscall a Linux. **No son lo mismo:** el mecanismo de invocación de una función común no necesariamente cambia de privilegio, mientras que una syscall entra por una interfaz controlada del kernel.

## Evidencia de aprendizaje

- [ ] Explicar ISA, ABI y microarquitectura sin confundirlos.
- [ ] Documentar arquitectura, compilador y comandos.
- [ ] Identificar argumentos, retorno, pila y registros en el programa compilado.
- [ ] Diferenciar protección ofrecida por CPU frente a política aplicada por kernel.
- [ ] Publicar observaciones y preguntas abiertas.

## Fuentes primarias

- [System V AMD64 ABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
- [Intel 64 and IA-32 Software Developer Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [RISC-V Specifications](https://riscv.org/specifications/)
- [Arm Architecture Reference Manuals](https://developer.arm.com/Architectures)
- [Linux kernel documentation](https://docs.kernel.org/)
