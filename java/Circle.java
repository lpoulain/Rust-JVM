class Circle {
    public static void main(String[] args) {
        for (int j=0; j<51; j++) {
            for (int i=0; i<120; i++) {
                int c_x = (i-60) / 2;
                int c_y = j-25;

                String output = ".";
                if (c_x*c_x + c_y*c_y < 256) {
                    output = "X";
                }
                System.out.print(output);
            }
            System.out.println("");
        }
    }
}
