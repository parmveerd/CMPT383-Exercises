pascal :: Int -> [Int]
pascal 0 = [1]
pascal n = [1] ++ zipWith (+) prev (tail prev) ++ [1]
  where
    prev = pascal (n - 1)


addPair :: (Integer, Integer) -> Integer
addPair = uncurry (+)

withoutZeros :: (Eq a, Num a) => [a] -> [a]
withoutZeros = filter (/=0)


findElt :: (Eq t, Num a) => t -> [t] -> Maybe a
findElt _ [] = Nothing  
findElt x (y:ys) 
  | x == y = Just 0
  | otherwise = case findElt x ys of
                  Just i -> Just (i + 1) 
                  Nothing -> Nothing  
