#[cfg(test)]
mod benches {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bench_program_analysis(b: &mut Bencher) {
        let data = include_bytes!("../../../tests/test_programs/sample_token.so");
        b.iter(|| analyze_binary(data));
    }
}