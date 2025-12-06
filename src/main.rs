use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mf_uid = &args[1];

    match convert_uid_to_arx(mf_uid) {
        Ok(val) => println!("ARX kortnummer: {}", val),
        Err(e) => println!("Feil: {}", e),
    }
}

fn convert_uid_to_arx(hex_uid: &str) -> Result<u32, Box<dyn std::error::Error>> {
    // Step 1 - remove spaces.
    let h: String = hex_uid.chars().filter(|c| !c.is_whitespace()).collect();

    // Step 2: Ensure length is exactly 8 hex chars (4 bytes)
    if h.len() != 8 {
        return Err("UID må være 8 hex tegn (4 bytes)".into());
    }
    
    // Step 3: Split into 4 bytes.
    let bytes = [
        &h[0..2],
        &h[2..4],
        &h[4..6],
        &h[6..8],
    ];

    // Step 4: Reverse byte order.
    let reverse_hex = format!("{}{}{}{}", bytes[3], bytes[2], bytes[1], bytes[0]);
    
    // step 5 - convert to decimal:
    let value = u32::from_str_radix(&reverse_hex, 16)?;

    Ok(value)
}
