use anyhow::Result;
use std::io::Write;
use std::time::Duration;

pub struct VitalSignsGenerator;

impl VitalSignsGenerator {
    pub fn send<W: Write>(port: &mut W, num_samples: usize) -> Result<()> {
        println!("Sending vital signs data ({} samples)...\n", num_samples);

        for i in 1..=num_samples {
            let hr = 60 + (i % 20);
            let spo2 = 95 + (i % 5);
            let bp_sys = 110 + (i % 20);
            let bp_dia = 70 + (i % 10);
            let temp = 36.5 + (i as f32 * 0.1);

            let data = format!(
                "PATIENT_ID=12345|HR={}|SPO2={}|BP={}/{}|TEMP={:.1}|TIME={}\n",
                hr,
                spo2,
                bp_sys,
                bp_dia,
                temp,
                chrono::Local::now().format("%H:%M:%S")
            );

            port.write_all(data.as_bytes())?;
            port.flush()?;
            println!("Sent: {}", data.trim());
            std::thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }
}
