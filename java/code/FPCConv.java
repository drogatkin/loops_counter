package code;

import java.util.stream.*;

public class FPCConv {
  final static int N = 10000000;
  public static void main  (String ... params) {
  
  IntStream.range(0, N).filter(value -> value % 2 == 0).map(value -> value + value).filter(value -> value > 2 * N - 100).forEach(System.out::println);
  }
  
}

