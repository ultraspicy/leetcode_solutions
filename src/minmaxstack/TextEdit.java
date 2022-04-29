package minmaxstack;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class TextEdit {
    public static void main(String[] args) {
        try {
            File myObj = new File("/Users/jianfeng/Documents/idea_projects/leetcode_solutions/src/minmaxstack/input.txt");
            Scanner myReader = new Scanner(myObj);
            while (myReader.hasNextLine()) {
                String data = "maxStack." + myReader.nextLine() + ";";
                System.out.println(data);
            }
            myReader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
}
