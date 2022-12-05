import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;


class Main {

    record Elf(List<Integer> carried) {

        void add(String size) {
            var count = Integer.parseInt(size.trim());
            carried.add(count);
        }

        int sum() {
            return carried.stream().mapToInt(Integer::intValue).sum();
        }

    }


    public static void main(String[] args) throws IOException {
        var elves = new ArrayList<Elf>();
        elves.add(new Elf(new ArrayList<>()));

        for (var line : Files.readAllLines(Paths.get("2022/day1/src/main/resources/input.txt"))) {
            if (line.isEmpty()) {
                elves.add(new Elf(new ArrayList<>()));
            } else {
                elves.get(elves.size()-1).add(line);
            }
        }

        elves.sort(Comparator.comparingInt(Elf::sum).reversed());
        displayTop(3, elves);
    }
    
    private static void displayTop(int count, List<Elf> elves) {
        var total = elves.stream().limit(count).mapToInt(Elf::sum).sum();
        System.out.printf("total elves: %d\n", elves.size());
        System.out.printf("Top %d carried: %d\n", count, total);
    }
}
