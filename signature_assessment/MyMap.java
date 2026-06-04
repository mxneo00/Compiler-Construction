package signature_assessment;

import java.util.ArrayList;
import java.util.List;
import java.util.function.IntUnaryOperator;


public class MyMap 
{

    /**
     * Applies {@code op} to every element of {@code input} and returns a new
     * list with the results. The original list is never modified.
     *
     * @param input the source list of integers (left unchanged)
     * @param op    the Int -> Int operation to map over each element
     * @return a new list containing op(x) for each x in input
     */
    public static List<Integer> myMap(List<Integer> input, IntUnaryOperator op) 
    {
        List<Integer> result = new ArrayList<>();
        for (int n: input) {
            if (op != null) {
                result.add(op.applyAsInt(n));
            }
        }
        return result;
    }

    public static void main(String[] args) 
    {
        List<Integer> nums = List.of(1, 2, 3, 4, 5);
        List<Integer> add5 = myMap(nums, n -> n + 5);
        System.out.println("Original: " + nums);
        System.out.println("Add 5: " + add5);
       
    }
}
