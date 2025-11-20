use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
	let (str, mut outfile) = setup()?;
	str.lines().skip(1).for_each(|line| {
		writeln!(
			&mut outfile,
			"{}",
			line.split_whitespace()
				.map(|elem| elem.parse::<u32>().expect("number"))
				.sum::<u32>()
		)
		.unwrap()
	});
	Ok(())
}

fn setup() -> Result<(String, std::fs::File), std::io::Error> {
	let filename: String = std::env::args().nth(1).expect("outfile to be provided");
	let mut infile = std::fs::File::open(format!("../{filename}")).expect("file not found");
	let mut str = String::new();
	infile.read_to_string(&mut str).expect("read failed!");
	let outfilename = filename.strip_suffix(".in").unwrap().to_owned() + ".out";
	let outfile = std::fs::File::create(outfilename)?;
	Ok((str, outfile))
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	#[rstest]
	#[case("level1_0_example.in")]
	#[case("level1_1_small.in")]
	#[case("level1_2_large.in")]
	fn level_1(#[case] input_file: &str) {
		// Run the program
		let output = std::process::Command::new("cargo")
			.arg("run")
			.arg("--")
			.arg(input_file)
			.output()
			.expect("failed to run cargo");
		assert!(output.status.success(), "program execution failed");

		// Compare output file with expected
		let out_filename = input_file.strip_suffix(".in").unwrap().to_string() + ".out";
		let generated_out =
			std::fs::read_to_string(&out_filename).expect("failed to read generated output");
		let expected_out = std::fs::read_to_string(format!("../{}", out_filename))
			.expect("failed to read expected output");

		assert_eq!(
			generated_out, expected_out,
			"output mismatch for {}",
			input_file
		);

		// Cleanup
		std::fs::remove_file(&out_filename).ok();
	}
}
