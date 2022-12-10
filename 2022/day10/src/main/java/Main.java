package main.java;

import java.util.Arrays;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Main {
    final static int PIXEL_WIDTH = 40;
    final static int PIXEL_HEIGHT = 6;
    final static char PIXEL_LIT = '#';
    final static char PIXEL_DARK = ' ';
    final static StringBuilder DISPLAY = new StringBuilder(PIXEL_HEIGHT * PIXEL_WIDTH);
    final static int[] SIGNAL_STRENGTHS = new int[]{20, 60, 100, 140, 180, 220};

    static int registerX = 1;
    static int instruction = 0;
    static int sum = 0;

    public static void main(String[] args) throws IOException {
        // var input = Files.lines(Paths.get("2022/day10/src/main/resources/test1.txt")).toList();
        // var input = Files.lines(Paths.get("2022/day10/src/main/resources/test2.txt")).toList();
        var input = Files.lines(Paths.get("2022/day10/src/main/resources/input.txt")).toList();
        
        for (String command : input) {
            if ("noop".equals(command)) {
                tick();
            }
            else {
                tick();
                int add = Integer.parseInt(command.substring("addx".length()).trim(), 10);
                tick();
                registerX += add;
            }
        }

        System.out.println("sum: " + sum);
        showDisplay();
    }


    static void showDisplay() {
        System.out.println("---------------------------------------");
        for(int pixel_row = 0; pixel_row < PIXEL_HEIGHT; pixel_row++) {
            System.out.println(DISPLAY.substring(pixel_row*PIXEL_WIDTH).substring(0, PIXEL_WIDTH));
        }
        System.out.println("---------------------------------------");
    }


    static void updateDisplay() {
        boolean lit = Math.abs((instruction % PIXEL_WIDTH) - registerX) < 2;
        DISPLAY.append(lit ? PIXEL_LIT : PIXEL_DARK);
    }


    static void tick() {
        updateDisplay();
        instruction++;
        boostSignal();
    }
 

    static void boostSignal() {
        int signal = registerX;
        if( (instruction - 20) % 40 == 0) {
            signal = signal * instruction;
        }
        if (Arrays.binarySearch(SIGNAL_STRENGTHS, instruction) >= 0) {
            System.out.printf("%3d %d --> %d\n", instruction, registerX, signal);
            sum += signal;
        }
    }

}
