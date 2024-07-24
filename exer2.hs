divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n `div` 2)], n `mod` i == 0]

primes :: Int -> [Int]
primes n = [i | i <- [2..n], divisors i == []]

pythagorean :: Int -> [(Int, Int, Int)]
pythagorean n = [(a, b, c) | c <- [1..n], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2]

join :: [Char] -> [String] -> [Char]
join _ [] = "" 
join _ [x] = x
join j (x:xs) = x ++ j ++ join j xs

fact' :: Int -> Int
fact' n = foldl (*) 1 [1..n]

hailLen :: Int -> Int
hailLen n = hailTail 0 n
    where
        hailTail a 1 = a 
        hailTail a n = hailTail (a+1) (hailstone n)

-- Helper from last week
hailstone :: Integral a => a -> a
hailstone n 
    | even n = n `div` 2
    | otherwise = 3*n+1
