use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
	let (str, mut outfile) = setup()?;
	str.lines().skip(1).for_each(|line| {
		let mut trip_len = 0;
		let mut trip_time = 0;
		line.split_whitespace()
			.map(|elem| elem.parse::<i32>().expect("number"))
			.for_each(|movement| {
				if movement == 0 {
					trip_time += 1;
				} else {
					trip_time += movement.abs();
					if movement > 0 {
						trip_len += 1;
					} else {
						trip_len -= 1;
					}
				}
			});
		writeln!(&mut outfile, "{trip_len} {trip_time}",).unwrap();
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
	#[case("level2_0_example.in")]
	#[case("level2_1_small.in")]
	#[case("level2_2_large.in")]
	fn level_2(#[case] input_file: &str) {
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
