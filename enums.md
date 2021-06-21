# Enums in Java

Enum implementation in Java needs to jump through a few hoops to work with the JVM. Consider the use of the `AgeRange` enum as used in `Person.java`:

```
enum AgeRange {
    AGE_0_9,
    AGE_10_19,
    AGE_20_39,
    AGE_40_64,
    AGE_65_79,
    AGE_80_99,
    AGE_100_PLUS
}
```

`AgeRange` is implemented as a class deriving from the `java.lang.Enum` class which is part of the JVM standard runtime. An `Enum` can be seen as a class with a name (of type `String`) which represents the default label (e.g. `AGE_10_19`) and an ordinal (of time `Int`) which represents the value index (from 0 to 6 in the present example).

However, when building `Person.java`, the Java compiler also creates a class `Person$1.class`. This class contains the following:

- A static member `int[] $SwitchMap$AgeRange`
- A static initializer which populates `$SwitchMap$AgeRange` to map each value used in `Person.java` to an index starting at 1. So if `Person.java` only use, say, `AGE_10_19`, `AGE_20_39` and `AGE_100_PLUS`, `$SwitchMap$AgeRange` would be initialized to `[0, 1, 2, 0, 0, 0, 3]`.

When the code in `Person.java` needs to look at the `Person` instance `ageRange` object:

```
switch (this.ageRange) {
    case AGE_10_19:
        System.out.println("Teen");
        break;
    case AGE_20_39:
        ...
```

the bytecode first calls `this.ageRange.ordinal()` which returns the enum's index, then get the mapping from the `Person$1.$SwitchMap$AgeRange` array which should be between 1 and 3. This allows a compact `tableswitch` instruction with only 3 entries.
