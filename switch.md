# The switch statement in bytecode

The Java `switch` statement can be translated in either `lookupswitch` or `tableswitch` bytecode instructions - or both.

The `Person.printLastNameInfo()` method illustrates this:

- The input string is converted into a hash code
- It is first processed by the `lookupswitch` instruction. This instruction contains a code -> jump address map
- The `lookupswitch` then routes the call where an actual string comparison is performed. If the comparison clears, a number is stored in local variable 2 (0, 1, 2 or -1)
- A `tableswitch` instruction is then called. This instruction contains a table of jump addresses. It reads the value set in variable 2 (pushed to the stack by calling `iload_2` right before `tableswitch`)
- Based on the value, the instruction then reroutes the execution to the actual action to perform

```
  public void printLastNameInfo();
    descriptor: ()V
    flags: ACC_PUBLIC
    Code:
      stack=2, locals=3, args_size=1
         0: aload_0
         1: getfield      #3                  // Field lastName:Ljava/lang/String;
         4: astore_1
         5: iconst_m1
         6: istore_2
         7: aload_1
         8: invokevirtual #21                 // Method java/lang/String.hashCode:()I
        11: lookupswitch  { // 3
                   68890: 58
                68574440: 72
                80004067: 44
                 default: 83
            }
        44: aload_1
        45: ldc           #22                 // String Smith
        47: invokevirtual #23                 // Method java/lang/String.equals:(Ljava/lang/Object;)Z
        50: ifeq          83
        53: iconst_0
        54: istore_2
        55: goto          83
        58: aload_1
        59: ldc           #24                 // String Doe
        61: invokevirtual #23                 // Method java/lang/String.equals:(Ljava/lang/Object;)Z
        64: ifeq          83
        67: iconst_1
        68: istore_2
        69: goto          83
        72: aload_1
        73: ldc           #25                 // String Gates
        75: invokevirtual #23                 // Method java/lang/String.equals:(Ljava/lang/Object;)Z
        78: ifeq          83
        81: iconst_2
        82: istore_2
        83: iload_2
        84: tableswitch   { // 0 to 2
                       0: 112
                       1: 123
                       2: 134
                 default: 145
            }
       112: getstatic     #9                  // Field java/lang/System.out:Ljava/io/PrintStream;
       115: ldc           #26                 // String Will or Kevin?
       117: invokevirtual #11                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
       120: goto          153
       123: getstatic     #9                  // Field java/lang/System.out:Ljava/io/PrintStream;
       126: ldc           #27                 // String Joe or Jane?
       128: invokevirtual #11                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
       131: goto          153
       134: getstatic     #9                  // Field java/lang/System.out:Ljava/io/PrintStream;
       137: ldc           #28                 // String Bill??
       139: invokevirtual #11                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
       142: goto          153
       145: getstatic     #9                  // Field java/lang/System.out:Ljava/io/PrintStream;
       148: ldc           #29                 // String Non-common name
       150: invokevirtual #11                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
       153: return
```

## lookupswitch encoding

The `lookupswitch` from the code above is encoded as follows:

```
ab              lookupswitch opcode
                (no padding needed)
00 00 00 48     default: jump to offset +0x48 = +72 = 11+72 = 83
00 00 00 03     There are three entries
00 01 0d 1a     If the value is 0x00010d1a = 68890...
00 00 00 2f     ...then jump to offset +0x2f = +47 = 11+47 = 58
04 16 5c e8     If the value is 0x04165ce8 = 68574440...
00 00 00 3d     ...then jump to offset +0x3d = +61 = 11+61 = 72
04 c4 c3 e3     If the value is 0x04c4c3e3 = 80004067...
00 00 00 21     ...then jump to offset +0x21 = +33 = 11+33 = 44
```

When executed, the `lookupswitch` instruction pops up the value from the stack (it must be an integer), compares it to each entry and jumps to the corresponding offset if there is a match. If none fits then it jumps to the default value which is 83.

## tableswitch encoding

The `tableswitch` from the code above is encoded as follows:

```
aa              tableswitch opcode
00 00 00        padding
00 00 00 3d     default: jump to offset +0x3d = 61 = 84 + 61 = 145
00 00 00 00     low=0
00 00 00 02     high=2
00 00 00 1c     Jump to offset +0x1c = +28 = 84+28 = 112
00 00 00 27     Jump to offset +0x27 = +39 = 84+39 = 123
00 00 00 32     Jump to offset +0x32 = +50 = 84+50 = 134
```

This tells us that the table has `high-low+1=3` entries, indexed from 0 (`low`) to 2 (`high`). When executed, the instruction pops up the input value from the stack (it must be an integer). If it is 0, 1 or 2 the code respectively jumps to offset 112, 123 or 134. Otherwise it jumps to the default value which is 145.
