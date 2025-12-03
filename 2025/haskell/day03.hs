import Data.Char
import Data.List

joltage :: Int -> [Int] -> Int
joltage n ds = fst $ foldl step (0, ds) [n-1,n-2..0] where
  step (x, ds) i = let (ds', end) = splitAt (length ds - i) ds; d = maximum ds'
                   in (x * 10 + d, tail (dropWhile (/= d) ds') ++ end)

main = mapM_ (print . sum) . transpose
     . map (sequenceA [joltage 2, joltage 12] . map digitToInt)
     . lines =<< getContents
