package databricks;

import java.util.*;

public class Solution {

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.computePenalty("Y Y N Y", 0));
        System.out.println(solution.computePenalty("N Y N Y", 2));
        System.out.println(solution.computePenalty("Y Y N Y", 4));

        System.out.println("========");
        System.out.println(solution.findBestClosingTime("Y Y N N"));
        System.out.println(solution.findBestClosingTime("Y Y Y Y"));
        System.out.println(solution.findBestClosingTime("N N N N"));

        System.out.println("========");
        //System.out.println(solution.computeLogs("BEGIN Y Y END \nBEGIN N N END"));
        System.out.println(solution.computeLogs("BEGIN BEGIN \nBEGIN N N BEGIN Y Y \n END N N END"));
    }

    private List<Integer> computeLogs(String log) {
        int index = 0;
        List<Integer> ret = new ArrayList<>();

        while(index != -1 && index < log.length()) {
            int beginIndex = log.indexOf("BEGIN", index);
            if(beginIndex == -1) {
                break;
            }
            int nextBegin = log.indexOf("BEGIN", beginIndex + 5);
            int nextEnd = log.indexOf("END", beginIndex + 5);
            if(nextEnd == -1) {
                break;
            } else if(nextBegin != -1 && nextEnd > nextBegin) {
                // BEGIN BEGIN END
                // processing
                index = nextBegin;
                continue;
            } else {
                // findBestClosingTime
                String sub = log.substring(beginIndex + 6, nextEnd);
                ret.add(findBestClosingTime(sub.trim()));
                index = nextEnd + 3;
            }
        }
        return ret;
    }

    private int findBestClosingTime(String logs) {
        int maxClosing = (logs.length() + 1) / 2;
        int minPenalty = Integer.MAX_VALUE;
        int ret = -1;

        for(int i = 0; i <= maxClosing; i++) {
            int penalty = computePenalty(logs, i);
            if(penalty < minPenalty) {
                minPenalty = penalty;
                ret = i;
            }
        }
        return ret;
    }

    private int computePenalty(String logs, int closing) {
        if(closing < 0) {
            return -1;
        }
        logs = "# " + logs;
        String[] logArray = logs.split(" ");

        int NBeforeT = 0, YAfterT = 0;
        for(int i = 0; i < logArray.length; i++) {
            if(i <= closing) {
                if(logArray[i].charAt(0) == 'N') {
                    NBeforeT++;
                }
            } else {
                if(logArray[i].charAt(0) == 'Y') {
                    YAfterT++;
                }
            }
        }
        return NBeforeT + YAfterT;
    }
}
