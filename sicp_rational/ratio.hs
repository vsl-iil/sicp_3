{- HLINT ignore "Use uncurry" -}

makeRat :: Integral a => a -> a -> (a, a)
makeRat _ 0 = error "Деление на ноль!"
makeRat x y = (x, y)

numer :: Integral a => (a, a) -> a
numer (x, _) = x

denom :: Integral a => (a, a) -> a
denom (_, 0) = error "Переданный кортеж не является рациональным числом - знаменатель равен 0!"
denom (_, y) = y

numer' :: Integral a => (a, a) -> a 
numer' pair = 
        let g = gcd (fst pair) (snd pair) 
        in fst pair `div` g

denom' :: Integral a => (a, a) -> a 
denom' pair =
        let g = gcd (fst pair) (snd pair)
        in snd pair `div` g

numerOrDenom :: (Integral a, Integral b) => (a, a) -> b -> a 
numerOrDenom pair index
                      | index == 0 = fst pair `div` g 
                      | index == 1 = snd pair `div` g 
                      | otherwise  = error "Неверный индекс!"
                      where g = gcd (fst pair) (snd pair)
