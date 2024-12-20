// package java.StaticArrays;

public class StaticArray {
  // Method 1: Declare and initialize in one line
  static int[] numbers = {1, 2, 3, 4, 5};

  // Method 2: Declare fixed-size array and initialize later
  static String[] names = new String[3];

  public static void main(String[] args) {
    for (int num : numbers) {
      System.out.println(num);
    }

    names[0] = "John";
    names[1] = "Jane";
    names[2] = "Jack";

    for (String name : names) {
      System.out.println(name);
    }
  }
}
