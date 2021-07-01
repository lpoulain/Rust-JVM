class Exceptions {
    public static int operation(int a, int b) throws Throwable {
        if (b == 1) {
            throw new RuntimeException("Division by 1! Really? (exception caught)");
        }

        if (b == 2) {
            throw new Exception("Keep trying (exception caught)");
        }

        if (b == 3) {
            throw new Throwable("C'mon! You can do better than that! (exception not caught)");
        }

        return a / b;
    }

    public static void main(String[] args) throws Throwable {
        int a = Integer.parseInt(args[0]);
        int b = Integer.parseInt(args[1]);

        int c;

        operation(b, a);

        try {
            c = operation(a, b);
            System.out.println(String.format("Result is %d", c));
        } catch (Exception e) {
            System.out.println("Exception caught with the following stack trace:");
            e.printStackTrace(System.out);
        }
    }
}
