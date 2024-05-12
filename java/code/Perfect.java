package code;

import java.time.Instant;
import java.time.Duration;

public class Perfect {
    public static void main(String ...args) {
        Instant start = Instant.now();
        
        int num, sum;
        sum = 0;
        num = 20_000;
        
        for (int j = num; j >= 1; j--) {
            for ( int i = num - 1; i >= 1; i--) {
                if (num % i == 0) {
                    sum += i;
                }
            }
            if (num == sum) {
                System.out.printf("num: %d/ sum:  %d%n", num, sum);
            }
            sum = 0;
            num --;
       }
       
       Instant end = Instant.now();
       System.out.println(Duration.between(start, end));
    }
}