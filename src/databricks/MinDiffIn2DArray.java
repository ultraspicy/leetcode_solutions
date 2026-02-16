package databricks;

import java.util.*;

public class MinDiffIn2DArray {
    public static void main(String[] args) {
        MinDiffIn2DArray m = new MinDiffIn2DArray();
        List<Integer> a = Arrays.asList(61,90,60);
        List<Integer> b = Arrays.asList(59, 61);
        List<Integer> c = Arrays.asList(58,62,92);
        List<List<Integer>> input = List.of(a, b, c);

        List<Integer> ret = m.solution(input);
        System.out.println(ret);
    }

    private class Tuple implements Comparable<Tuple> {
        int r;
        int c;
        int num;
        public Tuple(int r, int c, int num) {
            this.r = r;
            this.c = c;
            this.num = num;
        }

        @Override
        public int compareTo(Tuple o) {
            return this.num - o.num;
        }
    }

    private List<Integer> solution (List<List<Integer>> input) {
        List<List<Tuple>> tuples = new ArrayList<>();
        for(int i = 0; i < input.size(); i++) {
            Collections.sort(input.get(i));
        }
        for (int i = 0; i < input.size(); i++) {
            List<Tuple> tupleList = new ArrayList<>();
            for (int j = 0; j < input.get(i).size(); j++) {
                tupleList.add(new Tuple(i, j, input.get(i).get(j)));
            }
            tuples.add(tupleList);
        }
        Queue<Tuple> heap = new PriorityQueue<>();
        Queue<Tuple> snapshot = null;
        int minDiff = Integer.MAX_VALUE;
        int maxInHeap = Integer.MIN_VALUE;

        for(int i = 0; i < input.size(); i++) {
            heap.add(tuples.get(i).get(0));
            maxInHeap = Math.max(maxInHeap, tuples.get(i).get(0).num);
        }

        while (true) {
            Tuple top = heap.poll();
            if (minDiff > maxInHeap - top.num) {
                snapshot = new PriorityQueue<>(heap);
                snapshot.add(top);
                minDiff = maxInHeap - top.num;
            }
            if (top.c == tuples.get(top.r).size() - 1) {
                break;
            }
            heap.add(tuples.get(top.r).get(top.c + 1));
            maxInHeap = Math.max(maxInHeap, tuples.get(top.r).get(top.c + 1).num);
        }
        List<Integer> ret = new ArrayList<>();
        for (int i = 0; i < tuples.size(); i++) {
            ret.add(snapshot.poll().num);
        }
        return ret;
    }
}
