import java.lang.IllegalArgumentException;
import java.util.Random;

public class PercolationStats {
    private int N;
    private int T;

    private double[] measurements;

    private double mean;
    private double stddev;
    private double confidenceLo;
    private double confidenceHi;

    public PercolationStats(int N, int T) {  // perform T independent experiments on an N-by-N grid
        if (N <= 0 || T <= 0)
            throw new IllegalArgumentException("N and T must be larger than 0");

        this.N = N;
        this.T = T;

        measurements = new double[T];

        action();
        calcStats();
    }

    private void action() {
        for (int i = 0; i < T; i++) {
            measurements[i] = run() * 1.0/(N*N);
        }
    }

    private int run() {
        Percolation p = new Percolation(N);

        int count = 0;
        int[] sites = new int[N*N];
        for (int i = 0; i < sites.length; i++)
            sites[i] = i;

        int available = sites.length;

        Random r = new Random();
        while (!p.percolates()) {

            //get site
            int n = r.nextInt(available);
            int site = sites[n];
            int i = site/N;
            int j = site % N;

            //open it
            p.open(i + 1, j + 1);

            //swap to tne end
            int temp = sites[n];
            sites[n] = sites[available-1];
            sites[available-1] = temp;

            available--;

            count++;
        }

        return count;
    }

    private void calcStats() {
        double sum = 0;
        for (int i = 0; i < T; i++)
            sum += measurements[i];

        mean =  sum/T;

        double delta = 0;
        for (int i = 0; i < T; i++)
            delta += (measurements[i] - mean) * (measurements[i] - mean);

        stddev = Math.sqrt(delta/(T-1));

        confidenceLo = mean - 1.96 * stddev/Math.sqrt(T);

        confidenceHi = mean + 1.96 * stddev/Math.sqrt(T);

    }

    public double mean() {                  // sample mean of percolation threshold
       return mean;
    }
    public double stddev() {             // sample standard deviation of percolation threshold
       return stddev;
    }

    public double confidenceLo() {           // low  endpoint of 95% confidence interval
        return confidenceLo;
    }

    public double confidenceHi() {          // high endpoint of 95% confidence interval
        return confidenceHi;
    }

    public static void main(String[] args) {  // test client (described below)
        if (args.length != 2)
            throw new IllegalArgumentException("args must be 2");

        int N = Integer.parseInt(args[0]);
        int T = Integer.parseInt(args[1]);

        PercolationStats ps = new PercolationStats(N, T);

        System.out.printf("mean                    = %f\n", ps.mean());
        System.out.printf("stddev                  = %f\n", ps.stddev());
        System.out.printf("95%% confidence interval = %f, %f\n" , ps.confidenceLo(), ps.confidenceHi());
    }
}
