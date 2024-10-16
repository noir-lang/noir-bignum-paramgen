use std::env;
use std::io::prelude::*;
use num_bigint::BigUint;

fn bls12377_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the BLS12_377 curve generated in [[BCGMMW20, \"Zexe\"]]((https://eprint.iacr.org/2018/962).
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 377-bit (prime) field. The main feature of this curve is
//! that both the scalar field and the base field are highly 2-adic.
//! (This is in contrast to the BLS12_381 curve for which only the scalar field
//! is highly 2-adic.)
//!
//!
//! Curve information:
//! * Base field: q = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
//! * Scalar field: r =
//!   8444461749428370424248824938781546531375899335154063827935233455917409239041
//! * valuation(q - 1, 2) = 46
//! * valuation(r - 1, 2) = 47
//! * G1 curve equation: y^2 = x^3 + 1
//! * G2 curve equation: y^2 = x^3 + B, where
//!    * B = Fq2(0, 155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906)"))
}

fn bls12377_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the BLS12_377 curve generated in [BCGMMW20, \"Zexe\"](https://eprint.iacr.org/2018/962).
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 377-bit (prime) field. The main feature of this curve is
//! that both the scalar field and the base field are highly 2-adic.
//! (This is in contrast to the BLS12_381 curve for which only the scalar field
//! is highly 2-adic.)
//!
//!
//! Curve information:
//! * Base field: q = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
//! * Scalar field: r =
//!   8444461749428370424248824938781546531375899335154063827935233455917409239041
//! * valuation(q - 1, 2) = 46
//! * valuation(r - 1, 2) = 47
//! * G1 curve equation: y^2 = x^3 + 1
//! * G2 curve equation: y^2 = x^3 + B, where
//!    * B = Fq2(0, 155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906)
"))}

fn bls12381_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the BLS12_381 curve generated by [Sean Bowe](https://electriccoin.co/blog/new-snark-curve/).
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 381-bit (prime) field.
//! This curve was intended to replace the BN254 curve to provide a higher
//! security level without incurring a large performance overhead.
//!
//!
//! Curve information:
//! * Base field: q = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
//! * Scalar field: r =
//!   52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 32
//! * G1 curve equation: y^2 = x^3 + 4
//! * G2 curve equation: y^2 = x^3 + Fq2(4, 4)"))}
fn bls12381_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the BLS12_381 curve generated by [Sean Bowe](https://electriccoin.co/blog/new-snark-curve/).
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 381-bit (prime) field.
//! This curve was intended to replace the BN254 curve to provide a higher
//! security level without incurring a large performance overhead.
//!
//!
//! Curve information:
//! * Base field: q = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
//! * Scalar field: r =
//!   52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 32
//! * G1 curve equation: y^2 = x^3 + 4
//! * G2 curve equation: y^2 = x^3 + Fq2(4, 4)"))}
fn mnt4753_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the MNT4_753 curve generated in
//! [[BCTV14]](https://eprint.iacr.org/2014/595). The name denotes that it is a
//! Miyaji--Nakabayashi--Takano curve of embedding degree 4, defined over a
//! 753-bit (prime) field. The main feature of this curve is that its scalar
//! field and base field respectively equal the base field and scalar field of
//! MNT6_753.
//!
//! Curve information:
//! * Base field: q = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB117E776F218059DB80F0DA5CB537E38685ACCE9767254A4638810719AC425F0E39D54522CDD119F5E9063DE245E8001
//! * Scalar field: r = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB26C5C28C859A99B3EEBCA9429212636B9DFF97634993AA4D6C381BC3F0057974EA099170FA13A4FD90776E240000001
//! * valuation(q - 1, 2) = 15
//! * valuation(r - 1, 2) = 30
//! * G1 curve equation: y^2 = x^3 + ax + b, where
//!    * a = 2
//!    * b = 0x01373684A8C9DCAE7A016AC5D7748D3313CD8E39051C596560835DF0C9E50A5B59B882A92C78DC537E51A16703EC9855C77FC3D8BB21C8D68BB8CFB9DB4B8C8FBA773111C36C8B1B4E8F1ECE940EF9EAAD265458E06372009C9A0491678EF4
//! * G2 curve equation: y^2 = x^3 + Ax + B, where
//!    * A = Fq2 = (a * NON_RESIDUE, 0)
//!    * B = Fq2(0, b * NON_RESIDUE)
//!    * NON_RESIDUE = 13 is the quadratic non-residue used to conpub struct  the
//!      extension field Fq2"))}
fn mnt4753_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the MNT4_753 curve generated in
//! [[BCTV14]](https://eprint.iacr.org/2014/595). The name denotes that it is a
//! Miyaji--Nakabayashi--Takano curve of embedding degree 4, defined over a
//! 753-bit (prime) field. The main feature of this curve is that its scalar
//! field and base field respectively equal the base field and scalar field of
//! MNT6_753.
//!
//! Curve information:
//! * Base field: q = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB117E776F218059DB80F0DA5CB537E38685ACCE9767254A4638810719AC425F0E39D54522CDD119F5E9063DE245E8001
//! * Scalar field: r = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB26C5C28C859A99B3EEBCA9429212636B9DFF97634993AA4D6C381BC3F0057974EA099170FA13A4FD90776E240000001
//! * valuation(q - 1, 2) = 15
//! * valuation(r - 1, 2) = 30
//! * G1 curve equation: y^2 = x^3 + ax + b, where
//!    * a = 2
//!    * b = 0x01373684A8C9DCAE7A016AC5D7748D3313CD8E39051C596560835DF0C9E50A5B59B882A92C78DC537E51A16703EC9855C77FC3D8BB21C8D68BB8CFB9DB4B8C8FBA773111C36C8B1B4E8F1ECE940EF9EAAD265458E06372009C9A0491678EF4
//! * G2 curve equation: y^2 = x^3 + Ax + B, where
//!    * A = Fq2 = (a * NON_RESIDUE, 0)
//!    * B = Fq2(0, b * NON_RESIDUE)
//!    * NON_RESIDUE = 13 is the quadratic non-residue used to conpub struct  the
//!      extension field Fq2"))}
fn mnt6753_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the MNT6_753 curve generated in
//! [[BCTV14]](https://eprint.iacr.org/2014/595). The name denotes that it is a
//! Miyaji--Nakabayashi--Takano curve of embedding degree 6, defined over a
//! 753-bit (prime) field. The main feature of this curve is that its scalar
//! field and base field respectively equal the base field and scalar field of
//! MNT4_753.
//!
//! Curve information:
//! * Base field: q = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB26C5C28C859A99B3EEBCA9429212636B9DFF97634993AA4D6C381BC3F0057974EA099170FA13A4FD90776E240000001
//! * Scalar field: r = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB117E776F218059DB80F0DA5CB537E38685ACCE9767254A4638810719AC425F0E39D54522CDD119F5E9063DE245E8001
//! * valuation(q - 1, 2) = 30
//! * valuation(r - 1, 2) = 15
//! * G1 curve equation: y^2 = x^3 + ax + b, where
//!    * a = 11
//!    * b = 0x7DA285E70863C79D56446237CE2E1468D14AE9BB64B2BB01B10E60A5D5DFE0A25714B7985993F62F03B22A9A3C737A1A1E0FCF2C43D7BF847957C34CCA1E3585F9A80A95F401867C4E80F4747FDE5ABA7505BA6FCF2485540B13DFC8468A
//! * G2 curve equation: y^2 = x^3 + Ax + B, where
//!    * A = Fq3(0, 0, a)
//!    * B = Fq3(b * NON_RESIDUE, 0, 0)
//!    * NON_RESIDUE = 11 is the cubic non-residue used to conpub struct  the
//!      extension field Fq3"))}
fn mnt6753_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the MNT6_753 curve generated in
//! [[BCTV14]](https://eprint.iacr.org/2014/595). The name denotes that it is a
//! Miyaji--Nakabayashi--Takano curve of embedding degree 6, defined over a
//! 753-bit (prime) field. The main feature of this curve is that its scalar
//! field and base field respectively equal the base field and scalar field of
//! MNT4_753.
//!
//! Curve information:
//! * Base field: q = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB26C5C28C859A99B3EEBCA9429212636B9DFF97634993AA4D6C381BC3F0057974EA099170FA13A4FD90776E240000001
//! * Scalar field: r = 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB117E776F218059DB80F0DA5CB537E38685ACCE9767254A4638810719AC425F0E39D54522CDD119F5E9063DE245E8001
//! * valuation(q - 1, 2) = 30
//! * valuation(r - 1, 2) = 15
//! * G1 curve equation: y^2 = x^3 + ax + b, where
//!    * a = 11
//!    * b = 0x7DA285E70863C79D56446237CE2E1468D14AE9BB64B2BB01B10E60A5D5DFE0A25714B7985993F62F03B22A9A3C737A1A1E0FCF2C43D7BF847957C34CCA1E3585F9A80A95F401867C4E80F4747FDE5ABA7505BA6FCF2485540B13DFC8468A
//! * G2 curve equation: y^2 = x^3 + Ax + B, where
//!    * A = Fq3(0, 0, a)
//!    * B = Fq3(b * NON_RESIDUE, 0, 0)
//!    * NON_RESIDUE = 11 is the cubic non-residue used to conpub struct  the
//!      extension field Fq3"))}
fn pallas_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the prime-order curve Pallas, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Vesta, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Vesta.
//!
//!
//! Curve information:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32"))}
fn pallas_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the prime-order curve Pallas, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Vesta, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Vesta.
//!
//!
//! Curve information:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32"))}
fn vesta_fq_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the base field of the prime-order curve Vesta, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Pallas, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Pallas.
//!
//!
//! Curve information:
//! Vesta:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32"))}
fn vesta_fr_blurb()-> String { String::from(format!("//! Blurb sourced from https://github.com/arkworks-rs
//! This library implements the scalar field of the prime-order curve Vesta, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Pallas, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Pallas.
//!
//!
//! Curve information:
//! Vesta:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32"))}
fn empty_blurb()-> String { String::from(format!(""))}

use noir_bignum_paramgen::{bn_instance_from_string, bn_runtime_instance_from_string, bn_limbs_from_string, redc_limbs_from_string, bignum_from_string};

fn bignum_from_string_slices<const K: usize>(slices: [&str; K]) -> String {

  //  let scaling_factor = BigUint::from(1 as u64) << 120;
    let mut result: BigUint = BigUint::from(0 as u64);
    for i in 0..K {
        result = result << 120;
        let slice: BigUint = bignum_from_string(slices[K - 1 - i].to_string());
        result = result + slice;
    }
    let mut r =  String::from("0x");
   r.push_str(result.to_str_radix(16).as_str());
   r
}

fn generate_parameter_file<const K: usize>(modulus_slices: [&str; K], file_name: &str, field_name: &str, blurb: String, is_uint: bool) -> std::io::Result<()> {
    let mut str: String = String::from("./fields/");
    str.push_str(file_name);
    str.push_str(".nr");
    let mut file = std::fs::File::create(str)?;
    let mut file_text: String = String::from(blurb);
    let field_inst = bn_instance_from_string(bignum_from_string_slices(modulus_slices), field_name.to_string(), is_uint);
    file_text.push_str(field_inst.as_str());

    file.write_all(file_text.as_bytes())?;

    Ok(())
}

// fn generate_testparams() {
//     let modulus: [&str; 18] = [
//     "0x0000000000000000000000000000000000c0a197a5ae0fcdceb052c9732614fe",
//     "0x0000000000000000000000000000000000656ae034423283422243918ab83be3",
//     "0x00000000000000000000000000000000006bf590da48a7c1070b7d5aabaac678",
//     "0x00000000000000000000000000000000000cce39f530238b606f24b296e2bda9",
//     "0x000000000000000000000000000000000001e1fef9bb9c1c3ead98f226f1bfa0",
//     "0x0000000000000000000000000000000000ad8c1c816e12e0ed1379055e373abf",
//     "0x0000000000000000000000000000000000cebe80e474f753aa9d1461c435123d",
//     "0x0000000000000000000000000000000000aee5a18ceedef88d115a8b93c167ad",
//     "0x0000000000000000000000000000000000268ba83c4a65c4307427fc495d9e44",
//     "0x0000000000000000000000000000000000dd2777926848667b7df79f342639d4",
//     "0x0000000000000000000000000000000000f455074c96855ca0068668efe7da3d",
//     "0x00000000000000000000000000000000005ddba6b30bbc168bfb3a1225f27d65",
//     "0x0000000000000000000000000000000000591fec484f36707524133bcd6f4258",
//     "0x000000000000000000000000000000000059641b756766aeebe66781dd01d062",
//     "0x000000000000000000000000000000000058bc5eaff4b165e142bf9e2480eebb",
//     "0x0000000000000000000000000000000000667a3964f08e06df772ce64b229a72",
//     "0x00000000000000000000000000000000009c1fdb18907711bfe3e3c1cf918395",
//     "0x00000000000000000000000000000000000000000000000000000000000000b8"
//     ];
//     generate_parameter_file(modulus, "test2048", "test2048", empty_blurb(), false);
// }

fn generate_parameters_full() -> std::io::Result<()> {
    let bls381_fq: [&str; 4] = ["0xabfffeb153ffffb9feffffffffaaab", "0x4b84f38512bf6730d2a0f6b0f6241e", "0xea397fe69a4b1ba7b6434bacd76477", "0x1a0111" ];
    let bls381_fr: [&str; 3] = ["0xbda402fffe5bfeffffffff00000001", "0xa753299d7d483339d80809a1d80553", "0x73ed"];
    let bls377_fq: [&str; 4] = [ "0x0b5d44300000008508c00000000001", "0xd9f300f5138f1ef3622fba09480017", "0x4617c510eac63b05c06ca1493b1a22", "0x01ae3a"];
    let bls377_fr: [&str; 3] = [ "0xaa76fed00000010a11800000000001", "0x655e9a2ca55660b44d1e5c37b00159", "0x12ab"];
    let bn254_fq: [&str; 3] = [ "0x816a916871ca8d3c208c16d87cfd47", "0x4e72e131a029b85045b68181585d97", "0x3064"];
    let ed25519_fq: [&str; 3] = [ "0xffffffffffffffffffffffffffffed", "0xffffffffffffffffffffffffffffff", "0x7fff"];
    let ed25519_fr: [&str; 3] = ["0xdef9dea2f79cd65812631a5cf5d3ed", "0x14", "0x1000"];
    let mnt4753_fq: [&str; 7] = [ "0x9d54522cdd119f5e9063de245e8001", "0xcce9767254a4638810719ac425f0e3", "0x76f218059db80f0da5cb537e38685a", "0xe8a0ed8d99d124d9a15af79db117e7", "0x8fafed5eb7e8f96c97d87307fdb925", "0xc41110229022eee2cdadb7f997505b", "0x01c4c62d92"];
    let mnt4753_fr: [&str; 7] = [ "0xa099170fa13a4fd90776e240000001", "0xf97634993aa4d6c381bc3f0057974e", "0x28c859a99b3eebca9429212636b9df", "0xe8a0ed8d99d124d9a15af79db26c5c", "0x8fafed5eb7e8f96c97d87307fdb925", "0xc41110229022eee2cdadb7f997505b", "0x01c4c62d92"];
    let mnt6753_fq: [&str; 7] = [ "0xa099170fa13a4fd90776e240000001", "0xf97634993aa4d6c381bc3f0057974e", "0x28c859a99b3eebca9429212636b9df", "0xe8a0ed8d99d124d9a15af79db26c5c","0x8fafed5eb7e8f96c97d87307fdb925", "0xc41110229022eee2cdadb7f997505b","0x01c4c62d92"];
    let mnt6753_fr: [&str; 7] = ["0x9d54522cdd119f5e9063de245e8001", "0xcce9767254a4638810719ac425f0e3", "0x76f218059db80f0da5cb537e38685a", "0xe8a0ed8d99d124d9a15af79db117e7", "0x8fafed5eb7e8f96c97d87307fdb925", "0xc41110229022eee2cdadb7f997505b", "0x01c4c62d92"];
    let pallas_fq: [&str; 3] = [ "0x4698fc094cf91b992d30ed00000001", "0x22", "0x4000"];
    let pallas_fr: [&str; 3] = [ "0x4698fc0994a8dd8c46eb2100000001", "0x22", "0x4000"];
    let secp256k1_fq: [&str; 3] = [ "0xfffffffffffffffffffffefffffc2f", "0xffffffffffffffffffffffffffffff", "0xffff"];
    let secp256k1_fr: [&str; 3] = [ "0xaedce6af48a03bbfd25e8cd0364141", "0xfffffffffffffffffffffffffffeba","0xffff"];
    let secp256r1_fq: [&str; 3] = ["0xffffffffffffffffffffffff", "0xffff00000001000000000000000000", "0xffff"];
    let secp256r1_fr: [&str; 3] = [ "0xe6faada7179e84f3b9cac2fc632551", "0xffff00000000ffffffffffffffffbc", "0xffff"];
    let secp384r1_fq: [&str; 4] = [ "0xffffff0000000000000000ffffffff", "0xfffffffffffffffffffffffffffeff", "0xffffffffffffffffffffffffffffff", "0xffffff"];
    let secp384r1_fr: [&str; 4] = ["0x1a0db248b0a77aecec196accc52973", "0xffffffffffffc7634d81f4372ddf58", "0xffffffffffffffffffffffffffffff", "0xffffff"];
    let vesta_fq: [&str; 3] = [ "0x4698fc0994a8dd8c46eb2100000001", "0x22", "0x4000"];
    let vesta_fr: [&str; 3] = [ "0x4698fc094cf91b992d30ed00000001", "0x22", "0x4000"];
    let u256: [&str; 3] = [ "0x00", "0x00", "0x010000"];
    let u384: [&str; 4] = [ "0x00", "0x00", "0x00", "0x01000000"];
    let u512: [&str; 5] = ["0x00", "0x00", "0x00", "0x00", "0x0100000000"];
    let u768: [&str;7] = [ "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x01000000000000"];
    let u1024: [&str; 9] = [ "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x010000000000000000"];
    let u2048: [&str; 18] = ["0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x0100"];
    let u4096: [&str; 35] = [ "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x010000"];
    let u8192: [&str; 69] = ["0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x00", "0x0100000000"];

    generate_parameter_file(bls377_fq, "bls12_377Fq", "BLS12_377_Fq", bls12377_fq_blurb(), false)?;
    generate_parameter_file(bls377_fr, "bls12_377Fr", "BLS12_377_Fr", bls12377_fr_blurb(), false)?;
    generate_parameter_file(bls381_fq, "bls12_381Fq", "BLS12_381_Fq", bls12381_fq_blurb(), false)?;
    generate_parameter_file(bls381_fr, "bls12_381Fr", "BLS12_381_Fr", bls12381_fr_blurb(), false)?;
    generate_parameter_file(bn254_fq, "bn254Fq", "BN254_Fq", empty_blurb(), false)?;
    generate_parameter_file(ed25519_fq, "ed25519Fq", "ED25519_Fq", empty_blurb(),false)?;
    generate_parameter_file(ed25519_fr, "ed25519Fr", "ED25519_Fr", empty_blurb(),false)?;
    generate_parameter_file(mnt4753_fq, "mnt4_753Fq", "MNT4_753_Fq", mnt4753_fq_blurb(), false)?;
    generate_parameter_file(mnt4753_fr, "mnt4_753Fr", "MNT4_753_Fr", mnt4753_fr_blurb(), false)?;
    generate_parameter_file(mnt6753_fq, "mnt6_753Fq", "MNT6_753_Fq", mnt6753_fq_blurb(), false)?;
    generate_parameter_file(mnt6753_fr, "mnt6_753Fr", "MNT6_753_Fr", mnt6753_fr_blurb(), false)?;
    generate_parameter_file(pallas_fq, "pallasFq", "Pallas_Fq", pallas_fq_blurb(), false)?;
    generate_parameter_file(pallas_fr, "pallasFr", "Pallas_Fr", pallas_fr_blurb(), false)?;
    generate_parameter_file(vesta_fq, "vestaFq", "Vesta_Fq", vesta_fq_blurb(), false)?;
    generate_parameter_file(vesta_fr, "vestaFr", "Vesta_Fr", vesta_fr_blurb(), false)?;
    generate_parameter_file(secp256k1_fq, "secp256k1Fq", "Secp256k1_Fq", empty_blurb(), false)?;
    generate_parameter_file(secp256k1_fr, "secp256k1Fr", "Secp256k1_Fr", empty_blurb(), false)?;
    generate_parameter_file(secp256r1_fq, "secp256r1Fq", "Secp256r1_Fq", empty_blurb(), false)?;
    generate_parameter_file(secp256r1_fr, "secp256r1Fr", "Secp256r1_Fr", empty_blurb(), false)?;
    generate_parameter_file(secp384r1_fq, "secp384r1Fq", "Secp384r1_Fq", empty_blurb(), false)?;
    generate_parameter_file(secp384r1_fr, "secp384r1Fr", "Secp384r1_Fr", empty_blurb(), false)?;
    generate_parameter_file(u256, "U256", "U256",    empty_blurb(), true)?;
    generate_parameter_file(u384, "U384", "U384",    empty_blurb(), true)?;
    generate_parameter_file(u512, "U512", "U512",    empty_blurb(), true)?;
    generate_parameter_file(u768, "U768", "U768",    empty_blurb(), true)?;
    generate_parameter_file(u1024, "U1024", "U1024", empty_blurb(), true)?;
    generate_parameter_file(u2048, "U2048", "U2048", empty_blurb(), true)?;
    generate_parameter_file(u4096, "U4096", "U4096", empty_blurb(), true)?;
    generate_parameter_file(u8192, "U8192", "U8192", empty_blurb(), true)?;
    Ok(())
}

/**
 * @brief given a big number command line argument (either base10 or base 16 e.g. "1234", "0xabcd", "abcd" are all valid), spit out BigNum parameters formatted for noir.  
 */
fn main() {

    let args: Vec<String> = env::args().collect();

    let valid_args = args.len() == 3 || args.len() == 4 || (args.len() == 2 && &args[1] == "full");

    let help_msg = "usage: noir-bignum-paramgen [instance, runtime_instance, limbs, redc_param, full] ?[bignum modulus in hex or decimal] ?[parameter name]
        instance: outputs a string that represents a bignum::BigNumInstance object
        runtime_instance: outputs a string that represents a bignum::runtime_bignum::BigNumInstance object
        limbs: outputs an array of Field elements that represents a BigNum object's 120-bit limbs
        redc_param: outputs an array of Field elements that represents a BigNumInstance Barret reduction parameter
        full: writes a list of .nr parameter files for all currently supported BigNum fields, into the `fields` directory

EXAMPLE:
    noir-bignum-paramgen instance 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab BLS12_381_Fq";
    if !valid_args
    {
        println!("{}",help_msg);
    }
    else
    {
        let instruction_type = &args[1];
    
        let mut input_number: String = String::from("");
        if args.len() > 2
        {
            input_number = args[2].clone();
        }
        let mut name: String = String::from("BigNumInstance");
        if args.len() == 4 {
            name = args[3].clone();
        }
        let mut result =String::from(help_msg);
        match instruction_type.as_str() {
            "instance" => {
                result = bn_instance_from_string(input_number, name, false);
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
            "full" => {
                let _ = generate_parameters_full();
                result = String::from("parameter instances written into fields directory");
            }
            _ => {
            }
        }
        println!("{}", result.as_str());
    }
}



