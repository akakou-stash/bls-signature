fn main() {}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate pairing;

    use pairing::Engine;
    use group::CurveAffine;
    use ff::Field;
    use pairing::bls12_381::{Bls12, Fr, G1Affine, G2Affine};

    #[test]
    fn pass_verifying() {
        let mut rng = rand::thread_rng();
        let prikey = <Fr as Field>::random(&mut rng);
        let pubkey = G2Affine::one().mul(prikey);

        let hash = 0xdeadbeef;
        let hash = G1Affine::one().mul(hash);
        let hash = G1Affine::from(hash);
        let signature = hash.mul(prikey);

        let rhs = Bls12::pairing(hash, pubkey);
        let lhs = Bls12::pairing(signature, G2Affine::one());

        println!("{}", lhs == rhs);
        assert_eq!(lhs == rhs, true);
    }

    #[test]
    fn fail_verifying() {
        let mut rng = rand::thread_rng();
        let prikey = <Fr as Field>::random(&mut rng);
        let pubkey = G2Affine::one().mul(prikey);

        let hash = 0xdeadbeef;
        let hash = G1Affine::one().mul(hash);
        let hash = G1Affine::from(hash);

        let dummy_prikey = <Fr as Field>::random(&mut rng);
        let signature = hash.mul(dummy_prikey);

        let rhs = Bls12::pairing(hash, pubkey);
        let lhs = Bls12::pairing(signature, G2Affine::one());

        println!("{}", lhs == rhs);
        assert_eq!(lhs != rhs, true);
    }
}