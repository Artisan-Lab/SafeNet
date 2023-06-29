use std::mem::MaybeUninit;
fn main() {
    static BYTE_TO_EMOJI: [String; 256] = {
        let mut m: [MaybeUninit<String>; 256] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..=255u8 {
            m[i as usize] = MaybeUninit::new(byte_to_emoji(i));
        }
        unsafe { mem::transmute::<_, [String; 256]>(m) }
    };
}
/*
https://github.com/lexbailey/bottom-rs/blob/6ec8f3b53a9d3076f9492059b88ae843dd8de727/build.rs#L22
*/