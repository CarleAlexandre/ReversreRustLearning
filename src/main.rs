use std::env;
use std::fs;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::Write;

fn main()
{
    let args: Vec<String> = env::args().collect();
	let file_path = args[1].clone();

    println!("In file {}", file_path);

    let contents = fs::read(file_path)
		.expect("Should have been able to read the file");
	
	let n = contents.capacity();
	println!("n = {}", n);
	
	sleep(Duration::new(2, 0));	
	
	let now = Instant::now();
	let mut iter: u32 = 0;

	print!("{:08x?}\t", iter);
	let mut writer = File::create("output.dump").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("output.dump").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
	write!(&mut writer, "{:08x?}\t", iter);
	for n in 1 .. n
	{
		print!("{:02X?}", contents[n]);
		write!(&mut writer, "{:02X?}", contents[n]);
		if n %8 == 0
		{
			print!("\n");
			write!(&mut writer, "\n");
			iter += 1;
			print!("0x{:07x?}\t", iter);
			write!(&mut writer, "0x{:07x?}\t", iter);
		}
		else if n % 2 == 0
		{
			print!(" ");
			write!(&mut writer, " ");
		}
	}
	println!("\ntime elapsed {}", now.elapsed().as_secs());
}
