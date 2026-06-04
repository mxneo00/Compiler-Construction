

module Main (main, myMap) where

myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x: myMap f xs

main :: IO ()
main = do
  let nums = [1, 2, 3, 4, 5]
  main = print (myMap (\x -> x * x) nums)


