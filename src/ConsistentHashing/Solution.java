package ConsistentHashing;

import java.util.*;

public class Solution {
    /*
     * @param n: a positive integer
     * @param k: a positive integer
     * @return: a Solution object
     */

    private TreeSet<Integer> set = new TreeSet<>();
    private Map<Integer, Integer> virtualNodeToMachineId = new HashMap<>();
    private int N = -1;
    private int K = -1;

    public static Solution create(int n, int k) {
        // Write your code here
        Solution ret =  new Solution();
        ret.K = k;
        ret.N = n;
        return ret;
    }

    /*
     * @param machine_id: An integer
     * @return: a list of shard ids
     */
    public List<Integer> addMachine(int machine_id) {
        // write your code here
        int newVirtualNode = 0;
        Random rand = new Random();
        List<Integer> ret = new ArrayList<>();
        while (newVirtualNode < K) {
            int node = rand.nextInt(N);
            if (set.add(node)) {
                virtualNodeToMachineId.put(node, machine_id);
                newVirtualNode++;
                ret.add(node);
            }
        }
        return ret;
    }

    /*
     * @param hashcode: An integer
     * @return: A machine id
     */
    public int getMachineIdByHashCode(int hashcode) {
        // write your code here
        hashcode = hashcode % N;
        Integer next = set.ceiling(hashcode);
        if (next == null) {
            return virtualNodeToMachineId.get(set.ceiling(0));
        }
        return virtualNodeToMachineId.get(next);
    }
}