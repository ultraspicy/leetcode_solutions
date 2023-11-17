package interview2023;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;

class LRUCache {

    class Node {
        int key;
        int val;
        Node left;
        Node right;
        public Node(int key, int val) {
            this.key = key;
            this.val = val;
        }
    }
    int capacity;
    Node dummyHead;
    Node dummyTail;

    Map<Integer, Node> map;
    int size;

    public LRUCache(int capacity) {
        System.out.println("capacity " + capacity);
        map = new HashMap<>();
        this.capacity = capacity;
        dummyHead = new Node(-1, -1);
        dummyTail = new Node(-1,-1);
        dummyHead.right = dummyTail;
        dummyTail.left = dummyHead;
        size = 0;
    }

    public int get(int key) {
        System.out.println("get " + key);
        if (!map.containsKey(key)) return -1;
        Node node = map.get(key);
        arrangeNode(node);
        return map.get(key).val;
    }

    // maintain size, map and list
    public void put(int key, int value) {
        System.out.println("put key,val = " + key + "," + value);
        if(map.containsKey(key)) {
            Node node = map.get(key);
            node.val = value;
            arrangeNode(node);
        } else {
            // new node
            if(size < capacity) {
                Node node = new Node(key, value);
                map.put(key, node);
                addLastNode(node);
                size++;
            } else {
                Node node = new Node(key, value);
                map.put(key, node);
                addLastNode(node);
                removeFirstNode();
            }
        }
    }

    private void removeFirstNode() {
        // dummyHead <----> first <----> second
        Node first = dummyHead.right;
        dummyHead.right = first.right;
        first.right.left = dummyHead;
        map.remove(first.key);
    }

    private void addLastNode(Node node) {
        // last <----> dummyTail
        Node last = dummyTail.left;
        last.right = node;
        dummyTail.left = node;
        node.left = last;
        node.right = dummyTail;
    }

    private void arrangeNode (Node node) {
        // left <----> node <-----> right
        Node left = node.left, right = node.right;
        left.right = right;
        right.left = left;

        // last <----> dummyTail
        // last <----> node <-----> dummyTail
        Node last = dummyTail.left;
        node.left = last;
        node.right = dummyTail;
        last.right = node;
        dummyTail.left = node;
    }
}