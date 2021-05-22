# Bytecode Overview

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
Most of these instructions have no argument (e.g. `iconst_0`, `aastore`, `dup`) and either look at the stack or JVM variables for arguments (the JVM has a few variables used to store temporary data).

Any instruction argument(s) only take a few bytes as they reference constants defined in the `.class` file, e.g. a string or a method description. The instruction arguments are displayed here in a human-readable version (`ldc "Convert %d into %d"` instead of `ldc 0x4`, `0x4` representing the constant index which contains the string).

Here is how is the bytecode executed when calling `jvm Arithm 43`

| Instruction | Description | Stack |
| --- | --- | --- |
| aload0 | Push the content of variable 0 (set to the `args` array passed to the function) to the stack | ``["43"]`` |
| iconst_0 | Push integer 0 to the stack | ``["43"]``<br>`0` |
| aaload | Pop integer `i` and array `a` from the stack and push `a[i]` to the stack | `"43"` |
| invokestatic java/lang/Integer parseInt (Ljava/lang/String;)I | Pop the value from the stack and call static method `Integer.parseInt()` over it. Push the result to the stack | `43` |
| istore_1 | Pop the value from the stack and store it in variable 1. From now on, variable 1 = `43` | |
| iload_1 | Push variable 1 (`43`) to the stack | `43` |
| iconst_2 | Push integer 2 to the stack | `43`<br>`2` |
| irem | Pop the integers `j` and `i` from the stack and push `i % j` (remainder) to the stack | `1` |
| ifne 18 | Pop the last value from the stack. Because it is not 0, jump to offset 18 | |
| iload_1 | Skipped | |
| istore_2 | Skipped | |
| goto 22 | Skipped | |
| iload_1 | Push variable 1 (`43`) to the stack | `43` |
| iconst_1 | Push integer 1 to the stack | `43`<br>`1` |
| isub | Pop the last two values from the stack, substract them (`43 - 1`) and push the result to the stack | `42` |
| istore_2 | Pop the value from the stack, and store it in variable 2 | |
| iload_2 | Push the content of variable 2 to the stack | `42` |
| iconst_2 | Push integer 2 to the stack | `42`<br>`2` |
| idiv | Pop the last two values from the stack, divide them (`42/2`) and push the result to the stack | `21` |
| istore_2 | Pop the result from the stack and store it in variable 2. From now on, variable 2 = `21` | |
| getstatic java/lang/System out Ljava/io/PrintStream; | Call static method `System.out`. This pushes to the stack a `PrintStream` instance used to print data to the standard output | `<PrintStream>` |
| ldc "Convert %d into %d" | Push the string `Convert %d into %d` onto the stack | `<PrintStream>`<br>`"Convert %d into %d"` |
| iconst_2 | Push integer 2 onto the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`2` |
| anewarray java/lang/Object | Pop the last value `i`, create a new array of `java.lang.Object` instances of size `i` and push it to the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[null, null]` |
| dup | Duplicate the last value in the stack (both point to the same array) | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[null, null]`<br>`[null, null]` |
| iconst_0 | Push integer 0 to the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[null, null]`<br>`[null, null]`<br>`0` |
| iload_1 | Push the content of variable 1 to the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[null, null]`<br>`[null, null]`<br>`0`<br>`43` |
| invokestatic java/lang/Integer valueOf (I)Ljava/lang/Integer; | Pop the latest value from the stack, call static method `Integer.valueOf()` over that value. Push the result to the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[null, null]`<br>`[null, null]`<br>`0`<br>`43` |
| aastore | Pop value `v`, integer `i` and array `a` from the stack, and set `a[i]` to `v` | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[43, null]` |
| dup | Duplicate the last value in the stack (both point to the same array) | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[43, null]`<br>`[43, null]` |
| iconst_1 | Push integer 1 onto the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[43, null]`<br>`[43, null]`<br>`1` |
| iload_2 | Push the content of value 2 onto the stack | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[43, null]`<br>`[43, null]`<br>`1`<br>`21` |
| invokestatic java/lang/Integer valueOf (I)Ljava/lang/Integer; | Pop the latest value from the stack, call static method `Integer.valueOf()` over that value. Push the result to the stack | `<PrintStream>`<br>`"Convert_%d_into_%d"`<br>`[43, null]`<br>`[43, null]`<br>`1`<br>`21` |
| aastore | Pop value `v`, integer `i` and array `a` from the stack, and set `a[i]` to `v` | `<PrintStream>`<br>`"Convert %d into %d"`<br>`[43, 21]` |
| invokestatic java/lang/String format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String; | Call static method `String.format()` which pops the last two values from the stack (the string to format and the associated array of arguments) and pushes the result to the stack | `<PrintStream>`<br>`"Convert 43 into 21"` |
| invokevirtual java/io/PrintStream println (Ljava/lang/String;)V | Call `PrintStream.println()` which pops the string and the `PrintStream` instance from the stack, and prints the string to the standard output | |
| return | End of the method | |
