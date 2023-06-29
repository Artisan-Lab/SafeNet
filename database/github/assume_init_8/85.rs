use std::mem::MaybeUninit;
pub static mut BUFFER: MaybeUninit<String> = MaybeUninit::uninit();
pub fn char_from_input(keyboard_input: &KeyboardInput) -> Option<char>
{
	unsafe
	{
		let c = if crate::SETTINGS.layout == 1
		{
			qwerty::char_from_input(keyboard_input)
		}
		else
		{
			azerty::char_from_input(keyboard_input)
		};
		if let Some(key) = c
		{
			let buf = BUFFER.assume_init_mut();
			buf.push(key);
		}
		c
	}
}
/*
https://github.com/elsOS-dev/elsOS/blob/5b7a8c5815c117d94bb560c0fabc8ae171765fc6/src/keyboard/mod.rs#L41
*/