import java.io.*;
import java.util.*;
import java.text.*;
import java.math.*;
import java.util.regex.*;

public class Solution {

    public static int gcd (int a, int b) {
        if ( b == 0) return a;
        else return gcd (b, a%b);
    }
    
    // helper method
    public static int lcm (int a, int b) {
        int gcd = gcd (a, b);
        return (a * b) / gcd;
    }

    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int t = in.nextInt();
        for(int a0 = 0; a0 < t; a0++){
            int n = in.nextInt();
            // starting loop to get lcm
            int temp_lcm = 1;
            for (int i = 2; i <= n; i++) {
                temp_lcm = lcm (temp_lcm, i);
            }
             // ending loop to get final answer below
            System.out.println(temp_lcm);

        }
    }
}