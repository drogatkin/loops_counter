package code;

public class Counter {

    static volatile boolean stop = false;

    public static void runLoop(int sleepTime)

    {

        long count = 0, start = 0, end = 0;

        new Thread(() -> {

            try {

                Thread.sleep(sleepTime);

            } catch (InterruptedException e) {

                // Ignore it

            }

            stop = true; // Stop the loop

        }).start();

        start = System.currentTimeMillis();

        for (; count >= 0; count++) {

            if (stop) break;

        }

        end = System.currentTimeMillis();

        System.out.println("start: " + start + "\n  end: " + end + "\ncount: " + String.format("%,d", count));

    }

    public static void main(String args[]) {

        runLoop(1000);

    }

}
