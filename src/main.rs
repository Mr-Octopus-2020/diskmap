mod scanner;
mod structure;

#[cfg(test)]
mod tests;

use std::env;
use std::path::Path;
use structure::DiskMap;

fn main() {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
                eprintln!("USAGE: diskmap <path>");
                std::process::exit(1);
        }

        let target_path = Path::new(&args[1]);

        if !target_path.exists() {
                eprintln!("Error: Path '{}' does not exist.", target_path.display());
                std::process::exit(1);
        }

        println!("Scanning: {} ...", target_path.display());

        let map = DiskMap::from(target_path);

        println!("\n--- DiskMap Scan Results ---");
        map.display();

        if let Some(r_idx) = map.root {
                println!(
                        "\nTotal size: {} bytes across {} nodes",
                        map.nodes[r_idx as usize].size,
                        map.nodes.len()
                );
        }
}
