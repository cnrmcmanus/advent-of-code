parse :: String -> [[Int]]
parse = map (map read . words) . lines

derivatives :: [Int] -> [Int]
derivatives xs = zipWith (-) (tail xs) xs

derive :: [Int] -> [[Int]]
derive = takeWhile (not . all (== 0)) . iterate derivatives

predict :: [[Int]] -> Int
predict = sum . map last

main :: IO ()
main = do
  numbers <- parse <$> getContents
  print . sum . map (predict . derive) $ numbers
  print . sum . map (predict . derive . reverse) $ numbers
