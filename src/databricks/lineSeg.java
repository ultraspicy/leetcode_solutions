package databricks;

import java.util.*;

public class lineSeg {

    static class Pair {
        double x;
        double y;

        public Pair(double x, double y) {
            this.x = x;
            this.y = y;
        }
    }

    void print(List<List<Pair>> input) {
        for(List<Pair> list: input) {
            System.out.println("======");
            for(Pair p : list) {
                System.out.println(p.x + ":" + p.y + "\t");
            }
        }
    }

    public static void main(String[] args) {
        lineSeg lineSeg = new lineSeg();
        List<Pair> list1 = Arrays.asList(new Pair(1, 1), new Pair(2, 2), new Pair(3.5, 3.5));
        List<Pair> list2 = Arrays.asList(new Pair(4, 2), new Pair(2, 1));
        List<Pair> list3 = Arrays.asList(new Pair(4, 4), new Pair(5, 5), new Pair(6.5, 6.5));
        List<List<Pair>> input = Arrays.asList(list1, list2, list3);
        List<List<Pair>> ret = lineSeg.lineSeg(input);
        lineSeg.print(ret);
    }

    private List<List<Pair>> lineSeg(List<List<Pair>> input) {
        Map<Double, Map<Double, List<Pair>>> map = new HashMap<>();
        for(List<Pair> list : input) {
            // compute c and b
            Pair first = list.get(0), last = list.get(list.size() - 1);
            double offset = computeOffset(first, last);
            double slope = computeSlope(first, offset);

            // update map
            Map<Double, List<Pair>> inner = null;
            for(double key : map.keySet()) {
                if(slope < key + epsilon && slope > key - epsilon) {
                    inner = map.get(key);
                    break;
                }
            }
            if (inner == null) {
                map.put(slope, new HashMap<>());
                inner = map.get(slope);
            }

            List<Pair> lineGroup = null;
            for(double key: inner.keySet()) {
                if(offset < key + epsilon && offset > key - epsilon) {
                    lineGroup = inner.get(key);
                    break;
                }
            }
            if (lineGroup == null) {
                inner.put(offset, new ArrayList<>());
                lineGroup = inner.get(offset);
            }
            lineGroup.addAll(list);
        }
        List<List<Pair>> ret = new ArrayList<>();

        for(double c : map.keySet()) {
            for(double b : map.get(c).keySet()) {
                ret.add(map.get(c).get(b));
            }
        }
        return ret;
    }

    double epsilon = 0.001;

    private double computeOffset(Pair p1, Pair p2) {
        return (p2.x * p1.y - p1.x * p2.y) / (p2.x - p1.x);
    }

    private double computeSlope(Pair p, double b) {
        return (p.y - b) / p.x;
    }
}
