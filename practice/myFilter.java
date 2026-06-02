package practice;

import java.util.List;
import java.util.function.IntPredicate;
import java.util.ArrayList;

public class myFilter {
    public static List<Integer> myFilter(List<Integer> xs, IntPredicate keep) {
        List<Integer> result = new ArrayList<>();
        for (int x: xs) {
            if (keep.test(x)) {
                result.add(x);
            }
        }
        return result;
    }

    public static List<Integer> myReject(List<Integer> xs, IntPredicate keep) {
        List<Integer> result = new ArrayList<>();
        for (int x: xs) {
            if (!keep.test(x)) {
                result.add(x);
            }
        }
        return result;
    }

    public static void main(String[] args) {
        List<Integer> nums = List.of(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        List<Integer> evens = myFilter(nums, x -> x % 2==0);
        List<Integer> greaterThan5 = myFilter(nums, x -> x > 5);
        List<Integer> negatives = myFilter(nums, x -> x < 0);
        List<Integer> all = myFilter(nums, x -> true);
        List<Integer> none = myFilter(nums, x -> false);
        System.out.println("Original: " + nums);
        System.out.println("Evens: " + evens);
        System.out.println("Greater than 5: " + greaterThan5);
        System.out.println("Negatives: " + negatives);
        System.out.println("All: " + all);
        System.out.println("None: " + none);
        System.out.println(" Opposite: ");
        List<Integer> evensRej = myReject(nums, x -> x % 2==0);
        List<Integer> greaterThan5Rej = myReject(nums, x -> x > 5);
        List<Integer> negativesRej = myReject(nums, x -> x < 0);
        List<Integer> allRej = myReject(nums, x -> true);
        List<Integer> noneRej = myReject(nums, x -> false);
        System.out.println("Original: " + nums);
        System.out.println("Evens: " + evensRej);
        System.out.println("Greater than 5: " + greaterThan5Rej);
        System.out.println("Negatives: " + negativesRej);
        System.out.println("All: " + allRej);
        System.out.println("None: " + noneRej);
    }
}