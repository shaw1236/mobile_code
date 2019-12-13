use mobile_code::MobileCode;

fn main() {
    // Create a container
    let mut mobile_code = MobileCode::new(8);
    
    let number:u64 = 4162215930;
    mobile_code.add(number);   // Add a phone number

    // Add a phone list
    let mut vlist: Vec<u64> = Vec::new();
    vlist.push(4166471234);
    vlist.push(9086079876);
    mobile_code.add_list(&vlist);

    // Display the current codes
    mobile_code.list(); 

    // Get the code of a phone
    let code = mobile_code.get(&number);
    println!("Code is {:?}", code);
    // Verify the code
    println!("Checking verify: {}", mobile_code.verify(&number, code));
    // is_empty() 
    println!("Checking empty: {}", mobile_code.is_empty());
    // Remove a phone
    mobile_code.remove(&number);
    println!("Checking empty: {}", mobile_code.is_empty());
    // Check has()
    println!("Checking has: {}", mobile_code.has(&number));
   
    // List all
    mobile_code.list();

    println!("Ok");
}
