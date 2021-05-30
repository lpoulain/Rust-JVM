class Mandelbrot {
    public static void main(String[] args) {
        for (int j=0; j<51; j++) {
            for (int i=0; i<120; i++) {
                float c_x = (float)(i-85) / 40.0f;
                float c_y = (float)(j-25) / 20.0f;

                int iter = 0;
                float x = 0.0f;
                float y = 0.0f;
                float dx = 1.0f;
                float dy = 0.0f;
                while (x*x + y*y < 4.0f && iter < 25) {
                    float old_x = x, old_dx = dx;
                    x = x*x - y*y + c_x;
                    dx = 2.0f * (old_x * dx - y * dy) + 1;
                    dy = 2.0f * (old_dx * y + dy * old_x);
                    y = 2.0f * y * old_x + c_y;
                    iter += 1;
                }

                // For a more accurate representation of the set, we compute the distance
                // to the Mandelbrot set
                float distance = (float)Math.sqrt(x*x+y*y) * ((float)Math.log(Math.sqrt(x*x+y*y))) / (float)Math.sqrt(dx*dx+dy*dy);

                String output = ".";
                if (iter == 1) { output = ":"; }
                else if (iter == 2) { output = "="; }
                else if (iter == 3) { output = "*"; }
                else if (iter >= 5) { output = "-"; }
                if (distance <= 0.004f || iter >= 25) { output = "X"; }
                System.out.print(output);
            }
            System.out.println("");
        }
    }
}
