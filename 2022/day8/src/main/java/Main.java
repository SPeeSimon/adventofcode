import java.io.IOException;
import java.lang.reflect.Array;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.BiFunction;
import java.util.function.Consumer;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {

    static class Grid<T> {
        Data<T>[][] grid;
        final int size;

        public Grid(int[][] heightGrid, T defaultValue) {
            this.size = heightGrid.length;
            grid = new Data[size][size];
            for (int x = 0; x < size; x++) {
                for (int y = 0; y < size; y++) {
                    grid[x][y] = new Data<>(heightGrid[x][y], defaultValue);
                }
            }
        }

        Data<T> point(int x, int y) {
            return grid[x][y];
        }

        T get(int x, int y) {
            return (T)grid[x][y].data;
        }

        int getHeight(int x, int y) {
            return grid[x][y].height;
        }

        void set(int x, int y, T value) {
            grid[x][y].data = value;
        }

        void setEdges(T value) {
            for (int i = 0; i < size; i++) {
                grid[0][i].data = value; // top
                grid[i][0].data = value; // left
                grid[size-1][i].data = value; // bottom
                grid[i][size-1].data = value; // right
            }
        }

        void all(Consumer<T> consumer) {
            for(int x =0; x < size; x++)
                for(int y =0; y < size; y++) {
                    consumer.accept(get(x, y));
                }
        }

        static class Data<T> {
            final int height;
            public T data;

            Data(int height, T data) {
                this.height = height;
                this.data = data;
            }
        }
    }

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day8/src/main/resources/input.txt"));
        // var input = """
        //     30373
        //     25512
        //     65332
        //     33549
        //     35390
        //         """.lines();
        
        int[][] treeGrid = input.map(line -> line.chars().map(c -> Character.digit(c, 10)).toArray())
                                .reduce(new int[0][], (v,r) -> add(v, r), (a,b) -> a);
        // boolean[][] visibleTreeGrid = calculateVisible(treeGrid);
        System.out.println("treeGrid: " + Arrays.deepToString(treeGrid));
        
        // int visibleTotal = countVisible(visibleTreeGrid);
        // System.out.println("treeGrid2: " + Arrays.deepToString(visibleTreeGrid));
        // System.out.println("Total visible: " + visibleTotal);
        // System.out.println("Not visible: " + ((visibleTreeGrid.length*visibleTreeGrid.length) - visibleTotal));

        calculate1(treeGrid);
        calculate2(treeGrid);
    }

    static void calculate1(int[][] treeGrid) {
        var visibleGrid = new Grid<>(treeGrid, false);
        int visibleTotal = calculateVisible(visibleGrid);
        System.out.println("Total visible: " + visibleTotal);
        System.out.println("Not visible: " + ((visibleGrid.size * visibleGrid.size) - visibleTotal));
    }

    static void calculate2(int[][] treeGrid) {
        var grid = new Grid<>(treeGrid, 0L);
        for(int x=0; x < grid.size; x++) {
            for (int y=0; y < grid.size; y++) {
                calculateScenicScore(grid, x, y);
            }
        }

        var total = new AtomicLong(0L);
        grid.all(v -> {
            if(v > total.get()){
                total.set(v);
            }
        });

        // System.out.println();
        // for(int x=0; x < grid.size; x++) {
        //     for (int y=0; y < grid.size; y++) {
        //         System.out.printf(" %3d ", grid.getHeight(x, y));
        //     }
        //     System.out.println();
        // }
        // System.out.println();
        // for(int x=0; x < grid.size; x++) {
        //     for (int y=0; y < grid.size; y++) {
        //         System.out.printf(" %3d ", grid.get(x, y));
        //     }
        //     System.out.println();
        // }

        System.out.println("\nHighest scenic score " + total.get());
    }

    private static void calculateScenicScore(Grid<Long> grid, int x, int y) {
        long left = 0;
        long right = 0;
        long top = 0;
        long bottom = 0;
        // from tree - go sideways - not blocked? -- advance

        for(int a = x - 1; a >= 0; a--) {
            left++;
            if (grid.getHeight(x, y) <= grid.getHeight(a, y)) {
                break;
            }
        }
        for(int a = x + 1; a < grid.size; a++) {
            right++;
            if (grid.getHeight(x, y) <= grid.getHeight(a, y)) {
                break;
            }
        }
        for(int a = y - 1; a >= 0; a--) {
            top++;
            if (grid.getHeight(x, y) <= grid.getHeight(x, a)) {
                break;
            }
        }
        for(int a = y + 1; a < grid.size; a++) {
            bottom++;
            if (grid.getHeight(x, y) <= grid.getHeight(x, a)) {
                break;
            }
        }

        grid.set(x, y, left * right *  top * bottom);
    }


    static int[][] add(int[][] array, int[] entry) {
        int[][] copy = Arrays.copyOf(array, array.length + 1);
        copy[array.length] = entry;
        return copy;
    }

    static int countVisible(boolean[][] grid) {
        int visibleTotal = 0;
        for (int x = 0; x < grid.length; x++) {
            for (int y = 0; y < grid[x].length; y++) {
                if (grid[x][y]) {
                    visibleTotal++;
                }
            }
        }
        return visibleTotal;
    }

    static int calculateVisible(Grid<Boolean> treeGrid) {

        treeGrid.setEdges(false);
        // LR
        for(int x=0; x < treeGrid.size; x++) {
            // LR
            int max = -1;
            for(int y=0; y < treeGrid.size; y++) {
                max = treeVisibility(treeGrid.point(x, y), max);
            }
            // RL
            max = -1;
            for(int y=treeGrid.size - 1; y > 0; y--) {
                max = treeVisibility(treeGrid.point(x, y), max);
            }
        }

        // TB
        for(int y=0; y < treeGrid.size; y++) {
            // LR
            int max = -1;
            for(int x=0; x < treeGrid.size; x++) {
                max = treeVisibility(treeGrid.point(x, y), max);
            }
            // RL
            max = -1;
            for(int x=treeGrid.size - 1; x > 0; x--) {
                max = treeVisibility(treeGrid.point(x, y), max);
            }
        }

        var total = new AtomicInteger(0);
        treeGrid.all(v -> {
            if(v){
                total.incrementAndGet();
            }
        });
        return total.get();
    }

    static int treeVisibility(Grid.Data<Boolean> tree, int max) {
        tree.data = tree.data || tree.height > max;
        return Math.max(max, tree.height);
    }


    static boolean[][] calculateVisible(int[][] treeGrid) {
        final int gridSize = treeGrid.length;
        boolean[][] visibleTreeGrid = new boolean[gridSize][gridSize];
        
        Arrays.stream(visibleTreeGrid).forEach(i -> {
            Arrays.fill(i, false);
            i[0] = true;
            i[i.length-1] = true;
        });
        Arrays.fill(visibleTreeGrid[0], true);
        Arrays.fill(visibleTreeGrid[visibleTreeGrid.length-1], true);

        // LR
        for(int x=0; x < gridSize; x++) {
            // LR
            int max = -1;
            for(int y=0; y < gridSize; y++) {
                visibleTreeGrid[x][y] |= treeGrid[x][y] > max;
                max = Math.max(max, treeGrid[x][y]);
            }
            // RL
            max = -1;
            for(int y=gridSize - 1; y > 0; y--) {
                visibleTreeGrid[x][y] |= treeGrid[x][y] > max;
                max = Math.max(max, treeGrid[x][y]);
            }
        }

        // TB
        for(int y=0; y < treeGrid.length; y++) {
            // LR
            int max = -1;
            for(int x=0; x < gridSize; x++) {
                visibleTreeGrid[x][y] |= treeGrid[x][y] > max;
                max = Math.max(max, treeGrid[x][y]);
            }
            // RL
            max = -1;
            for(int x=gridSize - 1; x > 0; x--) {
                visibleTreeGrid[x][y] |= treeGrid[x][y] > max;
                max = Math.max(max, treeGrid[x][y]);
            }
        }
        return visibleTreeGrid;
    }    


}