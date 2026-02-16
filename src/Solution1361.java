import java.util.*;

public class Solution1361 {
    public boolean validateBinaryTreeNodes(int n, int[] leftChild, int[] rightChild) {
        if (n == 1) return true;
        Map<Integer, Integer> m = new HashMap<>();
        int[] flag = new int[n];
        for (int i =0; i < leftChild.length; i++) {
            if (leftChild[i] != -1) {
                if(m.putIfAbsent(leftChild[i], i) != null) {
                    return false;
                }
                flag[leftChild[i]] = -1;
            }

            if(rightChild[i] != -1) {
                if (m.putIfAbsent(rightChild[i], i) != null) {
                    return false;
                }
                flag[rightChild[i]] = -1;
            }
        }
        if(m.size() != n - 1) return false;
        for(int i = 0 ; i < n; i++) {
            if (flag[i] != -1) {
                return !(leftChild[i] == -1 && rightChild[i] == -1);
            }
        }

        return false;
    }
}

