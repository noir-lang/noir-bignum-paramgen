use std::env;

use noir_bignum_paramgen::{bn_instance_from_string, bn_runtime_instance_from_string, bn_limbs_from_string, redc_limbs_from_string};
/**
 * @brief given a big number command line argument (either base10 or base 16 e.g. "1234", "0xabcd", "abcd" are all valid), spit out BigNum parameters formatted for noir.  
 */
fn main() {

    let args: Vec<String> = env::args().collect();

    let valid_args = args.len() == 3 || args.len() == 4;

    let help_msg = "usage: noir-bignum-paramgen [instance, runtime_instance, limbs, redc_param] [bignum modulus in hex or decimal] ?[parameter name]
        instance: outputs a string that represents a bignum::BigNumInstance object
        runtime_instance: outputs a string that represents a bignum::runtime_bignum::BigNumInstance object
        limbs: outputs an array of Field elements that represents a BigNum object's 120-bit limbs
        redc_param: outputs an array of Field elements that represents a BigNumInstance Barret reduction parameter

EXAMPLE:
    noir-bignum-paramgen instance 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001 BLS12_381_Fq";
    if !valid_args
    {
        println!("{}",help_msg);
    }
    else
    {
        let instruction_type = &args[1];
        let input_number: String = args[2].clone();
        let mut name: String = String::from("BigNumInstance");
        if args.len() == 4 {
            name = args[3].clone();
        }
        let mut result =String::from(help_msg);
        match instruction_type.as_str() {
            "instance" => {
                result = bn_instance_from_string(input_number, name);
            },
            "runtime_instance" => {
                result = bn_runtime_instance_from_string(input_number, name);
            },
            "limbs" => {
                result = bn_limbs_from_string(input_number);
            },
            "redc_param" => {
               result = redc_limbs_from_string(input_number);
            },
            _ => {
            }
        }
        println!("{}", result.as_str());
    }
}



