import java.util.*;

class Streams {
    public static void main(String[] args) {
      List<String> strings = Arrays.asList(args);

      strings.stream()
        .filter(s -> s.startsWith("A"))
        .map(s -> s.toLowerCase())
        .forEach(s -> System.out.println(s));
    }
}
