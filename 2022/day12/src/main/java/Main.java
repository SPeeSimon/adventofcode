import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.function.BiConsumer;
import java.util.stream.Stream;
import java.awt.Point;

public class Main {

    static class Grid<T> {
        Data<T>[][] grid;
        final int width;
        final int heigth;

        public Grid(char[][] heightGrid, T defaultValue) {
            this.width = heightGrid.length;
            this.heigth = heightGrid[0].length;
            grid = new Data[width][heigth];
            for (int x = 0; x < width; x++) {
                for (int y = 0; y < heigth; y++) {
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

        boolean checkElevation(Point current, Point next) {
            int heightDiff = grid[next.x][next.y].height - grid[current.x][current.y].height;
            return heightDiff < 2;
        }

        boolean withinGrid(Point current) {
            return current.x >= 0 && current.y >= 0 && current.x < width && current.y < heigth;
        }
    
        void forAll(BiConsumer<Point, Data<T>> action) {
            for (int x=0; x < width; x++) {
                for (int y=0; y < heigth; y++) {
                    action.accept(new Point(x,y), point(x,y));
                }
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

    enum Direction {
        LEFT(-1, 0),
        UP(0, -1),
        RIGHT(1, 0),
        DOWN(0, 1);

        final int x;
        final int y;
        private Direction(int x, int y) {
            this.x = x;
            this.y = y;
        }

        Point move(Point point) {
            return new Point(point.x + x, point.y + y);
        }
    }

    record Move(Point from, Point to){}

    public static void main(String[] args) throws IOException {
        // var input = Files.lines(Paths.get("2022/day12/src/main/resources/test1.txt"));
        var input = Files.lines(Paths.get("2022/day12/src/main/resources/input.txt"));

        char[][] grid = input.map(line -> line.toCharArray())
                                .reduce(new char[0][], (v,r) -> add(v, r), (a,b) -> a);
        final Point start = getPoint('S', grid);
        final Point end = getPoint('E', grid);
        grid[start.x][start.y] = 'a'-1;
        grid[end.x][end.y] = 'z'+1;
        Grid<Integer> grid2 = new Grid<>(grid, Integer.MAX_VALUE);

        System.out.printf("grid: %dx%d  (%d)\n", grid2.width, grid2.heigth, grid2.width*grid2.heigth);
        System.out.println(findPath(start, end, grid2));
    }


    static int findPath(Point start, Point end, Grid<Integer> grid) {
        setStartingPoints(start, grid);
        for(int step=-1; step < grid.width*grid.heigth; step++) { // repeat until top is reached
            for (int x=0; x < grid.width; x++) {
                for (int y=0; y < grid.heigth; y++) {
                    if (grid.get(x, y) == step) {
                        var nextIsTop = setNextDirections(new Point(x, y), end, step, grid);
                        if (nextIsTop){
                            return step;
                        }
                    }
                }
            }
        }
        return Integer.MAX_VALUE;
    }


    static void setStartingPoints(Point start, Grid<Integer> grid) {
        grid.point(start.x, start.y).data = -1;
        grid.forAll((p,data) -> {
            if (data.height == 'a') {
                data.data = -1;
            }
        });
    }


    static boolean setNextDirections(Point point, Point end, int step, Grid<Integer> grid) {
        for(var direction : Direction.values()) {
            var nextPoint = direction.move(point);
            if (checkConditions(point, nextPoint, grid)) {
                grid.point(nextPoint.x, nextPoint.y).data = step + 1;
                if (nextPoint.equals(end)) {
                    return true;
                }
            }
        }
        return false;
    }


    private static boolean checkConditions(Point point, Point nextPoint, Main.Grid<Integer> grid) {
        return grid.withinGrid(nextPoint) && 
                grid.checkElevation(point, nextPoint) && 
                grid.point(nextPoint.x, nextPoint.y).data == Integer.MAX_VALUE; // point not reached before
    }


    static List<Point> nextPoint(Point from, Point end, char[][] grid) {
        return Stream.of(Direction.values())
                    .map(direction -> direction.move(from))
                    .filter(next -> next.x >= 0 && next.y >= 0 && next.x < grid.length && next.y < grid.length)
                    .filter(next -> (grid[from.x][from.y] - grid[next.x][next.y]) < 2 || next.equals(end))
                    .toList();
    }


    static Point getPoint(char lookup, char[][] grid) {
        for(int x=0; x < grid.length; x++) {
            for (int y=0; y < grid[x].length; y++) {
                if (grid[x][y]==lookup) {
                    return new Point(x, y);
                }
            }
        }
        return null;
    }


    static char[][] add(char[][] array, char[] entry) {
        char[][] copy = Arrays.copyOf(array, array.length + 1);
        copy[array.length] = entry;
        return copy;
    }

}