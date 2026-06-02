from collections.abc import Callable

def myFilter(xs: list[int], pred: Callable[[int], bool]) -> list[int]:
    result: list[int] = []
    for x in xs:
        if pred(x):
            result.append(x)
    return result

def myReject(xs: list[int], pred: Callable[[int], bool]) -> list[int]:
    result: list[int] = []
    for x in xs:
        if not pred(x):
            result.append(x)
    return result

if __name__ == "__main__":
    nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    evens = myFilter(nums, lambda x: x % 2 == 0)
    greater_than_5 = myFilter(nums, lambda x: x > 5)
    negatives = myFilter(nums, lambda x: x < 0)
    all_nums = myFilter(nums, lambda x: True)
    none = myFilter(nums, lambda x: False)
    print("Original:", nums)
    print("Evens:", evens)
    print("Greater than 5:", greater_than_5)
    print("Negatives:", negatives)
    print("All:", all_nums)
    print("None:", none)
    print(" Opposite: ")
    evensRej = myReject(nums, lambda x: x % 2 == 0)
    greater_than_5Rej = myReject(nums, lambda x: x > 5)
    negativesRej = myReject(nums, lambda x: x < 0)
    all_numsRej = myReject(nums, lambda x: True)
    noneRej = myReject(nums, lambda x: False)
    print("Original:", nums)
    print("Odds:", evensRej)
    print("Less than 5:", greater_than_5Rej)
    print("Positives:", negativesRej)
    print("None:", all_numsRej)
    print("All:", noneRej)