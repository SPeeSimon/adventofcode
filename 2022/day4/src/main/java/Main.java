package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.IntStream;

public class Main {

    record Range(int from, int to) {
        static Range fromInput(String range) {
            String[] rangeParts = range.split("-");
            return new Range(Integer.parseInt(rangeParts[0]), Integer.parseInt(rangeParts[1]));
        }

        boolean containsFully(Range other) {
            return from <= other.from && to >= other.to && to >= other.from;
        }

        boolean hasSection(int section) {
            return from <= section && to >= section;
        }
    }

    record ElfPair(Range elf1, Range elf2) {
        static ElfPair fromInput(String line) {
            String[] elfPair = line.split(",");
            return new ElfPair(Range.fromInput(elfPair[0]), Range.fromInput(elfPair[1]));
        }

        boolean needsReconsideration() {
            return elf1.containsFully(elf2) || elf2.containsFully(elf1);
        }

        long overlappingSections() {
            return IntStream.rangeClosed(Math.min(elf1.from, elf2.from), Math.max(elf1.to, elf2.to))
                            .filter(elf1::hasSection)
                            .filter(elf2::hasSection)
                            .count();
        }
    }

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day4/src/main/resources/input.txt")).toList();
        // var input = """
        //     2-4,6-8
        //     2-3,4-5
        //     5-7,7-9
        //     2-8,3-7
        //     6-6,4-6
        //     2-6,4-8
        //         """.lines().toList();

        long count = input.stream()
            .map(ElfPair::fromInput)
            .filter(e -> e.overlappingSections() > 0)
            .count();
        System.out.println("total: " + count);
    }
}
