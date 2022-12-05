package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

public class Main {

    record Rucksack (String compartment1, String compartment2) {

        public static Rucksack fromInput(String line) {
            int split = line.length() / 2;
            return new Rucksack(line.substring(0, split), line.substring(split));
        }

        List<Character> duplicateInCompartment() {
            return compartment1.chars()
                        .filter(c -> compartment2.indexOf((char)c) > -1)
                        .distinct()
                        .mapToObj(c -> Character.valueOf((char)c))
                        .toList();
        }

        boolean carries(char c) {
            return compartment1.indexOf(c) > -1 || compartment2.indexOf(c) > -1;
        }

        int sumPriorities() {
            return duplicateInCompartment().stream()
                                    .mapToInt(c -> priority((char)c))
                                    .sum();
        }

        IntStream allItems() {
            return IntStream.concat(compartment1.chars(), compartment2.chars());
        }
    }

    public record Group(Rucksack elf1, Rucksack elf2, Rucksack elf3) {
        char badge() {
            return (char)elf1.allItems()
                             .filter(c -> elf2.carries((char)c))
                             .filter(c -> elf3.carries((char)c))
                             .findFirst()
                             .orElse('?');
        }
    }

    static int priority(char c) {
        return " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".indexOf(c);
    }

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day3/src/main/resources/input.txt"))
        // var input = """
        //     vJrwpWtwJgWrhcsFMMfFFhFp
        //     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        //     PmmdzqPrVvPwwTWBwg
        //     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        //     ttgJtRGJQctTZtZT
        //     CrZsJsPPZsGzwwsLwLmpwMDw
        //         """.lines()
                .map(Rucksack::fromInput)
                .toList();

        var groups = new ArrayList<Group>();
        for(var elf = input.iterator(); elf.hasNext();) {
            groups.add(new Group(elf.next(), elf.next(), elf.next()));
        }

        int sum = groups.stream()
                .mapToInt(g -> priority(g.badge()))
                .sum();
        System.out.println("Total: " + sum);
    }

}
