
public class Percolation {
    private int N;

    private int numSites;

    private boolean[] sites;

    private WeightedQuickUnionUF ufWithTopBottom;
    private WeightedQuickUnionUF ufWithTopOnly;

    public Percolation(int N) {
        if (N <= 0)
            throw new IllegalArgumentException("N must be larger than 0");

        this.N = N;

        numSites = N*N;

        sites = new boolean[numSites];
        ufWithTopBottom = new WeightedQuickUnionUF(numSites+2);
        ufWithTopOnly = new WeightedQuickUnionUF(numSites + 1);

    }


    private int index(int i, int j) {
        return (i-1) * N + j-1;
    }

    private int topIndex() {
        return numSites;
    }

    private int bottomIndex() {
        return numSites + 1;
    }

    public void open(int i, int j) {     // open site (row i, column j) if it is not open already
        if (i < 1 || i > N || j < 1 || j > N)
            throw new IndexOutOfBoundsException("i and j need be between 1 and " + this.N);

        int index = index(i, j);
        sites[index] = true;

        //left, i,j-1;
        if (j>1) {
            int lindex = index(i,j-1);
            if (sites[lindex]) {
                ufWithTopBottom.union(index, lindex);
                ufWithTopOnly.union(index, lindex);
            }
        }

        //right i, j+ 1;
        if (j < N) {
            int rindex = index(i, j+ 1);
            if (sites[rindex]) {
                ufWithTopBottom.union(index, rindex);
                ufWithTopOnly.union(index, rindex);
            }
        }

        //top i-1, j
        if (i > 1) {
            int tindex = index(i-1, j);
            if (sites[tindex]) {
                ufWithTopBottom.union(index, tindex);
                ufWithTopOnly.union(index, tindex);
            }
        }

        //bottom i+1, j

        if (i < N) {
            int bindex = index(i+1, j);
            if (sites[bindex]) {
                ufWithTopBottom.union(index, bindex);
                ufWithTopOnly.union(index, bindex);
            }
        }

        if (i == 1) {
            ufWithTopBottom.union(index, topIndex());
            ufWithTopOnly.union(index, topIndex());
        }

        if (i == N)
            ufWithTopBottom.union(index, bottomIndex());
    }

    public boolean isOpen(int i, int j) {  // is site (row i, column j) open?
        if (i < 1 || i > N || j < 1 || j > N)
            throw new IndexOutOfBoundsException("i and j need be between 1 and " + this.N);

        return sites[index(i,j)];
    }

    public boolean isFull(int i, int j) {  // is site (row i, column j) full?
        if (i < 1 || i > N || j < 1 || j > N)
            throw new IndexOutOfBoundsException("i and j need be between 1 and " + this.N);

        int site = index(i, j);

        return ufWithTopOnly.connected(site, topIndex());
    }

    public boolean percolates() {        // does the system percolate?
        return ufWithTopBottom.connected(topIndex(), bottomIndex());
    }

    public static void main(String[] args) { // test client (optional)
    }
}
