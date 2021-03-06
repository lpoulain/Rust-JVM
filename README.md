# Rust JVM
A far-from-complete Java Virtual Machine written in Rust

## What is supported

This JVM implements most bytecode instructions. Their implementation may however not be complete as it has been tested with a limited number of classes. Moreover, the JVM's native class support stays limited. The files in the `java` folder show what is currently being supported:

- `Hello.java`: basic Hello World
- `Arithm.java`: simple integer arithmetic
- `Mandelbrot.java` floating-point calculations
- `Streams.java`: a Java streams example using `map()`, `filter()` and `forEach()`
- `Person.java`, `AgeRange`: class, `switch` statements and enums
- `TestThread.java`, `RunnableDemo.java`: multi-threading
- `Exceptions.java`: exception handling

See also the following to understand more about Java/Scala/Kotlin code gets converted into bytecode instructions:

- [Bytecode overview](bytecode.md)
- [Java Streams overview](streams.md)
- [The Java switch statement](switch.md)
- [Java enums](enums.md)
- [Compiling bytecode into assembly](assembly.md)

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
Young Adult
Will or Kevin?

> ./target/debug/jvm Streams Alice Bob Charlie Anna Delta
alice
anna

> ./target/debug/jvm TestThread
Creating Thread-1
Starting Thread-1
Creating Thread-2
Running Thread-1
Starting Thread-2
Thread: Thread-1, 4
Running Thread-2
Thread: Thread-2, 4
Thread: Thread-1, 3
Thread: Thread-2, 3
Thread: Thread-1, 2
Thread: Thread-2, 2
Thread: Thread-1, 1
Thread: Thread-2, 1
Thread Thread-1 exiting.
Thread Thread-2 exiting.

> ./target/debug/jvm Exceptions 123 0
Exception caught with the following stack trace:
Exception in java/lang/ArithmeticException: / by zero
        at Exceptions.operation(Exceptions.java:15)
        at Exceptions.main(Exceptions.java:27)

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
