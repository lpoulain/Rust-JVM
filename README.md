# JVM
A far-from-complete Java Virtual Machine made in Rust

## What is supported

At this point it is a very primitive JVM. The `Hello.java` and `Arithm.java` show what is currently being supported.

## How to use it

The main use is to run `jvm <class name>`. It is also possible to pass various debugging level (1 to 3) to provide more information about the bytecode both defined in the .class file and the bytecode instructions actually executed.

```
> ./target/debug/jvm Arithm 43
Convert 43 into 21

> ./target/debug/jvm -d 1 Hello
Executing method main
Execute 0       getstatic java/lang/System out Ljava/io/PrintStream;
Execute 1       ldc "Hello, World!"
Execute 2       invokevirtual java/io/PrintStream println (Ljava/lang/String;)V
Hello, World!
Execute 3       return
```