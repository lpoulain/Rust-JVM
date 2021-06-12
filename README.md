# Rust JVM
A far-from-complete Java Virtual Machine written in Rust

## What is supported

At this point it is a very primitive JVM. The files in the `java` folder (`Hello.java`, `Arithm.java`, `Streams.java`, `Person.java` and `Mandelbrot.java`) show what is currently being supported.

See also the following to understand more about Java/Scala/Kotlin code gets converted into bytecode instructions:

- [Bytecode overview](bytecode.md)
- [Java Streams overview](streams.md)
- [The Java switch statement](switch.md)

## How to use it

The main use is to run `jvm <class name>` (the corresponding .class file must be in the current directory or in the `Java` directory). It is also possible to pass a debugging level (from 1 to 3) to get more information about the bytecode both defined in the .class file and the bytecode instructions actually executed.

```
> cargo build
   Compiling jvm v0.1.0 (/Users/user/JVM)
    Finished dev [unoptimized + debuginfo] target(s) in 1.69s

> cargo build --release
   Compiling jvm v0.1.0 (/Users/user/JVM)
    Finished dev [optimized] target(s) in 5.32s

> ./target/debug/jvm Arithm 43
Convert 43 into 21

> ./target/debug/jvm -d 1 Hello
Executing method main
Execute 0       getstatic java/lang/System out Ljava/io/PrintStream;
Execute 1       ldc "Hello, World!"
Execute 2       invokevirtual java/io/PrintStream.println(Ljava/lang/String;)V
Hello, World!
Execute 3       return

> ./target/debug/jvm Person
John Smith
Can legally drink (in the U.S.)
Will or Kevin?

> ./target/debug/jvm Streams Alice Bob Charlie Anna Delta
alice
anna

> ./target/release/jvm Mandelbrot
:::::::::::::::::::::::=======================***************************************===================================
::::::::::::::::::::::==================***************************************************=============================
::::::::::::::::::::===============***********************************........--.......*********========================
:::::::::::::::::::=============*********************************.........---X------.......********=====================
:::::::::::::::::============********************************...........------XX------........********==================
::::::::::::::::==========********************************............----------X--XXX--.........********===============
:::::::::::::::========********************************..............------------X--------.........*********============
::::::::::::::=======********************************..............-----------XXXX-X--------........**********==========
:::::::::::::======*******************************...............--------XXXX-XXXXXX-XX--------.......**********========
::::::::::::=====*******************************..............--------------XXXXXXXXXXX-----------.....***********======
:::::::::::=====******************************.............---------------XXXXXXXXXXXX--------------....***********=====
:::::::::::===*****************************............---------X-----------XXXXXXXXX--------------X--....***********===
::::::::::===****************************..........-----------XX-----XXX-XXXXXXXXXXXXXXXX-XX-------X----...***********==
:::::::::===**************************........-----------------XXXX-XXXXXXXXXXXXXXXXXXXXXXXXX---X-X-XX---..************=
:::::::::==*************************.....---------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------..************
::::::::==**********************....-----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------...***********
::::::::=*******************....-------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------...**********
:::::::=**************.......------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---X---...**********
:::::::=********..........--------XX--------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---....*********
::::::=******...........-------------XXXX----XX-X-------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----....*********
::::::=***............----------------XXXXXXXXXXXXX-----XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::**............--------------X--XXXXXXXXXXXXXXXX---XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX----.....********
::::::*...........----------XX-----XXXXXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::..........--------------X-X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------......*******
::::::----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------......*******
::::::XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX----------......*******
::::::----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------......*******
::::::..........--------------X-X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------......*******
::::::*...........----------XX-----XXXXXXXXXXXXXXXXXXX-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::**............--------------X--XXXXXXXXXXXXXXXX---XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX----.....********
::::::=***............----------------XXXXXXXXXXXXX-----XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----.....********
::::::=******...........-------------XXXX----XX-X-------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-----....*********
:::::::=********..........--------XX--------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---....*********
:::::::=**************.......------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX---X---...**********
::::::::=*******************....-------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX--------...**********
::::::::==**********************....-----------------------X--XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------...***********
:::::::::==*************************.....---------------------XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX------..************
:::::::::===**************************........-----------------XXXX-XXXXXXXXXXXXXXXXXXXXXXXXX---X-X-XX---..************=
::::::::::===****************************..........-----------XX-----XXX-XXXXXXXXXXXXXXXX-XX-------X----...***********==
:::::::::::===*****************************............---------X-----------XXXXXXXXX--------------X--....***********===
:::::::::::=====******************************.............---------------XXXXXXXXXXXX--------------....***********=====
::::::::::::=====*******************************..............--------------XXXXXXXXXXX-----------.....***********======
:::::::::::::======*******************************...............--------XXXX-XXXXXX-XX--------.......**********========
::::::::::::::=======********************************..............-----------XXXX-X--------........**********==========
:::::::::::::::========********************************..............------------X--------.........*********============
::::::::::::::::==========********************************............----------X--XXX--.........********===============
:::::::::::::::::============********************************...........------XX------........********==================
:::::::::::::::::::=============*********************************.........---X------.......********=====================
::::::::::::::::::::===============***********************************........--.......*********========================
::::::::::::::::::::::==================***************************************************=============================
:::::::::::::::::::::::=======================***************************************===================================
```
