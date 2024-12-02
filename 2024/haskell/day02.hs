safe :: [Int] -> Bool
safe xs = compare close && (compare (<) || compare (>)) where
  compare f = all (uncurry f) $ xs `zip` tail xs
  close a b = abs (a - b) `elem` [1..3]

looselySafe :: [Int] -> Bool
looselySafe xs = safe xs || any safe gapped where
  gapped = [take i xs ++ drop (i + 1) xs | i <- [0 .. length xs - 1]]

main :: IO ()
main = do
  rows <- map (map read . words) . lines <$> getContents
  print $ length (filter safe rows)
  print $ length (filter looselySafe rows)
