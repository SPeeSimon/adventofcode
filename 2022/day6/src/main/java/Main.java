package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;

public class Main {
    private static final int MARKER_SIZE_1 = 4;
    private static final int MARKER_SIZE_2 = 14;
    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day6/src/main/resources/input.txt")).toList();
        // var input = Arrays.asList(
        //         "bvwbjplbgvbhsrlpgdmjqwftvncz",
        //             "nppdvjthqldpwncqszvftbrmjlhg",
        //             "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        //             "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        //             );
        // var input = Arrays.asList(
        //         "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        //             "bvwbjplbgvbhsrlpgdmjqwftvncz",
        //             "nppdvjthqldpwncqszvftbrmjlhg",
        //             "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        //             "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        //             );

        input.forEach(i ->
            System.out.printf("%s, first marker: %d, first message: %d\n", 
                            i, nextMarkerSequence(i, 0, MARKER_SIZE_1), nextMarkerSequence(i, 0, MARKER_SIZE_2))
        );
    }

    private static int nextMarkerSequence(String input, int index, int markerSize) {
        for(; index < input.length() - markerSize; index++) {
            if (!hasDuplicate(input.substring(index, index + markerSize))) {
                return index + markerSize;
            }
        }
        return -1;
    }

    private static boolean hasDuplicate(String input) {
        for(int i = 0; i < input.length(); i++) {
            for(int c = i + 1; c < input.length(); c++) {
                if (input.charAt(i) == input.charAt(c)) {
                    return true;
                }
            }
        }
        return false;
    }
}
