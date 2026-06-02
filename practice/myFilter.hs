module Main where

import Data.List (intercalate)

myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs)
  | p x       = x : myFilter p xs
  | otherwise =     myFilter p xs

render :: [Int] -> String
render xs = "[" ++ intercalate ", " (map show xs) ++ "]"

main :: IO ()
main = do
  let nums  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
      evens = myFilter (\x -> x `mod` 2 == 0) nums   -- swap this predicate to change behavior
      gt5   = myFilter (\x -> x > 5) nums
  putStrLn ("input : " ++ render nums)
  putStrLn ("evens : " ++ render evens)
  putStrLn ("gt 5  : " ++ render gt5)
  putStrLn ("original after calls (unchanged): " ++ render nums)
