class Arithm {
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
}
