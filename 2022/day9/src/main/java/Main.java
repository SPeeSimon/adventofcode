package main.java;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import java.awt.Point;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Main {
    final static String HEAD = "H";
    final static String TAIL = "T";
    final static int ROPE_COUNT = 9;

    enum Direction {
        R(1, 0),
        U(0,-1),
        L(-1,0),
        D(0,1);

        final int x;
        final int y;

        Direction(int x, int y) {
            this.x = x;
            this.y = y;
        }

        void move(Point head) {
            head.translate(x, y);
        }
    
    }

    record Action(Direction direction, int steps) {
        static Action of(String line) {
            var parts = line.split(" ");
            return new Action(Direction.valueOf(parts[0]), Integer.parseInt(parts[1]));
        }
    }

    record Knot (int name, Point position) {
        public Stream<Point> follow(Point p) {
            int diffX = p.x - position.x;
            int diffY = p.y - position.y;
            int distanceX = Math.abs(diffX);
            int distanceY = Math.abs(diffY);

            if (distanceX < 2 && distanceY < 2) {
                return Stream.of(new Point(position));
            }

            if (distanceX > 1 || (distanceX + distanceY) > 2) {
                position.translate( diffX < 0 ? -1 : 1, 0);
            }
            if (distanceY > 1 || (distanceX + distanceY) > 2) {
                position.translate(0, diffY < 0 ? -1 : 1);
            }
            return Stream.of(new Point(position));
        }
    };

    static List<Knot> knots = new ArrayList<>();
    static Knot HeadPoint = new Knot(0, new Point(11, 15));

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day9/src/main/resources/input.txt"));
        // var input = """
        //     R 4
        //     U 4
        //     L 3
        //     D 1
        //     R 4
        //     D 1
        //     L 5
        //     R 2
        //         """.lines();
        // var input = """
        //     R 5
        //     U 8
        //     L 8
        //     D 3
        //     R 17
        //     D 10
        //     L 25
        //     U 20
        //     """.lines();

        for (int i = 1; i <= ROPE_COUNT; i++) {
            knots.add(new Knot(i, new Point(HeadPoint.position)));
        }
        
        var process = input.map(Action::of)
                           .flatMap(a -> actionToPosition(a));
        for (var knot : knots) {
            process = process.flatMap(knot::follow);
        }
        
        // printCurrentArray();
        System.out.println("Tail visited: " + process.distinct().count());
    }

    static Stream<Point> actionToPosition(Action a) {
        return IntStream.range(0, a.steps).mapToObj(i -> {
            a.direction.move(HeadPoint.position);
            return HeadPoint.position;
        });
    }

    static void printCurrentArray() {
        int gridSize = 27;
        var currentLocations = knots.stream().collect(Collectors.toMap(k -> k.position, k -> "" + k.name, (a,b) -> a));
        for(int y=0; y < gridSize; y++) {
            for(int x=0; x < gridSize; x++) {
                var point = new Point(x, y);
                if (point.equals(HeadPoint.position)) {
                    System.out.print('H');
                } else 
                    System.out.print(currentLocations.getOrDefault(point,"."));
            }
            System.out.println();
        }
        System.out.println("---------------------------------------------------");
    }
}
