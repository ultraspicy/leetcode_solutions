package databricks;

import java.util.*;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Trie {

    class Node{
        char c;
        boolean end;
        Map<Integer, Node> children;
        Set<String> words;
        String prefix;

        public Node (char c) {
            this.c = c;
            this.end = false;
            children = new HashMap<>();
            words = new HashSet<>();
            prefix = "";
        }

        public Node (char c, String prefix) {
            this.c = c;
            this.prefix = prefix;
            this.end = false;
            children = new HashMap<>();
            words = new HashSet<>();
        }
    }

    Node root = new Node('*');

    private void buildFullTrie (String[] strs) {
        for (String str : strs) {
            buildTrie(str);
        }
    }

    private void buildTrie(String str) {
        Node cur = root;
        for(int i = 0; i < str.length(); i++) {
            // move to the next node
            int index = str.charAt(i) - 'a';
            char newChar = (char)('a' + index);
            String newPrefix = cur.prefix + newChar;
            cur.children.computeIfAbsent(index, k -> new Node(newChar, newPrefix));
            cur = cur.children.get(index);
            String prev = cur.prefix;
            // update node state
            cur.words.add(str);
            if (i == str.length() - 1) {
                cur.end = true;
            }
        }
    }

    private void traversal(Node root, List<String> list) {
        if (root == null) {
            return;
        }
        if (root.end) {
            Iterator<String> iter = root.words.iterator();
            String word = iter.next();
            list.add(word);
            return;
        }
        if(root.words.size() == 1) {
            Iterator<String> iter = root.words.iterator();
            String word = iter.next();
            list.add("" + root.prefix + (word.length() - root.prefix.length() - 1) + word.charAt(word.length() - 1));
            return;
        }
        for(int key : root.children.keySet()) {
            traversal(root.children.get(key), list);
        }
    }

    public static void main(String[] args) {
        Trie trie = new Trie();
        trie.buildFullTrie(new String[]{"banana", "bancna", "bananc"});
        List<String> list = new ArrayList<>();
        trie.traversal(trie.root, list);

        for(String str : list) {
            System.out.println(str + "\t");
        }
    }

}
