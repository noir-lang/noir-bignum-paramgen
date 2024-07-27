# noir-bignum-paramgen

rust tool to generate parameters for noir's `noir-bignum` library (https://github.com/noir-lang/noir-bignum).

The tool takes in a big integer modulus and prints out a string that represents one of the following:

1. A `BigNumInstance` object that can be used to statically define the field (for when the field modulus is a circuit constant)
2. A `runtime_bignum::BigNumInstance` object that can be used to dynamically define the field from witness values (e.g. RSA signature verification)
3. An array of `Field` elements that represents a `BigNum` object
4. An array of `Field` elements that represents the Barrett reduction parameter for a `runtime_bignum::BigNumInstance` object

When constructing parameters for static fields, the 1st case should be usd.

If formatted `BigNum` limbs are public/private inputs to a noir circuit, the 3rd case can be used to generate the required witness values.

If a field modulus is defined via a witness, the Barrett reduction parameter must be provided to the circuit. It is only used in unconstrained functions and can be directly supplied without deriving in-circuit. In this case option 4 should be used.

# usage

`cargo build --release`
`./target/release/noir-bignum-paramgen [instance, runtime_instance, limbs, redc_param] [bignum modulus in hex or decimal] ?[parameter name]`

The 1st input argument defines which parameter is being generated
The 2nd argument defines the bignum modulus, either in base 10 or base 16 (base 16 can be prepended with `0x` but does not need to be)
The 3rd argument is the parameter name

# example usage

`./target/release/noir-bignum-paramgen instance 0x01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB117E776F218059DB80F0DA5CB537E38685ACCE9767254A4638810719AC425F0E39D54522CDD119F5E9063DE245E8001 MNT6_753_Fr > out.txt`

`./target/release/noir-bignum-paramgen runtime_instance 8444461749428370424248824938781546531375899335154063827935233455917409239041 BLS12_377_Fr > out.txt`

`./target/release/noir-bignum-paramgen limbs 01C4C62D92C41110229022EEE2CDADB7F997505B8FAFED5EB7E8F96C97D87307FDB925E8A0ED8D99D124D9A15AF79DB26C5C28C859A99B3EEBCA9429212636B9DFF97634993AA4D6C381BC3F0057974EA099170FA13A4FD90776E240000001 MNT4_753_Fr > out.txt`

`./target/release/noir-bignum-paramgen redc_param 0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff Secp256r1_Fq > out.txt`
