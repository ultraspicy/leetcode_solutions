public class Solution1360 {

    public int daysBetweenDates(String date1, String date2) {
        return Math.abs(dayFromBeginning(date1) - dayFromBeginning(date2));
    }

    private int dayFromBeginning (String date) {
        String[] yearMonthDay = date.split("-");
        int year = Integer.parseInt(yearMonthDay[0]), ret = 0;
        for (int i = 1971; i < year; i++) {
            ret += 365 + isLeapYear(i);
        }
        int[] days = new int[]{0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
        int month = Integer.parseInt(yearMonthDay[1]);
        for(int i = 1; i < month; i++) {
            ret += days[i];
            if (i == 2) {
                ret += isLeapYear(year);
            }
        }
        return ret + Integer.parseInt(yearMonthDay[2]);
    }

    private int isLeapYear(int year) {
        // step 1
        if (year % 4 == 0) {
            // step 2
            if(year % 100 == 0) {
                // step 3
                return year % 400 == 0 ? 1 : 0;
            } else {
                return 1;
            }
        } else {
            return 0;
        }
    }
}
