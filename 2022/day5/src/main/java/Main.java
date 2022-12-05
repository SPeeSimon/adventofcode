package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public class Main {
    record Ship(List<CargoStack> cargo) {
        public static Ship fromInput(String line) {
            var ship = new Ship(Stream.of(line.split(" "))
                                      .filter(s -> !s.isEmpty())
                                      .mapToInt(Integer::valueOf)
                                      .mapToObj(i -> new CargoStack(i, new ArrayDeque<>()))
                                      .toList());
            return ship;
        }

        public CargoStack getCargoAtIndex(int index) {
            return cargo.get(index);
        }

        public CargoStack getCargo(int index) {
            return cargo.stream().filter(c -> c.index == index).findFirst().get();
        }

        public void executeMovement(Movement action) {
            var from = getCargo(action.from);
            var to = getCargo(action.to);
            var moved = new ArrayList<String>();
            for(int i = 0; i < action.aantal; i++) {
                moved.add(0, from.pop());
            }
            moved.forEach(to::push);
        }

        public List<String> getTop() {
            return cargo.stream()
                        .map(c -> c.top())
                        .toList();
        }
    }

    record CargoStack(int index, ArrayDeque<String> crates) {
        public void push(String crate) {
            this.crates.push(crate);
        }

        public String pop() {
            return this.crates.pop();
        }

        public String top() {
            return this.crates.peek();
        }
    }

    record Movement(int aantal, int from, int to) {
        public static Movement fromInput(String line) {
            var token = line.split(" ");
            return new Movement(Integer.parseInt(token[1]), Integer.parseInt(token[3]), Integer.parseInt(token[5]));
        }
    }

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day5/src/main/resources/input.txt")).toList();
        // var input = """
        //         [D]    
        //     [N] [C]    
        //     [Z] [M] [P]
        //      1   2   3 
            
        //     move 1 from 2 to 1
        //     move 3 from 1 to 3
        //     move 2 from 2 to 1
        //     move 1 from 1 to 2
        //     """.lines().toList();
        
        var cargoShip = readCargoShipSituation(input);
        executeMovements(cargoShip, input);
        System.out.println("Top items: " + cargoShip.getTop());
    }

    private static void executeMovements(Ship cargoShip, List<String> input) {
        input.stream()
             .dropWhile(s -> !s.isEmpty())
             .filter(s -> !s.isEmpty())
             .map(Movement::fromInput)
             .forEach(mv -> cargoShip.executeMovement(mv));
    }

    private static Ship readCargoShipSituation(List<String> input) {
        var currentStacks = input.stream()
                                 .takeWhile(s -> !s.isEmpty())
                                 .toList();
        var ship = Ship.fromInput(currentStacks.get(currentStacks.size()-1));

        for(int i = currentStacks.size()-2; i >= 0; i--) {
            for(int stackIndex = 0; stackIndex < ship.cargo().size(); stackIndex++) {
                readCargoStackItem(ship, currentStacks.get(i), stackIndex);
            }
        }
        return ship;
    }

    private static void readCargoStackItem(Ship ship, String cargoLine, int stackIndex) {
        var strIndex = stackIndex * 4;
        if (strIndex < cargoLine.length()) {
            var cargoBox = cargoLine.substring(strIndex + 1, strIndex + 2).trim();
            if (!cargoBox.isEmpty()) {
                ship.getCargoAtIndex(stackIndex).push(cargoBox);
            }
        }
    }

}
