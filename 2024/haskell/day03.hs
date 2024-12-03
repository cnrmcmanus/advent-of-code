{-# LANGUAGE ViewPatterns #-}
import Data.List (stripPrefix)
import Data.Char (isDigit)
import Data.Bifunctor (first)

num :: Char -> String -> Maybe (Int, String)
num delim str = case span isDigit str of
  (ds, d:rest) | length ds `elem` [1..3] && delim == d -> Just (read ds, rest)
  _ -> Nothing

run :: Bool -> Bool -> String -> Int
run False _ (stripPrefix "do()" -> Just rest) = run False True rest
run False _ (stripPrefix "don't()" -> Just rest) = run False False rest
run force True (stripPrefix "mul(" -> Just rest) =
  case num ',' rest >>= (\(x, rest) -> first (x *) <$> num ')' rest) of
    Just (xy, rest) -> xy + run force True rest
    Nothing -> run force True rest
run force state (_ : rest) = run force state rest
run _ _ [] = 0

main :: IO ()
main = do
  input <- getContents
  print (run True True input)
  print (run False True input)
