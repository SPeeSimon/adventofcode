import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Main {

    enum Compare {
        LEFT(-1), RIGHT(1), BOTH(0);

        final int value;

        Compare(int value) {
            this.value = value;
        }
    }

    interface Node {
        int size();
    }

    record Pair(String left, String right){}
    static class ValueNode implements Node, Comparable<ValueNode> {
        int value;

        ValueNode(int value) {
            this.value = value;
        }
        @Override
        public int size() {
            return 1;
        }

        public TreeNode asTreeNode() {
            return new TreeNode(Arrays.asList(this));
        }

        @Override
        public int compareTo(ValueNode other) {
            if (value < other.value) 
                return Compare.RIGHT.value;
            if (value > other.value) 
                return Compare.LEFT.value;
            return Compare.BOTH.value;
        }

        @Override
        public String toString() {
            return Integer.toString(value);
        }
    }

    record TreeNode(List<Node> values) implements Node {
        @Override
        public int size() {
            return values.size();
        }

        public TreeNode subList() {
            var result = new TreeNode(new ArrayList<>());
            values.add(result);
            return result;
        }

        public void replaceLast(Main.ValueNode result) {
            values.remove(values.size()-1);
            values.add(result);
        }

        public void updateLatestValue(int digit) {
            var valueNode = (ValueNode)values.get(values.size()-1);
            valueNode.value = digit + (valueNode.value * 10);
        }

        @Override
        public String toString() {
            return values.toString();
        }
    }


    static TreeNode[] DIVIDER_PACKETS = {
        new TreeNode(Arrays.asList(new ValueNode(2).asTreeNode())), // "[[2]]"
        new TreeNode(Arrays.asList(new ValueNode(6).asTreeNode()))  // "[[6]]"
    };

    public static void main(String[] args) throws IOException {
        var input = Files.lines(Paths.get("2022/day13/src/main/resources/input.txt")).toList();
        // var input = Files.lines(Paths.get("2022/day13/src/main/resources/test.txt")).toList();

        
        var pairs = new ArrayList<TreeNode>();
        for(int i=0; i < input.size(); i+=3) {
            pairs.add(parseLine(input.get(i).trim()));
            pairs.add(parseLine(input.get(i+1).trim()));
        }
        pairs.sort((a,b) -> -compareTo(a, b));
        int[] beforeIndexes = new int[]{1, 2}; // 1 based index, +1 for [[2]] not in index

        for(int i=0; i < pairs.size(); i++) {
            var pair = pairs.get(i);
            for(int di =0; di < DIVIDER_PACKETS.length; di++){
                boolean rightOrder = compareTo(pair, DIVIDER_PACKETS[di]) != Compare.LEFT.value;
                if (rightOrder) {
                    beforeIndexes[di]++;
                }
            }
        }
        System.out.println("\nTotal: " + Arrays.toString(beforeIndexes) + " => " + Arrays.stream(beforeIndexes).reduce((a,b) -> a*b).getAsInt());
    }


    static boolean rightOrder(String left, String right) {
        var leftNode = parseLine(left);
        var rightNode = parseLine(right);
        
        int outcome = compareTo(leftNode, rightNode);
        return outcome != Compare.LEFT.value;
    }


    static int compareTo(Node left, Node right) {
        // System.out.println(left + " vs " + right);

        // ValueNode vs ValueNode
        if (left instanceof ValueNode l && right instanceof ValueNode r) {
            return l.compareTo(r);
        }

        // TreeNode vs TreeNode
        if (left instanceof TreeNode l && right instanceof TreeNode r) {
            return compareTo(l, r);
        }

        // if entry is value and tree then value as tree and retry
        if (left instanceof ValueNode l) {
            return compareTo(l.asTreeNode(), right);
        } else if (right instanceof ValueNode r) {
            return compareTo(left, r.asTreeNode());
        }
        throw new IllegalStateException("Invalid state for nodes");
    }

    static int compareTo(TreeNode left, TreeNode right) {
        // foreach(node : tree) check(node)
            // if left.isempty && right.isnotempty = OK
            // if left.isnotempty && right.isempty = NOK
            // if both.empty = check more
        for (int i = 0; i < Math.min(left.size(), right.size()); i++) {
            int cmp = compareTo(left.values.get(i), right.values.get(i));
            if (cmp != 0) {
                return cmp;
            }
        }
        return left.size() > right.size() ? Compare.LEFT.value : 
               left.size() < right.size() ? Compare.RIGHT.value : Compare.BOTH.value;
    }


    static TreeNode parseLine(String line) {
        TreeNode result = new TreeNode(new ArrayList<>());
        var stack = new ArrayDeque<TreeNode>();
        var input = line.toCharArray();
        for(int i=0; i < input.length; i++) {
            if (input[i] == '[') {
                if (stack.isEmpty()) {
                    stack.push(result);
                } else {
                    var newTree = stack.peek().subList();
                    stack.push(newTree);
                }
            } else if (input[i] == ']') {
                stack.pop();
            } else if (input[i] == ',') {

            } else if (Character.isDigit(input[i])) {
                if (Character.isDigit(input[i-1])) {
                    stack.peek().updateLatestValue(Character.digit(input[i], 10));
                } else {
                    stack.peek().values.add(new ValueNode(Character.digit(input[i], 10)));
                }
            }
        }
        return result;
    }

}