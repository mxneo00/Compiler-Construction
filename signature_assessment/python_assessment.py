
from typing import Callable, List


def my_map(nums: List[int], op: Callable[[int], int]) -> List[int]:
    """Apply ``op`` to every element of ``nums`` and return a new list.

    The original list is not modified.

    :param nums: source list of integers (left unchanged)
    :param op:   an int -> int operation to apply to each element
    :return:     a new list containing op(x) for each x in nums
    """
    result: list[int] = []
    for n in nums:
        if op(n) is not None:
            result.append(op(n))
    return result


def main() -> None:
    nums = [1, 2, 3, 4, 5]
    add5 = my_map(nums, lambda n: n + 5)
    print("Original: ", nums)
    print("Addition: ", add5)



if __name__ == "__main__":
    main()
