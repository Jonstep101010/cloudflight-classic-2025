use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
	let (infile_content, mut outfile) = setup()?;
	for line in infile_content.lines().skip(1) {
		let distance_xy: Vec<i32> = line
			.split_whitespace()
			.next() // discard paces
			.expect("first two elements")
			.split(",")
			.map(|distance| distance.parse::<i32>().unwrap())
			.collect();
		for mut distance in distance_xy {
			if distance.abs() == 0 {
				dump_vec_to_file(&mut outfile, vec![0, 0, 0]);
				continue;
			}
			let direction = if distance > 0 { 1 } else { -1 };
			let mut sequence = vec![0];
			let mut speed: i32 = direction * 5;
			let mut time_to_slowdown = 0;
			while distance.abs() != time_to_slowdown {
				sequence.push(speed);
				distance -= direction;
				if speed.abs() != 1 && distance.abs() > time_to_slowdown {
					speed -= direction;
				}
				time_to_slowdown = (5 - speed.abs()).abs();
			}
			while speed.abs() != 5 {
				speed += direction;
				sequence.push(speed);
			}
			sequence.push(0);
			dump_vec_to_file(&mut outfile, sequence);
		}
		writeln!(outfile).unwrap();
	}
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

fn dump_vec_to_file(outfile: &mut std::fs::File, sequence: Vec<i32>) {
	let mut printseq = String::new();
	sequence.iter().for_each(|elem| {
		printseq.push(' ');
		printseq.push_str(&elem.to_string());
	});
	writeln!(outfile, "{}", printseq.trim_start()).unwrap();
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	#[rstest]
	#[case("level4_0_example.in")]
	#[case("level4_1_small.in")]
	#[case("level4_2_large.in")]
	fn level_4(#[case] input_file: &str) {
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
