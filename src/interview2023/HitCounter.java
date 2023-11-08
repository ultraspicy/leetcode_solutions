package interview2023;

import java.util.ArrayList;
import java.util.List;

class HitCounter {

    List<Integer> list;
//    int index;

    public HitCounter() {
        list = new ArrayList<>();
    }

    public void hit(int timestamp) {
        list.add(timestamp);
    }

    public int getHits(int timestamp) {
        int ret = 0, index = list.size() - 1;
        while (index >= 0 && list.get(index) < timestamp - 300 + 1) {
            index--;
        }

        return ret;
    }
}
