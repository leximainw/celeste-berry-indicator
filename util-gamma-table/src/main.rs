fn main() {
    let mut gamma_table: [u32; 256] = [0; 256];
    println!("const GAMMA_TABLE: [u32; 256] = [");
    for i in 0..256 {
        let value = i as f64 / 255.0;
        let gamma = f64::powf(value, 2.21);
        gamma_table[i] = f64::round(gamma * 4294967295.0) as u32;
        print!(" {:#010x},", gamma_table[i]);
        if i % 8 == 7 {
            println!();
        }
    }
    println!("];");
    println!();
    println!("const LOOKUP_TABLE: [u32; 256] = [");
    for i in 0..256 {
        let value = (i as f64 + 0.5) / 255.0;
        let gamma = f64::powf(f64::min(value, 255.0), 2.21);
        gamma_table[i] = f64::round(gamma * 4294967295.0) as u32;
        print!(" {:#010x},", gamma_table[i]);
        if i % 8 == 7 {
            println!();
        }
    }
    println!("];");
}
