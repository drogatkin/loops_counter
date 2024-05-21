package code;
import java.util.Random;
import java.util.concurrent.TimeUnit;

public class StrTest {
    static String ran_str( int length )
    {
        String SALTCHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        StringBuilder salt = new StringBuilder();
        Random rnd = new Random();
        while (salt.length() < length) { 
            int index = (int) (rnd.nextFloat() * SALTCHARS.length());
            salt.append(SALTCHARS.charAt(index));
        }
        return salt.toString();
    }

    public static void main(String...args) {
        long startTime = System.currentTimeMillis();
       String ran_vals[] = {ran_str(6), ran_str(25), ran_str(14), ran_str(235) , ran_str(155)};
       long res = 0;
       StringBuffer strBuf = new StringBuffer(1024 * 1024);
       for (int i=1; i < 100_000; ++i) {
           strBuf.setLength(0);
            if (i == 1)
               strBuf.append(ran_vals[0]);
            else if(i == 2) 
               strBuf.append(ran_vals[0]).append( ran_vals[1]);
            else if(i == 3) 
               strBuf.append(ran_vals[1]).append(ran_vals[2]).append(ran_vals[0]);
            else if(i == 4) 
               strBuf.append(ran_vals[3]).append(ran_vals[1]).append(ran_vals[2]).append(ran_vals[0]);
            else if(i == 5) 
               strBuf.append(ran_vals[3]).append(ran_vals[1]).append(ran_vals[2]).append(ran_vals[0]).append(ran_vals[4]);
            else {
                strBuf.append(ran_vals[4]);
                for (int j= 1; j < i; ++j )
                     strBuf.append(ran_vals[j % ran_vals.length]);
            }
            String str = strBuf.toString();
            int found = str.indexOf('A');
            if (found >= 0)
                res +=  found;
            found = str.lastIndexOf('z');
            if (found >= 0)
                res +=  found;   
       }
       System.out.printf("Result: %d \n", res);
       long time = System.currentTimeMillis() - startTime;
       System.out.printf("Executed in %02d:%02d:%03d%n", TimeUnit.MILLISECONDS.toMinutes(time), TimeUnit.MILLISECONDS.toSeconds(time), time%1000);
    }

}
