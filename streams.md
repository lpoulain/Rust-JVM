# Java Streams Overview

Maybe because the JVM was never designed with functional programming (functions are not a first class citizen in the Java world), the bytecode needs to jump through hoops to use lambda functions.

Consider the following code:

```java
public static void main(String[] args) {
    List<String> strings = Arrays.asList(args);
    
    strings.stream()
        .filter(s -> s.startsWith("A"))
        .map(s -> s.toLowerCase())
        .forEach(s -> System.out.println(s));
}
```

Its gets compiled in the following:

```
aload0
invokestatic 0x2                      [Arrays.asList()]
astore_1
aload1
invokeinterface 0x3 0x1               [List.stream()]
invokedynamic 0x4                     [LambdaMetafactory.metafactory()]
invokeinterface 0x5 0x2               [Stream.filter()]
invokedynamic 0x6                     [LambdaMetafactory.metafactory()]
invokeinterface 0x7 0x2               [Stream.map()]
invokedynamic 0x8                     [LambdaMetafactory.metafactory()]
invokeinterface 0x9 0x2               [Stream.forEach()]
```

The `invokedynamic` instruction has only one argument (say `0x4`). The JVM needs to do the following to decode it:

- Get the `CONSTANT_InvokeDynamic` constant at index 0x4. It contains a bootstrap method index (0x0) as well as a name/type index (name is `test` and type is `()Ljava/util/function/Predicate;`)
- To get the bootstrap method, look for an attribute of type `BootstrapMethods` at the end of the class bytecode. This class attribute will contain three bootstrap methods in the current case
- Each bootstrap method contains a method reference index (all pointing to `java.lang.invoke.LambdaMetafactory.metafactory()`) and three arguments
- The second argument of the bootstrap method points to a `CONSTANT_MethodHandle` which points to a method defined in the class (`lambda$main$0()`). This function contains the bytecode for the `s -> s.startsWith("A")` lambda function. `lambda$main$1()` and `lambda$main$2()` are also defined for the other two lambdas.

The execution of the code works as follows:

| Instruction | Description | Stack |
| --- | --- | --- |
| aload0 | Push the content of variable 0 (set to the `args` array passed to the function) to the stack | `["Ann","Bob"]` |
| invokestatic 0x2 | Calls `Arrays.asList()` which pops the array from the stack and transforms it into a `List` instance | `<List>` |
| astore_1 | Pops the array reference from the stack to store it in the JVM value 1 | |
| aload1 | Pushes the value of JVM variable 1 onto the stack | `<List>` |
| invokeinterface 0x3 0x1 | Calls `List.stream()`, which pops the list from the stack and pushes a `Stream` object whose source is this list | `<Stream>` |
| invokedynamic 0x4 | Calls `LambdaMetafactory.metafactory()` with 6 arguments (the current class, `"test"`, the method description and the three bootstrap method arguments). After processing, a `Predicate` instance which points to the `s -> s.startsWith("A")` lambda function is pushed to the stack | `<Stream>`<br>`<Predicate>` |
| invokeinterface 0x5 0x2 | Calls `Stream.filter()`, which pops the two arguments from the stack and adds the predicate to the stream instance which is pushed back to the stack | `<Stream>` |
| invokedynamic 0x6 | Calls `LambdaMetafactory.metafactory()` with 6 arguments (the current class, `"apply"`, the method description and the three bootstrap method arguments). After processing, a `Function` instance which points to the `s -> s.toLowerCase()` lambda function is pushed to the stack | `<Stream>`<br>`<Function>` |
| invokeinterface 0x7 0x2 | Calls `Stream.map()`, which pops the two arguments from the stack and adds the function to the stream instance which is pushed back to the stack | `<Stream>` |
| invokedynamic 0x8 | Calls `LambdaMetafactory.metafactory()` with 6 arguments (the current class, `"accept"`, the method description and the three bootstrap method arguments). After processing, a `Consumer` instance which points to the `s -> System.out.println(s)` lambda function is pushed to the stack | `<Stream>`<br>`<Consumer>` |
| invokeinterface 0x9 0x2 | Calls `Stream.forEach()` which pops the two arguments from the stack. Because this method is acting on a Consumer, the stream execution starts. Strings from the original list gets read, goes through the `Predicate`. If successful they are sent to the `Function` where they are transformed. Last but not least they are sent to the `Consumer` which prints them on the screen | |
