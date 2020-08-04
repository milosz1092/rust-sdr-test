extern crate rtlsdr_mt;
extern crate sdr;
extern crate hound;

use sdr::IQ;
use sdr::FMDemod;

fn main() {
    rtlsdr_mt::devices().for_each(|d| {
        println!("{}", d.to_str().unwrap());
    });

    let devices_count = rtlsdr_mt::devices().count();

    if devices_count == 0 {
        println!("No devices found!");
        return
    }

    let (mut ctl, mut reader) = rtlsdr_mt::open(0).unwrap();

    
    ctl.enable_agc().unwrap();
    ctl.set_ppm(0).unwrap();
    ctl.set_bandwidth(170000).unwrap();


    ctl.set_center_freq(98_400_000).unwrap();

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 950000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("audio2.wav", spec).unwrap();
    let mut fdmod = FMDemod::new();

    reader.read_async(4, 32768, |bytes| {
        let mut complex_vector: Vec<IQ<i32>> = Vec::new();

        for i in 0..bytes.len() - 1 {
            if i % 2 != 0 {
                let re = i32::from(bytes[i]);
                let im = i32::from(bytes[i+1]);

                let cn = IQ { re: re, im: im };
                complex_vector.push(cn);
            }
        }

        let slice = &complex_vector[..];
        // println!("cv len: {}", complex_vector.len());

        let samples = fdmod.process(slice);

        for val in samples {
            writer.write_sample(val).unwrap();
        }

    }).unwrap();
}
