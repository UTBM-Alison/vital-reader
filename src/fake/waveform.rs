use anyhow::Result;
use std::io::Write;
use std::time::Duration;

pub struct WaveformGenerator;

impl WaveformGenerator {
    pub fn send<W: Write>(port: &mut W) -> Result<()> {
        println!("Sending binary waveform data (simulated ECG)...\n");

        for i in 0..100 {
            // Simple sine wave to simulate waveform
            let angle = (i as f32) * 0.1;
            let value = (angle.sin() * 127.0 + 128.0) as u8;

            // Binary packet: [STX, SEQ, VALUE, CHECKSUM, ETX]
            let seq = (i % 256) as u8;
            let checksum = seq.wrapping_add(value);
            let packet = vec![0x02, seq, value, checksum, 0x03];

            port.write_all(&packet)?;
            port.flush()?;

            if i % 10 == 0 {
                println!("Sent packet #{}: {:02X?}", i, packet);
            }

            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }
}
