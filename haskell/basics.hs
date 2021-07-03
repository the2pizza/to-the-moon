import Data.List (sort, groupBy)

-- sum 
add :: Int -> Int -> Int
add x y = x + y


-- "abc" "bca" -> true
-- "abc" "cdf" -> false
-- check anagram

isAnagram :: [Char] -> [Char] -> Bool 
isAnagram x y = sort x == sort y

-- grouping anagram
-- ["abc", "bca", "dfa", "afd"]
-- [["abc", bca"], ["dfa", "afd"]

groupAnagram :: [String] -> [[String]] 
groupAnagram = groupBy isAnagram     

data Tree = Leaf (Maybe Int) | Node { value :: Int, left :: Tree,  right :: Tree }
  deriving Show

takeN :: Int -> Tree -> Tree
takeN 0 t = Leaf Nothing
takeN n t = Node (value t) 
            (takeN (n-1) (left t)) 
            (takeN (n-1) (right t))

genTree :: Int -> Tree
genTree 0 = Leaf Nothing
genTree n = Node n 
            (genTree (n - 1)) 
            (genTree (n + 1))

takeNLevels n = takeN n (genTree n)

main :: IO() 
main = do 
  putStrLn(show (add 4 5))
  putStrLn(show (isAnagram "abc" "cba"))
  putStrLn(show (isAnagram "abc" "fba"))
  putStrLn(show (isAnagram "abb" "aba"))
  putStrLn(show (isAnagram "4fb" "f4b"))
  putStrLn(show (groupAnagram ["4fb", "f4b", "abc", "bca", "qwe", "ewq", "fghj"]))

  putStrLn(show (takeNLevels 2))
