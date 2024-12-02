import Data.List (sort, transpose)

main :: IO ()
main = do
  [xs, ys] <- transpose . map (map read . words) . lines <$> getContents
  print . sum $ zipWith (\x y -> abs (x - y)) (sort xs) (sort ys)
  print . sum $ map (\x -> x * length (filter (== x) ys)) xs
