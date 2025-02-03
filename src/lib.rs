
// use base64ct::{Base64, Encoding};
use num_bigint::{BigUint, RandomBits};
use num_traits::Num;
use rand::Rng;

/**
 * @brief mirrors the noir BigNumInstance object, where each noir Field element is a BigUint element
 */
struct BNInstance {
    has_multiplicative_inverse: bool,
    modulus: Vec<BigUint>,
    double_modulus: Vec<BigUint>,
    modulus_u60: [Vec<BigUint>; 2],
    modulus_u60_x4: [Vec<BigUint>; 4],
    redc_param: Vec<BigUint>,
}

const BARRETT_REDUCTION_OVERFLOW_BITS: usize = 4;
/**
 * @brief compute the reduction parameter used in Barrett reduction
 *        redc param = 2 * ceil(log2(modulus))
 *                     _______________________
 *                            modulus
 */
pub fn compute_barrett_reduction_parameter(modulus: &BigUint) -> BigUint {
    let k = modulus.bits();
    let multiplicand = BigUint::new([1].to_vec()) << (k as usize * 2 + BARRETT_REDUCTION_OVERFLOW_BITS);
    let barrett_reduction_parameter: BigUint = (multiplicand) / modulus;


//    let x: &str = "0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb";
//    let x = bignum_from_string(x.to_string());

//     let xxx = x.clone() + x.clone() + x.clone();
//     let x = x.clone();
//     let xxx_squared = xxx * x;

//     let x_mul_redc = xxx_squared.clone() * barrett_reduction_parameter.clone();
//     let shifted = x_mul_redc >> (k + k + BARRETT_REDUCTION_OVERFLOW_BITS);

//     let quotient_mul_modulus = shifted * modulus.clone();

//     let mut remainder_initial =  xxx_squared.clone() -quotient_mul_modulus.clone();

//     if (remainder_initial.clone() >= modulus.clone())
//     {
//         remainder_initial = remainder_initial - modulus.clone();
//     }
//     // let ff = remainder_initial.as
//     let bytes = remainder_initial.to_bytes_be();
//    let encoded = hex::encode(&bytes);
//    let mut r: String = String::from("");
//    r += &format!("0x{}, ", hex::encode(&bytes));
//     println!("REMAINDER = {remainder_initial}");
//     println!("formatted = {r}");
    barrett_reduction_parameter
}

/**
 * @brief split a BigUint into a vector of 120-bit slices 
 */
pub fn split_into_120_bit_limbs(_input: &BigUint, num_bits: usize) -> Vec<BigUint> {
    let num_limbs: usize = (num_bits / 120) + (num_bits % 120 != 0) as usize;
    let mut input = _input.clone();
    let one: BigUint = BigUint::from(1 as u64);
    let mask: BigUint = (one.clone() << 120 as usize) - one.clone();

    let mut r: Vec<BigUint> = Vec::new();
    for _ in 0..num_limbs {
        let slice = input.clone() & mask.clone();
        input = input.clone() >> 120 as usize;
        r.push(slice);
    }
    r
}

/**
 * @brief split a BigUint into a vector of 60-bit slices 
 * 
 * @details the 60-bit slices represent a noir U60Repr object. NUM_CHUNKS reqpresents the size-multiplier of the U60Repr
 *          (either 2 or 4)
 */
pub fn split_into_60_bit_limbs<const NUM_CHUNKS: usize>(_input: &BigUint, num_bits: usize) -> [Vec<BigUint>; NUM_CHUNKS] {
    let num_limbs: usize = (num_bits / 120) + (num_bits % 120 != 0) as usize;
     let mut input = _input.clone();
    let one = BigUint::from(1 as u64);
    let mask: BigUint = (one.clone() << 60 as usize) - one.clone();

    const INNER: Vec<BigUint> = Vec::new();
    let mut r: [Vec<BigUint>; NUM_CHUNKS] = [INNER; NUM_CHUNKS];
    for i in 0..(num_limbs as u64 * NUM_CHUNKS as u64) {
        let slice = input.clone() & mask.clone();
        input = input.clone() >> 60 as usize;
        let idx: usize = (i / num_limbs as u64) as usize;
        r[idx].push(slice);
    }
    r
}

