det a b c = b^2 - 4*a*c
quadsol1 a b c = (-b - sqrt (det a b c))/2*a
quadsol2 a b c = (-b + sqrt (det a b c))/2*a

-- Find third element with indexing operator
third_a (x:xs) = xs !! 1

-- Find third element with pattern matching
third_b [] = error "no elements"
third_b [x] = error "only one element"
third_b [x, y] = error "only two elements"
third_b (x:y:z:xs) = z

-- Find factorial using recursion
fact 0 = 1
fact n = n * fact(n-1)

-- Find next element in the hailstone sequence
hailstone n 
    | even n = n `div` 2
    | otherwise = 3*n+1


-- Recursively find the length of the hailstone sequence
hailLen 1 = 0
hailLen 0 = 0
hailLen x 
    | x == 1 = 0
    | even x = 1 + hailLen (div x 2)
    | otherwise = 1 + hailLen (3*x+1)