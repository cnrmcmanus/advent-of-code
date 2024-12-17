import Data.Bits (xor)
import Data.List (find, tails)

parse :: String -> (Integer, Integer, Integer, [Int])
parse input = (reg a, reg b, reg c, program) where
  [a, b, c, _, instructions] = lines input
  reg str = read (words str !! 2)
  program = read $ "[" ++ (words instructions !! 1) ++ "]"

execute :: [(Int, Int)] -> (Integer, Integer, Integer) -> [Int]
execute ((code, _):(literal, _):program) (a, b, c) = case code of
  0 -> execute program (a `div` 2^combo, b, c)
  1 -> execute program (a, b `xor` toInteger literal, c)
  2 -> execute program (a, combo `mod` 8, c)
  3 | a == 0 -> execute program (a, b, c)
  3 -> execute (dropWhile ((/= literal) . snd) program) (a, b, c)
  4 -> execute program (a, b `xor` c, c)
  5 -> fromInteger (combo `mod` 8) : execute program (a, b, c)
  6 -> execute program (a, a `div` 2^combo, c)
  7 -> execute program (a, b, a `div` 2^combo)
  8 -> []
  where combo = ([0..3] ++ [a, b, c]) !! literal

search :: [(Int, Int)] -> [[Int]] -> Integer -> Maybe Integer
search _ [] n = Just n
search program (goal:goals) n =
  check `find` [n * 8 + m | m <- [0..7]] >>= search program goals
  where check n = execute program (n, 0, 0) == goal

main :: IO ()
main = do
  (a, b, c, program) <- parse <$> getContents
  let endless = cycle . (`zip` [0..]) $ program ++ [8]
  putStrLn . tail . init . show $ execute endless (a, b, c)
  print . maybe 0 id $ search endless (tail . reverse . tails $ program) 0
