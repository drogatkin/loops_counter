// package hello;

/**
 * Generate all combination of K coin tosses.
 * K is passed as parameter, maximum value 30.
 * Calculate total number of "Head-Tail" combinations.
 * Calculate total number of "Head-Head" combinations.
 * Combination of 4 heads in a row gives 2 "Head-Head" combinations.
 */
import java.text.DecimalFormat;
import java.util.ArrayList;
import java.util.List;

public class HeadTail1 {
	// Constants
	static final int MAX_COIN_TOSSES     = 30;
	static final int DEFAULT_ARRAY_POWER = 10;
	static final int[] NO_MORE_VARIANTS  = null;
	static final int[] VARIANT_NOT_READY = new int[0];
	static final String HEAD_HEAD = "HEAD_HEAD";
	static final String HEAD_TAIL = "HEAD_TAIL";
	// Variants
	static volatile int count = 0;
	static volatile int maxvariants = 0;
	static volatile int arraySize = 2 << DEFAULT_ARRAY_POWER;
	static volatile List<int[]> variants = new ArrayList<>();
	// Results
	static volatile long totalHH = 0L;
	static volatile long totalHT = 0L;
	// Signals
	static volatile boolean doneVars = false;
	static volatile boolean doneHH   = false;
	static volatile boolean doneHT   = false;
	
	static boolean useThreads = true;
	
	public static void runLoop()
	{
		long start = System.nanoTime();
		if (useThreads) {
			new Thread(() -> {
				generateVariants();
				doneVars = true; // Generation is done
			}).start();

			new Thread(() -> {
				totalHH = calculateCombinations(0, 2);
				doneHH = true; // HH search is done
			}).start();

			new Thread(() -> {
				totalHT = calculateCombinations(1, 2);
				doneHT = true; // HT search is done
			}).start();

			while (!(doneHH && doneHT)) {
				// sleep 100 millisecond
				sleep(100);
			}
		}
		else
		{
			generateVariants();
			doneVars = true; // Generation is done
		
			totalHH = calculateCombinations(0, 2);
			doneHH = true; // HH search is done
		
			totalHT = calculateCombinations(1, 2);
			doneHT = true; // HT search is done
		}
		while (!(doneHH && doneHT))
		{
			// sleep 100 millisecond
			sleep(100);
		}
		long end = System.nanoTime();
		printResults(maxvariants, (end - start), totalHH, totalHT);
	}
	
	public static void generateVariants()
	{
		int variant = 0;
		for (int var = 0; var < maxvariants; var += arraySize) {
			int[] varArray = new int[arraySize];
			for (int index = 0; index < arraySize; index++)
			{
				varArray[index] = variant;
				variant++;
			}
			variants.add(varArray);
		}
	}
	
	public static long calculateCombinations(int pattern, int length)
	{
		long total = 0;
		int index = 0;
		// DEBUG int processedVariants = 0; // DEBUG
		for (int varArrayIndex = 0; varArrayIndex < maxvariants; )
		{	
			// get variants
			int[] varArray = getVariant(index);
			while (varArray == VARIANT_NOT_READY)
			{
				sleep(1); // sleep 1 millisecond
				varArray = getVariant(index);
			}
			if (varArray == NO_MORE_VARIANTS)
			{
				break;
			}
			index++;
			varArrayIndex += varArray.length;
			// find patterns
			for (int varIndex = 0; varIndex < varArray.length; varIndex++)
			{	
				int variant = varArray[varIndex];
				for (int shift = count - length; shift >= 0; shift--) {
					if (((variant >> shift) & 3) == pattern) {
						total++;
						shift--;
					}
				}
				// DEBUG processedVariants++; // DEBUG
			}
		}
		// DEBUG System.out.println("calculateCombinations: pattern: " + String.format("0x%02X", pattern) + " processed variants: " + processedVariants);
		return total;
	}
	
	public static int[] getVariant(int index)
	{
		if (index >= variants.size())
		{
			if (doneVars)
			{
				if (index >= variants.size())
				{
					return NO_MORE_VARIANTS;
				}
			}
			return VARIANT_NOT_READY;
		}
		return variants.get(index);
	}
	
	public static void printResults(int variants, long time, long totalHH, long totalHT)
	{
		DecimalFormat formatter = new DecimalFormat("#,###");
		System.out.println("Processed variants: " + formatter.format(variants)
			+ "\nTotal HEAD_HEAD combinations: " + formatter.format(totalHH)
			+ "\nTotal HEAD_TAIL combinations: " + formatter.format(totalHT)
			+ "\nTime: " + formatter.format(time) + " ns");
	}
	
	/**
	 * Sleep time in milliseconds
	 * @param sleepTime
	 */
	public static void sleep(long sleepTime)
	{
		try {
			Thread.sleep(sleepTime);
		} catch (InterruptedException e) {
			// Ignore it
		}
	}
	
	public static void main(String args[]) {
		if (args.length < 1)
		{
			System.err.println("Required parameter: count");
			System.exit(1);
		}
		System.out.println("Count: " + args[0]);
		try
		{
			count = Integer.parseInt(args[0]);
			if (count > MAX_COIN_TOSSES)
			{
				System.err.println("Count greater than " + MAX_COIN_TOSSES + " is not supported.");
				System.exit(2);
			}
			maxvariants = 1 << count;
			if (count <= DEFAULT_ARRAY_POWER)
			{
				arraySize = maxvariants;
			}
			runLoop();
		}
		catch (NumberFormatException nfex)
		{
			System.err.println("Invalid parameter: " + args[0]);
			System.exit(3);
		}
		catch (Exception anyex)
		{
			System.err.println("Internal error: " + anyex.getMessage());
			anyex.printStackTrace();
			System.exit(4);
		}
	}
}
