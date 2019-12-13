//! # mobile_code
//!
//! `mobile_code` is used for generating a code container to get, match 
//! and verify the code of a specific phone number 
//!

/// # Example
///
/// Create a container
///    let mut mobile_code = MobileCode::new(8);
/// Add a phone number   
///    let number:u64 = 4162215930;
///    mobile_code.add(number);   // Add a phone number
/// Add a phone list
///    let mut vlist: Vec<u64> = Vec::new();
///    vlist.push(4166471234);
///    vlist.push(9086079876);
///    mobile_code.add_list(&vlist);
///
/// Display the current codes
///    mobile_code.list();
///
/// Get the code of a phone
///    let code = mobile_code.get(&number);
///    println!("Code is {:?}", code);
/// Verify the code
///    println!("Checking verify: {}", mobile_code.verify(&number, code));
/// Test is_empty()
///    println!("Checking empty: {}", mobile_code.is_empty());
/// Remove a phone
///    mobile_code.remove(&number);
///    println!("Checking empty: {}", mobile_code.is_empty());
/// Check has()
///    println!("Checking has: {}", mobile_code.has(&number));
use std::collections::HashMap;
use rand::Rng;
use num_traits::Pow;

pub struct MobileCode {
    mobile_codes: HashMap<u64, u64>,
    length: u8,
}

impl MobileCode {
    pub fn new(len: u8) -> Self { // Create an empty container
        MobileCode {  
    	    mobile_codes: HashMap::new(),
            length: len,
        }
    }

    // Generate a random code 
    fn code_generator(&self) -> u64 {
        //number = number || 6;
        let low = Pow::pow(10u64, self.length - 1);
        let high = Pow::pow(10u64, self.length) + 1;
        rand::thread_rng().gen_range(low, high)
    }

    // List all the codes within the container
    pub fn list(&self) {
        for (key, value) in &self.mobile_codes {
           println!("{} -> {}", key, value);
        }
    }
 
    // Add a phone into the container 
    pub fn add(&mut self, mobile: u64) {
        self.mobile_codes.insert(mobile,  self.code_generator());
    }

    // Add a phone list into the container 
    pub fn add_list(&mut self, list: &Vec<u64>) {
        for mobile in list {   
            self.add(*mobile);
        }
    }

    // Get the code for this phone
    pub fn get(&self, mobile: &u64) -> Option<&u64> {
        self.mobile_codes.get(mobile)
    }      

    // Has the container included the phone?
    pub fn has(&self, mobile: &u64) -> bool {
        self.mobile_codes.contains_key(mobile)
    }

    // Verify the code against the phone stored in the container
    pub fn verify(&self, mobile: &u64, code: Option<&u64>) -> bool {
        if self.mobile_codes.contains_key(mobile) {
            //let v = self.mobile_codes.get(mobile).unwrap(); 
            self.mobile_codes.get(mobile) == code
        }
        else {
            false
        } 
    }

    // Remove the phone from the container
    pub fn remove(&mut self, mobile: &u64) {
        self.mobile_codes.remove(mobile);
    }

    pub fn clear(&mut self) {
	self.mobile_codes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.mobile_codes.is_empty()
    }
}

