import java.util.HashMap;
import java.util.Map;
import java.util.TreeMap;
import java.util.concurrent.locks.ReentrantReadWriteLock;

class TimeMap {

    private final ReentrantReadWriteLock lock = new ReentrantReadWriteLock();
    private final ReentrantReadWriteLock.ReadLock readLock = lock.readLock();
    private final ReentrantReadWriteLock.WriteLock writeLock = lock.writeLock();
    private Map<String, TreeMap<Integer, String>> timeMap = new HashMap<>();

    public void set(String key, String value, int timestamp) {
        writeLock.lock();
        try {
            timeMap.computeIfAbsent(key, k -> new TreeMap<>()).put(timestamp, value);
        } finally {
            writeLock.unlock();
        }
    }

    public String get(String key, int timestamp) {
        readLock.lock();
        try {
            TreeMap<Integer, String> tsToValues = timeMap.get(key);
            if (tsToValues == null) return null;
            Map.Entry<Integer, String> entry = tsToValues.floorEntry(timestamp);
            return entry != null ? entry.getValue() : null;
        } finally {
            readLock.unlock();
        }
    }
}
