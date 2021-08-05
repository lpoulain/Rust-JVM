# Converting Bytecode into Assembly

`jvm` has an optional `--asm linux` or `--asm macos` flag which, instead of executing the class, attempts to convert it into x64 assembly for respectively Linux or macOS (Intel notation, not AT&T). If successful, a `<class>.asm` file gets created which can be compiled by [nasm](https://www.nasm.us/). The current limitations are:

- The class `main()` method only gets converted
- Only a limited number of methods that can called inside `main()` are limited (`System.out.print()`, `System.out.println()`, `Math.sqrt()`, `Math.log()`)

```
> ./target/debug/jvm --asm macos Mandelbrot
Bytecode class compiled into x64 assembly, Intel notation (Mandelbrot.asm)
> head Mandelbrot.asm 
    global    start
    section   .text
start:
    push 0x0
    pop r8
__branch2:
    push r8
    push 0x33
    pop rbx
    pop rax
```

To build it on macOS:

```
> nasm -f macho64 Mandelbrot.asm 
> ld -macosx_version_min 10.7.0 -o Mandelbrot Mandelbrot.o
```

To build it on Linux:

```
> nasm -f elf64 Mandelbrot.asm
> ld -o Mandelbrot Mandelbrot.o
```

To execute it:

```
> ./Mandelbrot 
:::::::::::::::::::::::=======================***************************************===================================
::::::::::::::::::::::==================***************************************************=============================
::::::::::::::::::::===============***********************************........--.......*********========================
:::::::::::::::::::=============*********************************.........---X------.......********=====================
:::::::::::::::::============********************************...........------XX------........********==================
::::::::::::::::==========********************************............-------------XXX--.........********===============
:::::::::::::::========********************************..............------------X--------.........*********============
::::::::::::::=======********************************..............-----------XXXX----------........**********==========
:::::::::::::======*******************************...............--------X-X--X-XX-X-XX--------.......**********========
::::::::::::=====*******************************..............--------------XXXXXXXXXX------------.....***********======
:::::::::::=====******************************.............---------------XXXXXXXXXXXX--------------....***********=====
:::::::::::===*****************************............---------X-----------XXXXXXXXX--------------X--....***********===
::::::::::===****************************..........-----------XX-----XXX-XXXXXXXXXXXXXXXX-XX-------X----...***********==
:::::::::===**************************........-----------------XXXX-XXXXXXXXXXXXXXXXXXXXXXXXX---X-X-XX---..************=
:::::::::==*************************.....---------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-XXXXX------..************
::::::::==**********************....-----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------...***********
::::::::=*******************....-------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------...**********
:::::::=**************.......------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---X---...**********
:::::::=********..........--------XX--------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---....*********
::::::=******...........-------------XXXX----X----------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----....*********
::::::=***............----------------XXXXXXXXXXXXX-----XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::**............--------------X--XXXXXXXXXXXXXXXX---XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX----.....********
::::::*...........----------XX-----XXXXXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::..........----------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------......*******
::::::----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------......*******
::::::XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----------......*******
::::::----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------......*******
::::::..........----------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------......*******
::::::*...........----------XX-----XXXXXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::**............--------------X--XXXXXXXXXXXXXXXX---XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX----.....********
::::::=***............----------------XXXXXXXXXXXXX-----XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::=******...........-------------XXXX----X----------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----....*********
:::::::=********..........--------XX--------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---....*********
:::::::=**************.......------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---X---...**********
::::::::=*******************....-------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------...**********
::::::::==**********************....-----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------...***********
:::::::::==*************************.....---------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-XXXXX------..************
:::::::::===**************************........-----------------XXXX-XXXXXXXXXXXXXXXXXXXXXXXXX---X-X-XX---..************=
::::::::::===****************************..........-----------XX-----XXX-XXXXXXXXXXXXXXXX-XX-------X----...***********==
:::::::::::===*****************************............---------X-----------XXXXXXXXX--------------X--....***********===
:::::::::::=====******************************.............---------------XXXXXXXXXXXX--------------....***********=====
::::::::::::=====*******************************..............--------------XXXXXXXXXX------------.....***********======
:::::::::::::======*******************************...............--------X-X--X-XX-X-XX--------.......**********========
::::::::::::::=======********************************..............-----------XXXX----------........**********==========
:::::::::::::::========********************************..............------------X--------.........*********============
::::::::::::::::==========********************************............-------------XXX--.........********===============
:::::::::::::::::============********************************...........------XX------........********==================
:::::::::::::::::::=============*********************************.........---X------.......********=====================
::::::::::::::::::::===============***********************************........--.......*********========================
::::::::::::::::::::::==================***************************************************=============================
:::::::::::::::::::::::=======================***************************************===================================
```

## Performance results

Here are the times it takes to compute `Mandelbrot.class`:

- Rust JVM: 0.731s
- Native JVM: 0.158s
- Assembly code generated by the Rust JVM: 0.012s

## How does the conversion work

The Bytecode instructions perform simple instructions and get/store data from/in either the JVM stack or stack frame variables. These are converted into similar assembly instrctions which store data in respectively the assembly stack and the CPU registers (r8 to r15 for integer operations, xmm0 to xmm13 for float and double operations)

Note that `Math.log()` is an approximation and not an actual logarithmic implementation. This is because x64's only log instruction (`fyl2x`) is an x87 floating point instruction. The more modern SSE floating point instruction set (which is what is used by the Rust JVM assembly converter) does not support logarithmic computation. And I was not able to find a way to convert data between the x87 and the SSE registers.
