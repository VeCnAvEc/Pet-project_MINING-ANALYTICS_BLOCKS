use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::blockdata::script::Instruction;

#[derive(Debug)]
pub struct ParsedScriptSig {
    pub block_height: u32,
    pub guessed_miner: String,
    pub timestamp_sec: Option<u64>,
    pub extra_nonce: Vec<u8>,
    pub coinbase_raw: Vec<u8>,
    pub raw_pushes: Vec<Vec<u8>>,
}

impl ParsedScriptSig {
    pub fn from(script: &ScriptBuf) -> Option<Self> {
        let mut raw_pushes = Vec::new();

        for ins in script.instructions() {
            match ins {
                Ok(Instruction::PushBytes(bytes)) => {
                    raw_pushes.push(bytes.as_bytes().to_vec());
                }
                _ => continue, // пропускаем OP-коды
            }
        }

        if raw_pushes.len() < 2 {
            return None; // слишком мало push'ей
        }

        // 0: block_height (LE)
        let block_height = {
            let raw = &raw_pushes[0];
            let mut padded = [0u8; 4];
            padded[..raw.len()].copy_from_slice(raw);
            u32::from_le_bytes(padded)
        };

        // 1: timestamp_sec (опционально)
        let timestamp_sec = {
            if raw_pushes.len() > 1 && raw_pushes[1].len() <= 8 {
                let mut padded = [0u8; 8];
                padded[..raw_pushes[1].len()].copy_from_slice(&raw_pushes[1]);
                Some(u64::from_le_bytes(padded))
            } else {
                None
            }
        };

        // 2 или 1: guessed_miner + extra_nonce
        let label_idx = if timestamp_sec.is_some() { 2 } else { 1 };

        let (guessed_miner, extra_nonce) = {
            let combined = &raw_pushes.get(label_idx)?;
            let split_at = combined.iter()
                .position(|&b| !b.is_ascii_graphic() && b != b' ') // найдём конец printable
                .unwrap_or(combined.len());

            let label = String::from_utf8_lossy(&combined[..split_at]).trim().to_string();
            let rest = combined[split_at..].to_vec();
            (label, rest)
        };

        // Составим всё
        Some(ParsedScriptSig {
            block_height,
            guessed_miner,
            timestamp_sec,
            extra_nonce,
            coinbase_raw: script.to_bytes(),
            raw_pushes,
        })
    }
}