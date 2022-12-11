import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.LongSupplier;
import java.util.function.LongUnaryOperator;
import java.util.function.ToLongFunction;


public class Main {
    static long MAX = 1L;
    static LongUnaryOperator relieve = i -> i % MAX;
    // static LongUnaryOperator relieve = i -> (long)Math.round(i / 3L);

    static class Item{
        long worryLevel;

        Item(long worryLevel) {
            this.worryLevel = worryLevel;
        }

        @Override
        public String toString() {
            return Long.toString(worryLevel);
        }
    }

    record ThrowNext(int toMonkey, Item item) {}

    record ConstantValue(long value) implements ToLongFunction<Item>, LongSupplier {
        @Override
        public long applyAsLong(Item ignored) {
            return value;
        }

        @Override
        public long getAsLong() {
            return value;
        }
    }
    record Devision(long devision) {
        boolean test(long worryLevel) {
            return (worryLevel % devision) == 0L;
        }
    }

    static class Monkey {
        String name;
        List<Item> items = new ArrayList<>();
        long countInspected = 0;
        ToLongFunction<Item> operation;
        Devision test;
        ConstantValue ifTrue;
        ConstantValue ifFalse;

        List<ThrowNext> turn() {
            List<ThrowNext> throwItems = new ArrayList<>(items.size());
            for(Item item : items) {
                item.worryLevel = bored(inspect(item));
                if (test.test(item.worryLevel)) {
                    throwItems.add(new ThrowNext((int)ifTrue.getAsLong(), item));
                } else {
                    throwItems.add(new ThrowNext((int)ifFalse.getAsLong(), item));
                }
                countInspected++;
            }
            items.clear();
            return throwItems;
        }

        long inspect(Item item){
            return operation.applyAsLong(item);
        }

        long bored(long worryLevel){
            return relieve.applyAsLong(worryLevel);
        }

        @Override
        public String toString() {
            return name + ": " + items;
        }
    }

    private static List<Monkey> monkeys = new ArrayList<>();

    public static void main(String[] args) throws IOException {
        // var input = Files.lines(Paths.get("2022/day11/src/main/resources/test1.txt")).toList();
        var input = Files.lines(Paths.get("2022/day11/src/main/resources/input.txt")).toList();

        // read to data
        for(int i = 0; i < input.size(); i++) {
            Monkey monkey = new Monkey();
            monkey.name = input.get(i++).split(":")[0];
            String[] startItems = input.get(i++).split(":",2)[1].trim().split(",\\b*");
            monkey.items.addAll(Arrays.stream(startItems).map(str -> new Item(Long.parseLong(str.trim()))).toList());
            monkey.operation = processOperation(input.get(i++).split(":",2)[1].trim());
            monkey.test = getTestOperation(input.get(i++).split(":",2)[1].trim());
            monkey.ifTrue = getThrowOperation(input.get(i++).split(":",2)[1].trim());
            monkey.ifFalse = getThrowOperation(input.get(i++).split(":",2)[1].trim());
            monkeys.add(monkey);
        }

        MAX = monkeys.stream().mapToLong(m -> m.test.devision()).reduce((a,b)-> a*b).orElseThrow();

        int rounds = 10000;
        for(int i=0; i < rounds; i++) {
            for(int m=0; m < monkeys.size(); m++) {
                monkeys.get(m).turn().stream().forEach(action -> monkeys.get(action.toMonkey).items.add(action.item));
            }
        }

        displaySummary(rounds);
        displayMonkeyBusiness();
    }

    static void displaySummary(Object rounds) {
        System.out.printf("After %d rounds:\n", rounds);
        monkeys.forEach(m -> System.out.printf("%s inspected items %d times.\n", m.name, m.countInspected));
    }

    static void displayMonkeyBusiness() {
        long[] monkeyBusinesses = monkeys.stream().mapToLong(m -> m.countInspected).sorted().toArray();
        long sum = monkeyBusinesses[monkeyBusinesses.length-1] * monkeyBusinesses[monkeyBusinesses.length-2];
        System.out.println("monkey business: " + sum);
    }

    static ToLongFunction<Item> processOperation(String input) {
        String[] operation = input.split(" = ")[1].split(" ");
        final ToLongFunction<Item> leftOperand = getOperation(operation[0]);
        final ToLongFunction<Item> rightOperand = getOperation(operation[2]);

        if ("*".equals(operation[1])) {
            return item -> leftOperand.applyAsLong(item) * rightOperand.applyAsLong(item);
        }
        else if ("+".equals(operation[1])) {
            return item -> leftOperand.applyAsLong(item) + rightOperand.applyAsLong(item);
        }
        throw new IllegalStateException("Could not process operation: " + input);
    }

    static ToLongFunction<Item> getOperation(String operation) {
        if ("old".equals(operation)) {
            return item -> item.worryLevel;
        }
        return new ConstantValue(Long.parseLong(operation));
    }

    static Devision getTestOperation(String input) {
        if (input.startsWith("divisible by")) {
            return new Devision(Long.parseLong(input.substring("divisible by".length()).trim()));
        }
        throw new IllegalStateException("Could not process test operation: " + input);
    }

    static ConstantValue getThrowOperation(String input) {
        if (input.startsWith("throw to monkey")) {
            final int monkey = Integer.parseInt(input.substring("throw to monkey ".length()).trim());
            return new ConstantValue(monkey);
        }
        throw new IllegalStateException("Could not process throw operation: " + input);
    }
}
