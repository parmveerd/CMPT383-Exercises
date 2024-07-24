import GHC.Real

myIterate :: (a -> a) -> a -> [a]
myIterate x y = y : myIterate x (x y)

mySplitAt :: Int -> [a] -> ([a], [a])
mySplitAt _ [] = ([], [])
mySplitAt n (x:xs)
  | n > 0 = (x : ys, zs)
  | otherwise = ([], x:xs)
  where (ys, zs) = mySplitAt (n - 1) xs

-- rationalSum :: Int -> [Ratio Int]
rationalSum :: Integral a => a -> [Ratio a]
rationalSum n = [x % y | x <- [1..n-1], y <- [1..n-1], x + y == n]

rationalSumLowest :: Int -> [Ratio Int]
rationalSumLowest n = [ x % (n - x) | x <- [1..n-1], check x (n - x) ]
  where
    check a b = gcd a b == 1

---------------------------------------------------------------------------------------
rationalSumLowest2 :: Integral a => a -> [Ratio a]
rationalSumLowest2 n = [x % y| x <- [1..n-1], y <- [1..n-1], x + y == n, gcd x y == 1]
---------------------------------------------------------------------------------------


rationals :: [Ratio Int]
rationals = concatMap rationalSumLowest [1..]


sumFile :: IO ()
sumFile = do
    content <- readFile "input.txt"
    let num = map readInt (splitAtSeparator '\n' content)
        total = sum num
    print total

splitAtSeparator :: Eq a => a -> [a] -> [[a]]
splitAtSeparator sep [] = []
splitAtSeparator sep content = first : splitAtSeparator sep rest
    where
    first = takeWhile (/= sep) content
    firstlen = length first
    rest = drop (firstlen+1) content

readInt :: String -> Int
readInt = read
    