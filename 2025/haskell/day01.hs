import Control.Arrow
import Data.List

parse :: String -> Int
parse = uncurry (*) . ((\dir -> if dir == "L" then -1 else 1) *** read) . splitAt 1

turn :: Int -> Int -> (Int, [Int])
turn pos by = (newPos, [btoi (newPos == 0), abs passes - btoi leftStartZero + btoi leftEndZero])
  where (passes, newPos) = (pos + by) `divMod` 100
        (leftStartZero, leftEndZero) = (pos == 0 && by < 0, newPos == 0 && by < 0)
        btoi = fromEnum

main = mapM_ (print . sum) . transpose . snd
     . mapAccumL turn 50 . map parse . lines
     =<< getContents
