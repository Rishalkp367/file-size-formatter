use std::env;
use std::fs;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: f64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes((size as f64) / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes((size as f64) / 1_000_000.0),
        _ => FileSize::Gigabytes((size as f64) / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn print_sizes(unit: &str, size: f64) {
    let bytes = match unit {
        "B"  => size,
        "KB" => size * 1_000.0,
        "MB" => size * 1_000_000.0,
        "GB" => size * 1_000_000_000.0,
        _ => {
            eprintln!("Unknown unit. Use B, KB, MB, or GB.");
            return;
        }
    };

    let sizes = Sizes {
        bytes,
        kilobytes: bytes / 1_000.0,
        megabytes: bytes / 1_000_000.0,
        gigabytes: bytes / 1_000_000_000.0,
    };

    println!("{:#?}", sizes);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path> <size> <unit>");
        return;
    }

    println!("args are: {:?}", args);

    //get the size of the file
    let file_path = &args[1];
    let metadata = fs::metadata(file_path).expect("Failed to read metadata");
    let size = metadata.len();
    println!("File Size is: {} bytes", size);
    let result = format_size(size);
    println!("{}", result);

    // get the size from the command line
    let input_size: f64 = args[2].parse().expect("Please provide a valid number for size");
    let unit = &args[3];
    print_sizes(unit, input_size);
}
