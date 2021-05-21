# Java Virtual Machine Overview

The `javac` Java compiler compiles Java/Scala/Kotlin code into bytecode, a pseudo-assembly which is then executed by the JVM (typically the `java` executable). You can find the list of all the bytecode instructions [here](https://en.wikipedia.org/wiki/Java_bytecode_instruction_listings).

Let's see how the following Java code is compiled:

```
public static void main(String[] args) {
  int a = Integer.parseInt(args[0]);
  int b;

  if ((a % 2) == 0) {
    b = a;
  } else {
    b = a - 1;
  }

  b /= 2;

  System.out.println(String.format("Convert %d into %d", a, b));
}
```

The Bytecode equivalent is:

```
aload0
iconst_0
aaload
invokestatic java/lang/Integer parseInt (Ljava/lang/String;)I
istore_1
iload_1
iconst_2
irem
ifne 18
iload_1
istore_2
goto 22
iload_1
iconst_1
isub
istore_2
iload_2
iconst_2
idiv
istore_2
getstatic java/lang/System out Ljava/io/PrintStream;
ldc "Convert %d into %d"
iconst_2
anewarray java/lang/Object
dup
iconst_0
iload_1
invokestatic java/lang/Integer valueOf (I)Ljava/lang/Integer;
aastore
dup
iconst_1
iload_2
invokestatic java/lang/Integer valueOf (I)Ljava/lang/Integer;
aastore
invokestatic java/lang/String format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
invokevirtual java/io/PrintStream println (Ljava/lang/String;)V
return
```

Most of these instructions have no argument (e.g. `iconst_0`, `aastore`, `dup`) and either look at the stack or JVM variables (the JVM has a few variables used to store temporary data) for arguments.

The instructions which take an argument generally only take a few bytes which typically reference a constant, e.g. a string or a method description. The instruction arguments are displayed here in a human-readable version (`ldc "Convert %d into %d"` instead of `ldc 0x4`, `0x4` represending the constant index which contains the string).
