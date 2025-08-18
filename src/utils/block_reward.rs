/// Расчет текущего вознаграждения за блок с учетом халвинга
pub struct BlockRewardCalculator;

impl BlockRewardCalculator {
    /// Начальное вознаграждение за блок (50 BTC в сатоши)
    const INITIAL_REWARD: i64 = 50_0000_0000; // 50 BTC в сатоши

    /// Высота блока, на которой происходит халвинг (каждые 210,000 блоков)
    const HALVING_INTERVAL: i64 = 210_000;

    /// Рассчитывает текущее вознаграждение за блок для заданной высоты
    pub fn calculate_block_reward(block_height: i64) -> i64 {
        let halving_count = block_height / Self::HALVING_INTERVAL;
        // Битовый сдвиг вправо '>>'.
        // Оператор сдвигает биты числа вправо на указанное количество позиций. Каждый свдиг вправо делит число на 2.
        Self::INITIAL_REWARD >> halving_count
    }

    /// Получает номер халвинга для заданной высоты блока
    pub fn get_halving_number(block_height: i64) -> i64 {
        block_height / Self::HALVING_INTERVAL
    }

    /// Проверяет, является ли блок халвинг-блоком
    pub fn is_halving_block(block_height: i64) -> bool {
        block_height % Self::HALVING_INTERVAL == 0
    }

    /// Получает информацию о следующем халвинге
    pub fn get_next_halving_info(current_height: i64) -> (i64, i64) {
        let current_halving = Self::get_halving_number(current_height);
        let next_halving_height = (current_halving + 1) * Self::HALVING_INTERVAL;
        let blocks_until_halving = next_halving_height - current_height;
        (next_halving_height, blocks_until_halving)
    }

    /// Получает историю халвингов
    pub fn get_halving_history() -> Vec<(u64, u64)> {
        vec![
            (0, 50_0000_0000),           // 0-й халвинг: 50 BTC
            (210_000, 25_0000_0000),      // 1-й халвинг: 25 BTC
            (420_000, 12_5000_0000),      // 2-й халвинг: 12.5 BTC
            (630_000, 6_2500_0000),       // 3-й халвинг: 6.25 BTC
            (840_000, 3_1250_0000),       // 4-й халвинг: 3.125 BTC
            (1_050_000, 1_5625_0000),     // 5-й халвинг: 1.5625 BTC
            (1_260_000, 781_2500),        // 6-й халвинг: 0.78125 BTC
            (1_470_000, 390_6250),        // 7-й халвинг: 0.390625 BTC
            (1_680_000, 195_3125),        // 8-й халвинг: 0.1953125 BTC
            (1_890_000, 976_562),         // 9-й халвинг: 0.09765625 BTC
            (2_100_000, 488_281),         // 10-й халвинг: 0.048828125 BTC
        ]
    }
}