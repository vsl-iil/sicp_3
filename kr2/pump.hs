-- Функциональная модель насоса
-- пустая функция nas -> nas_C, nas_R, nas_P
-- 1. передать в осн.функцию набор функций, описывающий варианты
-- 2. задать конкретные параметры
-- В зависимости от переданных параметров ф. возвращает неизвестный
-- Пр. F(R, I) -> U, F(I, U) -> R ...
-- Параметры: мощность, КПД

type Func = (Float -> Float)
g = 9.81

pump :: Func -> Func -> Maybe Float -> Maybe Float -> Float
pump funcPower funcEffic (Just kpd) Nothing = funcPower kpd
pump funcPower funcEffic Nothing (Just power) = funcEffic power

pumpEfficiencyC :: Float -> Float -> Float -> Float -> Float 
pumpEfficiencyC napor plotn delivery power = delivery*napor*(plotn/3670)*power

pumpPowerC :: Float -> Float -> Float -> Float -> Float 
pumpPowerC napor plotn delivery kpd = kpd/(delivery*napor*plotn/3670)

deliveryC :: Float -> Float -> Float
deliveryC dp plotn = dp / (plotn*g)

main = do
        let effic = pumpEfficiencyC 1 1000 
        let power = pumpPowerC 1 1000 
        let delivery = deliveryC 10e5 1000
        let pumpC = pump (power delivery) (effic delivery) 

        putStrLn (show (pumpC (Just 0.75) Nothing)) 
        putStrLn (show (pumpC Nothing (Just 3e-2)))