/**
 * @brief given a modulus BigUint, compute a BNInstance object
 */
fn compute_bn_instance_parameters(modulus: &BigUint, num_bits: usize) -> BNInstance {
    let modulus_limbs = split_into_120_bit_limbs(&modulus, num_bits);
    let double_modulus = compute_double_modulus(&modulus, num_bits);
    let modulus_u60: [Vec<BigUint>; 2] = split_into_60_bit_limbs(&modulus, num_bits);
    let modulus_u60_x4: [Vec<BigUint>; 4] = split_into_60_bit_limbs(&modulus, num_bits);
    let has_multiplicative_inverse = is_prime(&modulus);
    let redc_param =
        split_into_120_bit_limbs(&compute_barrett_reduction_parameter(&modulus), num_bits);
    BNInstance {
        has_multiplicative_inverse,
        modulus: modulus_limbs,
        double_modulus,
        modulus_u60,
        modulus_u60_x4,
        redc_param,
    }
}



/**
 * @brief given a BNInstance, construct a string that represents noir code that defines a BigNumInstance object
 */
fn compute_bn_instance_string(num_bits: usize, instance: &BNInstance, name: String, underscore_split: bool) -> String {
    let BNInstance {
        has_multiplicative_inverse,
        modulus,
        double_modulus,
        modulus_u60,
        modulus_u60_x4,
        redc_param,
    } = instance;
    let num_limbs: usize = num_bits / 120 + (num_bits % 120 != 0) as usize;

    let bits: String = String::from(itoa::Buffer::new().format(num_bits as u64));
    let limbs: String = String::from(itoa::Buffer::new().format(num_limbs as u64));

    let mut params: String = String::from("Params");
    if underscore_split
    {
        params= String::from("_Params");
    }

    let tparam: String = name.clone() + &params;

    let mut param_str: String = String::from("");
    param_str += &String::from(format!("
use crate::params::BigNumParams;
use crate::params::BigNumParamsGetter;
use crate::utils::u60_representation::U60Repr;

pub struct {}{} {{}}

impl BigNumParamsGetter<{},{}> for {}{} {{
    fn get_params() -> BigNumParams<{}, {}> {{
        {}_PARAMS
    }}
    }}", name, params, limbs, bits, name, params, limbs, bits, name));

    let mut r: String = String::from("");
    r += &String::from(format!("pub global {}_PARAMS: BigNumParams<{}, {}> = BigNumParams {{
        has_multiplicative_inverse: {},
        modulus: [
            ",
        name.as_str(), limbs, bits, has_multiplicative_inverse)
    );
    for i in 0..num_limbs - 1 {
        let bytes = modulus[i].to_bytes_be();
        r += &format!("0x{}, ", hex::encode(&bytes));
    }
    let bytes: Vec<u8> = modulus[num_limbs - 1].to_bytes_be();
    r += &format!(
        "0x{}
        ],
        double_modulus: [
            ",
        hex::encode(&bytes)
    );
    for i in 0..num_limbs - 1 {
        let bytes = double_modulus[i].to_bytes_be();
        r += &format!("0x{}, ", hex::encode(&bytes));
    }
    let bytes: Vec<u8> = double_modulus[num_limbs - 1].to_bytes_be();
    r += &format!(
        "0x{}
        ],
        modulus_u60: U60Repr {{ limbs: [
            ",
        hex::encode(&bytes)
    );
    for j in 0..2 {
        for i in 0..num_limbs as usize - 1 {
            let bytes = modulus_u60[j][i].to_bytes_be();
            r += &format!("0x{}, ", hex::encode(&bytes));
        }
        let bytes = modulus_u60[j][num_limbs as usize - 1].to_bytes_be();
        if j == 0 {
            r += &format!("0x{}, ", hex::encode(&bytes));
        } else {
            r += &format!(
                "0x{}]}},
        modulus_u60_x4: U60Repr {{ limbs: [
            ",
                hex::encode(&bytes)
            );
        }
    }

    for j in 0..4 {
        for i in 0..num_limbs as usize - 1 {
            let bytes = modulus_u60_x4[j][i].to_bytes_be();
            r += &format!("0x{}, ", hex::encode(&bytes));
        }
        let bytes = modulus_u60_x4[j][num_limbs as usize - 1].to_bytes_be();
        if j < 3 {
            r += &format!("0x{}, ", hex::encode(&bytes));
        } else {
            r += &format!(
                "0x{}] }},
        redc_param: [
            ",
                hex::encode(&bytes)
            );
        }
    }
    for i in 0..num_limbs - 1 {
        let bytes = redc_param[i].to_bytes_be();
        r += &format!("0x{}, ", hex::encode(&bytes));
    }
    let bytes: Vec<u8> = redc_param[num_limbs - 1].to_bytes_be();
    r += &format!(
        "0x{}
        ]
}};
",
        hex::encode(&bytes)
    );
    param_str + "\n" + &r
}

/**
 * @brief given a BNInstance, construct a string that represents noir code that defines a runtime_bignum::BigNumInstance object
 */
fn compute_runtime_bn_instance_string(
    num_bits: usize,
    instance: BNInstance,
    name: String,
) -> String {
    let BNInstance {
        has_multiplicative_inverse,
        modulus,
        double_modulus: _,
        modulus_u60: _,
        modulus_u60_x4: _,
        redc_param,
    } = instance;
    let num_limbs: usize = num_bits / 120 + (num_bits % 120 != 0) as usize;

    let limbs: String = String::from(itoa::Buffer::new().format(num_limbs as u64));
    let tparam: String = name.clone() + &String::from("_Params");

    let mut r: String = String::from("");

    r += &String::from(format!(
        "let {}: BigNumInstance<{}, {}> = BigNumInstance::new(
    [
        ",
        name, limbs, tparam
    ));
    for i in 0..modulus.len() - 1 {
        let bytes = modulus[i].to_bytes_be();
        r += &String::from(format!("0x{}, ", hex::encode(&bytes)));
    }
    let bytes: Vec<u8> = modulus[modulus.len() - 1].to_bytes_be();
    r += &String::from(format!(
        "0x{}
    ],
    [
        ",
        hex::encode(&bytes)
    ));
    for i in 0..modulus.len() - 1 {
        let bytes = redc_param[i].to_bytes_be();
        r += &String::from(format!("0x{}, ", hex::encode(&bytes)));
    }
    let bytes: Vec<u8> = redc_param[modulus.len() - 1].to_bytes_be();
    r += &String::from(format!(
        "0x{}
    ]
);",
        hex::encode(&bytes)
    ));
    r
}

/**
 * @brief given a string that represents a BigNum, convert into a BigUint
 *
 * @param bignum_str = base10 or base16 representation. base16 can be prepended with "0x"
 */
pub fn bignum_from_string(
    bignum_str: String
) -> BigUint {
    let mut formatted_bignum_str = bignum_str.clone();
    let mut base16 = false;
    if bignum_str.starts_with("0x") {
        formatted_bignum_str = bignum_str.split_once("0x").unwrap().1.to_string();
        base16 = true;
    }
    if bignum_str.contains("a") | bignum_str.contains("b") | bignum_str.contains("c") |  bignum_str.contains("d") | bignum_str.contains("e") |  bignum_str.contains("f") |
        bignum_str.contains("A") | bignum_str.contains("B") | bignum_str.contains("C") |  bignum_str.contains("D") | bignum_str.contains("E") |  bignum_str.contains("F") {
        base16 = true;
    }
    let mut base: u32 = 10;
    if base16 {
        base = 16;
    }
    let modulus_result = BigUint::from_str_radix(formatted_bignum_str.as_str(), base);
    let modulus = match modulus_result {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing input integer: {error:?}"),
    };
    modulus
}

/**
 * @brief Compute noir code for a runtime_bignum::BigNumInstance given a modulus String
 */
pub fn bn_runtime_instance_from_string(
    modulus_str: String,
    name: String
) -> String {
    let modulus = bignum_from_string(modulus_str);
    let num_bits = modulus.bits();
    let result = compute_runtime_bn_instance_string(num_bits, compute_bn_instance_parameters(&modulus, num_bits), name);
    result
}

/**
 * @brief Compute noir code for a runtime_bignum::BigNumInstance given a modulus String
 */
pub fn bn_runtime_instance(
    modulus: BigUint,
    num_bits: usize,
    name: String
) -> String {
    let result = compute_runtime_bn_instance_string(num_bits, compute_bn_instance_parameters(&modulus, num_bits), name);
    result
}


/**
 * @brief Compute noir code for a bignum::BigNumInstance given a modulus String
 */
pub fn bn_instance_from_string(
    modulus_str: String,
    name: String,
    is_uint: bool
) -> String {
    let modulus = bignum_from_string(modulus_str);
    let num_bits = modulus.bits();
    let result = compute_bn_instance_string(num_bits, &compute_bn_instance_parameters(&modulus, num_bits), name.clone(), !is_uint);
    result
}

/**
 * @brief Compute noir code for an array of 120-bit limbs that represents a BigNum object
 */
pub fn bn_limbs_from_string(
    bn_str: String
) -> String {
    
    let bn = bignum_from_string(bn_str);
    bn_limbs(bn.clone(), bn.bits())
}

/**
 * @brief Compute noir code for an array of 120-bit limbs that represents a BigNum object
 */
pub fn bn_limbs(
    bn: BigUint,
    num_bits: usize
) -> String {
    let limbs = split_into_120_bit_limbs(&bn, num_bits);
    
    let mut r = String::from("[");
    for i in 0..limbs.len() - 1 {
        let bytes = limbs[i].to_bytes_be();
        r += &String::from(format!("0x{}, ", hex::encode(&bytes)));
    }
    let bytes: Vec<u8> = limbs[limbs.len() - 1].to_bytes_be();
    r += &String::from(format!("0x{}]", hex::encode(&bytes)));
    r
}

/**
 * @brief Compute noir code for an array of 120-bit limbs that represents a Barrett reduction parameter
 */
pub fn redc_limbs_from_string(
    bn_str: String,
) -> String {
    let bn = bignum_from_string(bn_str);
    redc_limbs(bn.clone(), bn.bits())
}


/**
 * @brief Compute noir code for an array of 120-bit limbs that represents a Barrett reduction parameter
 */
pub fn redc_limbs(
    bn: BigUint,
    num_bits: usize
) -> String {
    let redc_param = compute_barrett_reduction_parameter(&bn);
    let limbs = split_into_120_bit_limbs(&redc_param, num_bits);
    
    let mut r = String::from("[");
    for i in 0..limbs.len() - 1 {
        let bytes = limbs[i].to_bytes_be();
        r += &String::from(format!("0x{}, ", hex::encode(&bytes)));
    }
    let bytes: Vec<u8> = limbs[limbs.len() - 1].to_bytes_be();
    r += &String::from(format!("0x{}]", hex::encode(&bytes)));
    r
}


pub fn compute_double_modulus(modulus: &BigUint, num_bits: usize) -> Vec<BigUint> {
    let double_modulus = modulus.clone() + modulus.clone();

    let shift = BigUint::from(1 as u64) << 120;
    let mut limbs = split_into_120_bit_limbs(&double_modulus, num_bits);
    let num_limbs = limbs.len();
    limbs[0] += shift.clone();
    for i in 1..num_limbs - 1 {
        limbs[i] = limbs[i].clone() + shift.clone() - BigUint::from(1 as u64);
    }
    limbs[num_limbs - 1] = limbs[num_limbs - 1].clone() - BigUint::from(1 as u64);
    limbs
}


pub fn is_prime(modulus: &BigUint) -> bool { 
    // we implement a fermat primality test
    //pick a random number less than the modulus
    let mut rng = rand::thread_rng();
    let num_bits = modulus.bits();
    let a: BigUint = rng.sample(RandomBits::new(num_bits));
    let modulus_minus_1 = modulus.clone() - BigUint::from(1 as u64);
    let a_to_the_power = a.modpow(&modulus_minus_1, modulus);
    a_to_the_power == BigUint::from(1 as u64)
}