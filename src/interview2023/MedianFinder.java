package interview2023;

import java.util.PriorityQueue;
import java.util.Queue;

class MedianFinder {

    Queue<Integer> minHeap;
    Queue<Integer> maxHeap;
    public MedianFinder() {
        minHeap = new PriorityQueue<>((x, y) -> y - x);
        maxHeap = new PriorityQueue<>();
    }

    public void addNum(int num) {
        maxHeap.add(num);
        minHeap.add(maxHeap.poll());
        if(minHeap.size() > maxHeap.size()) {
            maxHeap.add(minHeap.poll());
        }
    }

    public double findMedian() {
        if(maxHeap.size() > minHeap.size()) {
            return maxHeap.peek();
        }
        return (double)(minHeap.peek() + maxHeap.peek())/2;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder obj = new MedianFinder();
 * obj.addNum(num);
 * double param_2 = obj.findMedian();
 */