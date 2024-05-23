element :: Float -> Float 
element x = -(1 / x)

next :: Float -> Float 
next x = x + 2

combineFunctions :: (Float -> Float) -> (Float -> Float) -> (Float -> Float)
combineFunctions el next x = el x + el (next x)

wrap :: Float -> Float -> (Float -> Float) -> (Float -> Float) -> ((Float -> Float) -> (Float -> Float) -> (Float -> Float)) -> (Float -> Float)
wrap start end el next comb 
                          | start < end = wrap (start+1) end (comb el next) next comb 
                          | otherwise   = comb el next
