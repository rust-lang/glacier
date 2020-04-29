#![feature(llvm_asm)]

fn main()
{
    println!("Hello, world! {}", read_after_raising_any_exceptions());
}

#[inline(never)]
pub fn read_after_raising_any_exceptions() -> u16
{
	unsafe
	{
		// See <https://github.com/HJLebbink/asm-dude/wiki/FSTCW_FNSTCW>.
		let mut control_word: u16;
		llvm_asm!
		(
			"fstcw $0"
			:
				// Output constraints.
				"=m"(control_word)
			:
				// Input constraints.
			:
				// Clobbers.
			:
				// Options.
				"volatile"
		);
		control_word
	}
}
