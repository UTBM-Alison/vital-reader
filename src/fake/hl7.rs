use anyhow::Result;
use std::io::Write;
use std::time::Duration;

pub struct Hl7Generator;

impl Hl7Generator {
    pub fn send<W: Write>(port: &mut W) -> Result<()> {
        println!("Sending comprehensive HL7 messages (Medical devices simulation)...\n");

        let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();

        // Send all HL7 messages
        Self::send_header(port, &timestamp)?;
        Self::send_patient_info(port)?;
        Self::send_ge_monitor_data(port, &timestamp)?;
        Self::send_drager_ventilator_data(port, &timestamp)?;
        Self::send_drager_humidifier_data(port, &timestamp)?;

        println!("\n✓ Complete HL7 message sent with all modules!");
        Ok(())
    }

    fn send_header<W: Write>(port: &mut W, timestamp: &str) -> Result<()> {
        let msh = format!(
            "MSH|^~\\&|GE_MONITOR|ICU_01|VITAL_REC|HOSPITAL|{}||ORU^R01|MSG{:06}|P|2.5\r",
            timestamp, 1
        );
        port.write_all(msh.as_bytes())?;
        port.flush()?;
        println!("Sent: {}", msh.trim());
        std::thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    fn send_patient_info<W: Write>(port: &mut W) -> Result<()> {
        let pid = "PID|1||123456^^^HOSPITAL^MR||DOE^JOHN^A||19800515|M|||123 MAIN ST^^PARIS^^75001^FR||(33)123456789||FR|M|CAT|||123-45-6789\r";
        port.write_all(pid.as_bytes())?;
        port.flush()?;
        println!("Sent: {}", pid.trim());
        std::thread::sleep(Duration::from_millis(100));

        let pv1 = "PV1|1|I|ICU^101^01^HOSPITAL||||123456^SMITH^ROBERT^A^^^DR|||SUR||||ADM|||123456^SMITH^ROBERT^A^^^DR||V123456|||||||||||||||||||||||||20250103080000\r";
        port.write_all(pv1.as_bytes())?;
        port.flush()?;
        println!("Sent: {}", pv1.trim());
        std::thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    fn send_ge_monitor_data<W: Write>(port: &mut W, timestamp: &str) -> Result<()> {
        let obr = format!("OBR|1|ORD123456|RES123456|VS^VITAL SIGNS^LOCAL|||{}||||||||123456^SMITH^ROBERT^A^^^DR\r", timestamp);
        port.write_all(obr.as_bytes())?;
        port.flush()?;
        println!("Sent: {}", obr.trim());
        std::thread::sleep(Duration::from_millis(100));

        let observations = vec![
            format!("OBX|1|NM|8867-4^Heart Rate^LN||72|bpm^beats/min^UCUM|60-100|N|||F|||{}||GE_MONITOR^ECG_MODULE\r", timestamp),
            format!("OBX|2|ST|8884-9^ECG Rhythm^LN||NSR|||||N|||F|||{}||GE_MONITOR^ECG_MODULE\r", timestamp),
            format!("OBX|3|NM|9279-1^Respiratory Rate^LN||16|/min^per minute^UCUM|12-20|N|||F|||{}||GE_MONITOR^RESP_MODULE\r", timestamp),
            format!("OBX|4|NM|8480-6^Systolic BP^LN||125|mm[Hg]^millimeter of mercury^UCUM|90-140|N|||F|||{}||GE_MONITOR^PNI_MODULE\r", timestamp),
            format!("OBX|5|NM|8462-4^Diastolic BP^LN||78|mm[Hg]^millimeter of mercury^UCUM|60-90|N|||F|||{}||GE_MONITOR^PNI_MODULE\r", timestamp),
            format!("OBX|6|NM|8478-0^Mean BP^LN||93|mm[Hg]^millimeter of mercury^UCUM|70-105|N|||F|||{}||GE_MONITOR^PNI_MODULE\r", timestamp),
            format!("OBX|7|NM|2708-6^Oxygen Saturation^LN||98|%^percent^UCUM|95-100|N|||F|||{}||GE_MONITOR^SPO2_MODULE\r", timestamp),
            format!("OBX|8|NM|8889-8^Pulse Rate^LN||73|bpm^beats/min^UCUM|60-100|N|||F|||{}||GE_MONITOR^SPO2_MODULE\r", timestamp),
            format!("OBX|9|NM|8310-5^Body Temperature^LN||36.8|Cel^degree Celsius^UCUM|36.0-37.5|N|||F|||{}||GE_MONITOR^TEMP_MODULE\r", timestamp),
            format!("OBX|10|NM|76213-3^Arterial Systolic BP^LN||122|mm[Hg]^millimeter of mercury^UCUM|90-140|N|||F|||{}||GE_MONITOR^PI_MODULE\r", timestamp),
            format!("OBX|11|NM|76214-1^Arterial Diastolic BP^LN||75|mm[Hg]^millimeter of mercury^UCUM|60-90|N|||F|||{}||GE_MONITOR^PI_MODULE\r", timestamp),
            format!("OBX|12|NM|76215-8^Arterial Mean BP^LN||91|mm[Hg]^millimeter of mercury^UCUM|70-105|N|||F|||{}||GE_MONITOR^PI_MODULE\r", timestamp),
            format!("OBX|13|NM|8741-1^Cardiac Output^LN||5.2|L/min^liter/minute^UCUM|4.0-8.0|N|||F|||{}||GE_MONITOR^DC_MODULE\r", timestamp),
            format!("OBX|14|NM|8842-7^Cardiac Index^LN||2.8|L/min/m2^liter/minute/meter^2^UCUM|2.5-4.0|N|||F|||{}||GE_MONITOR^DC_MODULE\r", timestamp),
            format!("OBX|15|NM|20562-5^Stroke Volume^LN||72|mL^milliliter^UCUM|60-100|N|||F|||{}||GE_MONITOR^DC_MODULE\r", timestamp),
            format!("OBX|16|NM|19889-5^End Tidal CO2^LN||38|mm[Hg]^millimeter of mercury^UCUM|35-45|N|||F|||{}||GE_MONITOR^CO2_MODULE\r", timestamp),
            format!("OBX|17|NM|76270-3^Respiratory Rate CO2^LN||16|/min^per minute^UCUM|12-20|N|||F|||{}||GE_MONITOR^CO2_MODULE\r", timestamp),
            format!("OBX|18|NM|90371-9^BIS Index^LN||45|{{score}}^score^UCUM|40-60|N|||F|||{}||GE_MONITOR^BIS_MODULE\r", timestamp),
            format!("OBX|19|NM|90372-7^BIS Suppression Ratio^LN||5|%^percent^UCUM|0-10|N|||F|||{}||GE_MONITOR^BIS_MODULE\r", timestamp),
            format!("OBX|20|NM|93503-4^EEG Alpha Power^LN||28|%^percent^UCUM|20-40|N|||F|||{}||GE_MONITOR^EEG_MODULE\r", timestamp),
            format!("OBX|21|NM|93504-2^EEG Beta Power^LN||35|%^percent^UCUM|20-50|N|||F|||{}||GE_MONITOR^EEG_MODULE\r", timestamp),
            format!("OBX|22|NM|2713-6^Mixed Venous O2 Sat^LN||72|%^percent^UCUM|60-80|N|||F|||{}||GE_MONITOR^SVO2_MODULE\r", timestamp),
        ];

        for obs in observations {
            port.write_all(obs.as_bytes())?;
            port.flush()?;
            println!("Sent: {}", obs.trim());
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }

    fn send_drager_ventilator_data<W: Write>(port: &mut W, timestamp: &str) -> Result<()> {
        let obr_vent = format!("OBR|2|VENT123456|VRES123456|VENT^VENTILATION^LOCAL|||{}||||||||123456^SMITH^ROBERT^A^^^DR\r", timestamp);
        port.write_all(obr_vent.as_bytes())?;
        port.flush()?;
        println!("\nSent: {}", obr_vent.trim());
        std::thread::sleep(Duration::from_millis(100));

        let vent_data = vec![
            format!("OBX|23|NM|20112-9^Tidal Volume^LN||450|mL^milliliter^UCUM|400-600|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|24|NM|20139-2^Minute Volume^LN||7.2|L/min^liter/minute^UCUM|5.0-10.0|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|25|NM|76531-8^Peak Pressure^LN||22|cm[H2O]^centimeter of water^UCUM|15-30|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|26|NM|76530-0^Plateau Pressure^LN||18|cm[H2O]^centimeter of water^UCUM|10-25|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|27|NM|76248-9^PEEP^LN||5|cm[H2O]^centimeter of water^UCUM|3-10|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|28|NM|3150-0^FiO2^LN||40|%^percent^UCUM|21-100|N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
            format!("OBX|29|ST|76334-7^I:E Ratio^LN||1:2.5|||||N|||F|||{}||DRAGER^VENTILATOR\r", timestamp),
        ];

        for data in vent_data {
            port.write_all(data.as_bytes())?;
            port.flush()?;
            println!("Sent DRAGER Vent: {}", data.trim());
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }

    fn send_drager_humidifier_data<W: Write>(port: &mut W, timestamp: &str) -> Result<()> {
        let hum_data = vec![
            format!("OBX|30|NM|8310-5^Humidifier Temperature^LN||37.0|Cel^degree Celsius^UCUM|36.0-38.0|N|||F|||{}||DRAGER^HUMIDIFIER\r", timestamp),
            format!("OBX|31|NM|3143-5^Relative Humidity^LN||95|%^percent^UCUM|80-100|N|||F|||{}||DRAGER^HUMIDIFIER\r", timestamp),
            format!("OBX|32|NM|90401-4^Water Level^LN||85|%^percent^UCUM|50-100|N|||F|||{}||DRAGER^HUMIDIFIER\r", timestamp),
        ];

        for data in hum_data {
            port.write_all(data.as_bytes())?;
            port.flush()?;
            println!("Sent DRAGER Hum: {}", data.trim());
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }
}
