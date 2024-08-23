
import java.util.*;
import java.util.stream.Collector;
import java.util.stream.Collectors;

class AutocompleteSystem {

    private final Node root = new Node();

    private StringBuilder sb = new StringBuilder();
    private Node p = root;
    public static void main(String[] args) {
        AutocompleteSystem ac = new AutocompleteSystem(
            new String[]{
                "i love you",
                "island",
                "iroman",
                "i love leetcode"
            }, 
            new int[]{5,3,2,2});

        ac.input('i');    
    }

    public AutocompleteSystem(String[] sentences, int[] times) {
        for(int i = 0; i < sentences.length; i++) {
            // insert a sentence to inversed index
            root.insert(sentences[i], times[i]);
        }
    }
    
    public List<String> input(char c) {
        if (c == '#') {
            p = root;
            root.insert(sb.toString(), 1);
            sb = new StringBuilder();
            return new ArrayList<>();
        }
        if (!p.children.keySet().contains(c)) {
            p.children.put(c, new Node());
        }
        p = p.children.get(c);
        // sort the entry
        List<String> ret = p.stringOccurance.entrySet().stream().sorted(
            (e1, e2) -> {
                int freqCompare = e2.getValue().compareTo(e1.getValue());
                if (freqCompare != 0) {
                    return freqCompare;
                }
                return e1.getKey().compareTo(e2.getKey());
            }
        ).limit(3).map(Map.Entry::getKey).collect(Collectors.toList());
        
        sb.append(c);
        return ret;
    }

    class Node {
        Map<String, Integer> stringOccurance = new HashMap<>();
        Map<Character, Node> children = new HashMap<>();

        // insert the (sentence, times) to the inversed index
        private void insert (String sentence, int times) {
            Node p = this;
            for (int i = 0; i < sentence.length(); i++) {
                char c = sentence.charAt(i);
                if (!p.children.keySet().contains(c)) {
                    p.children.put(c, new Node());
                }
                p = p.children.get(c);
                p.stringOccurance.put(sentence, p.stringOccurance.getOrDefault(sentence, 0) + times);
            }
        }
    }
}

/**
 * Your AutocompleteSystem object will be instantiated and called as such:
 * AutocompleteSystem obj = new AutocompleteSystem(sentences, times);
 * List<String> param_1 = obj.input(c);
 */