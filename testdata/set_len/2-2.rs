/*
https://github.com/ddxl/rustdesk/blob/b17bda9a55fd7544ffeade0ba6dcec2f103d34cd/src/server/audio_service.rs#L357
*/

fn test_pulse() {
        use libpulse_binding as pulse;
        use libpulse_simple_binding as psimple;
        let spec = pulse::sample::Spec {
            format: pulse::sample::SAMPLE_FLOAT32NE,
            channels: 2,
            rate: 24000,
        };
        let hspec = hound::WavSpec {
            channels: spec.channels as _,
            sample_rate: spec.rate as _,
            bits_per_sample: (4 * 8) as _,
            sample_format: hound::SampleFormat::Float,
        };
        const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
        let mut writer =
            hound::WavWriter::create(PATH, hspec).expect("Could not create hsound writer");
        let device = crate::platform::linux::get_pa_monitor();
        let s = psimple::Simple::new(
            None,                             // Use the default server
            "Test",                           // Our application’s name
            pulse::stream::Direction::Record, // We want a record stream
            Some(&device),                    // Use the default device
            "Test",                           // Description of our stream
            &spec,                            // Our sample format
            None,                             // Use default channel map
            None,                             // Use default buffering attributes
        )
        .expect("Could not create simple pulse");
        let mut out: Vec<u8> = Vec::with_capacity(1024);
        unsafe {
            out.set_len(out.capacity());
        }
        for _ in 0..600 {
            s.read(&mut out).expect("Could not read pcm");
            let out2 =
                unsafe { std::slice::from_raw_parts::<f32>(out.as_ptr() as _, out.len() / 4) };
            for v in out2 {
                writer.write_sample(*v).ok();
            }
        }
        println!("{:?} {}", device, out.len());
        writer.finalize().expect("Could not finalize writer");
    }