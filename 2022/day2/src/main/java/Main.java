package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.Stream;

public class Main {

    enum Hand {
        ROCK(1) {
            @Override
            public Score against(Hand other) {
                if (this == other) return Score.DRAW;
                if (other == SCISSOR) return Score.WIN;
                return Score.LOSS;
            }    
        },
        PAPER(2) {
            @Override
            public Score against(Hand other) {
                if (this == other) return Score.DRAW;
                if (other == ROCK) return Score.WIN;
                return Score.LOSS;
            }    
        },
        SCISSOR(3) {
            @Override
            public Score against(Hand other) {
                if (this == other) return Score.DRAW;
                if (other == PAPER) return Score.WIN;
                return Score.LOSS;
            }    
        };

        final int points;

        Hand(int points) {
            this.points = points;
        }

        public Score against(Hand other) {
            return Score.DRAW;
        }

        public static Hand choice(String value) {
            switch (value) {
                case "A":
                    return ROCK;
                case "B":
                    return PAPER;
                case "C":
                    return SCISSOR;
                default:
                    return null;
            }
        }
    }

    enum Score {
        LOSS(),
        WIN(),
        DRAW();

        public static Score choice(String value) {
            switch (value) {
                case "X":
                    return LOSS;
                case "Y":
                    return DRAW;
                case "Z":
                    return WIN;
                default:
                    return null;
            }
        }
    }


    record Game(Hand opponent, Score outcome) {

        static Game of(String line) {
            var play = line.split(" ");
            return new Game(Hand.choice(play[0]), Score.choice(play[1]));
        }

        int score() {
            switch (outcome()) {
                case LOSS:
                    return yourHand().points + 0;
                case DRAW:
                    return yourHand().points + 3;
                case WIN:
                    return yourHand().points + 6;
            }
            return 0;
        }

        Hand yourHand() {
            return Stream.of(Hand.values())
                    .filter(h -> h.against(opponent) == outcome)
                    .findFirst()
                    .get();
        }
    }

    public static void main(String[] args) throws IOException {
       
        var games = Files.lines(Paths.get("2022/day2/src/main/resources/input.txt"))
                .map(Game::of)
                .toList();

        games.forEach(g -> System.out.printf(" - %s vs %s = %s (%d)\n", g.opponent, g.yourHand(), g.outcome(), g.score()));
        System.out.printf("games: %d with score %d\n", games.size(), games.stream().mapToInt(Game::score).sum());
    }
}
