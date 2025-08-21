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
}