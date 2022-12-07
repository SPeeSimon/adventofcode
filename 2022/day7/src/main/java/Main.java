package main.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.function.Predicate;
import java.util.function.UnaryOperator;

public class Main {
    private static final int TOTAL_DISK_SPACE = 70000000;
    private static final int REQUIRED_UNUSED_SIZE = 30000000;
    private static final Predicate<Path> IS_DIRECTORY = Dir.class::isInstance;
    private static final Path ROOT = new Dir("/", null, new ArrayList<>());


    interface Path {
        String name();
        long size();
        Path parent();
        List<Path> content();
        Path subdir(String substring);
    }


    record Dir(String name, Path parent, List<Path> content) implements Path {
        static Path create(Dir parent, String name) {
            var dir = new Dir(name, parent, new ArrayList<>());
            parent.content.add(dir);
            return dir;
        }

        public long size() {
            return content.stream().mapToLong(Path::size).sum();
        }

        public Path subdir(String substring) {
            return content.stream().filter(p -> p.name().equals(substring)).findFirst().orElseThrow();
        }

        @Override
        public String toString() {
            return String.format("%s (dir) content=%s", name, content);
        }
    }

    record File(String name, Path parent, long size) implements Path {
        @Override
        public List<Path> content() {
            return Collections.emptyList();
        }

        public Path subdir(String substring) {
            throw new IllegalStateException("File is not a directory: " + name);
        }


        @Override
        public String toString() {
            return String.format("%s (file, size=%d)", name, size);
        }
    }


    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day7/src/main/resources/input.txt"));
        // var input = """
        //     $ cd /
        //     $ ls
        //     dir a
        //     14848514 b.txt
        //     8504156 c.dat
        //     dir d
        //     $ cd a
        //     $ ls
        //     dir e
        //     29116 f
        //     2557 g
        //     62596 h.lst
        //     $ cd e
        //     $ ls
        //     584 i
        //     $ cd ..
        //     $ cd ..
        //     $ cd d
        //     $ ls
        //     4060174 j
        //     8033020 d.log
        //     5626152 d.ext
        //     7214296 k
        //         """.lines();

        input.map(Main::toAction)
             .reduce(ROOT, (p, a) -> a.apply(p), (t, u) -> t);
        assignment1();
        assignment2();
        // listTree(ROOT, "");
    }


    private static void assignment1() {
        System.out.println("Total: " + findOfSize(ROOT, IS_DIRECTORY.and(p -> p.size() <= 100000))
                                                .stream()
                                                .mapToLong(Path::size)
                                                .sum());
    }


    private static void assignment2() {
        final long unusedDiskSpace = TOTAL_DISK_SPACE - ROOT.size();
        final long needsCleanupSize = REQUIRED_UNUSED_SIZE - unusedDiskSpace;
        System.out.println("------------");
        System.out.printf("Total: %d\n Used: %d\n Free: %d\nRequired: %d\n", TOTAL_DISK_SPACE, ROOT.size(), unusedDiskSpace, needsCleanupSize);

        findOfSize(ROOT, IS_DIRECTORY.and(p -> p.size() >= needsCleanupSize))
            .stream()
            .min(Comparator.comparing(Path::size))
            .ifPresent(p -> System.out.printf("cleanup: %s (freeing %d)\n", p.name(), p.size()));
    }


    // private static void listTree(Path path, String prefix) {
    //     System.out.printf("%s- %s (%s) (%d)\n", prefix, path.name(), path.getClass().getSimpleName(), path.size());
    //     path.content().forEach(p -> listTree(p, prefix + "  "));
    // }


    private static List<Path> findOfSize(final Path path, final Predicate<Path> filter) {
        var result = new ArrayList<Path>();
        if (filter.test(path)) {
            result.add(path);
        }
        path.content().stream().flatMap(p -> findOfSize(p, filter).stream()).forEach(result::add);
        return result;
    }


    private static UnaryOperator<Path> toAction(String input) {
        if ("$ cd /".equals(input)) {
            return p -> ROOT; // go to root action
        } else if ("$ cd ..".equals(input)) {
            return Path::parent; // go to parent action
        } else if ("$ ls".equals(input)) {
            return UnaryOperator.identity(); // next is content
        } else if (input.startsWith("$ cd ")) {
            return p -> p.subdir(input.substring("$ cd ".length())); // go to subdir action
        } else if (input.startsWith("dir")) {
            return p -> {
                Dir.create((Dir)p, input.split(" ", 2)[1]); // dir
                return p;
            };
        } else {
            return p -> {
                var file = input.split(" ", 2);
                p.content().add(new File(file[1], ROOT, Integer.parseInt(file[0]))); // file
                return p;
            };
        }
    }

}
