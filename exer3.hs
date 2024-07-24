import Data.Time.Calendar
import Data.Time.Calendar.OrdinalDate
import Data.Time.Calendar.WeekDate

merge :: Ord a => [a] -> [a] -> [a]
merge x [] = x
merge [] y = y
merge (x:xs) (y:ys)
    | x < y = x : merge xs (y:ys)
    | otherwise = y : merge (x:xs) ys

mergeSort :: Ord a => [a] -> [a]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort l) (mergeSort r) where (l, r) = splitAt(length xs `div` 2) xs

daysInYear :: Integer -> [Day]
daysInYear y = [jan1..dec31]
    where
        jan1 = fromGregorian y 1 1
        dec31 = fromGregorian y 12 31

isFriday :: Day -> Bool
isFriday day = weekday == 5
    where
        (_, _, weekday) = toWeekDate day

isPrimeDay :: Day -> Bool
isPrimeDay day = null (divisors (helper day))

helper :: Day -> Int
helper day = let (_, _, dayOfMonth) = toGregorian day in dayOfMonth

divisors :: Int -> [Int]
divisors n = [i | i <- [2..(n `div` 2)], n `mod` i == 0]

primeFridays :: Integer -> [Day]
primeFridays year = filter fridayHelper (daysInYear year)

fridayHelper :: Day -> Bool
fridayHelper day = isFriday day && isPrimeDay day

primeFridays2 :: Integer -> [Day]
primeFridays2 a = [ i | i <- daysInYear a, isPrimeDay i, isFriday i]