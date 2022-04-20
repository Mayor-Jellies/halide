
//mod tests {
    use crate::build::GenBuilder;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn it_works() {
        let h = GenBuilder::new(
            "/home/rootbutcher2/CLionProjects/Halide-Rusts-tests/Halide",
            "test_files/",
        ).out_dir("test_files/");
        let g = h.new_gen("iir_blur".to_string());

        let out = g.make();
        println!("Gen Creation Status: {}", out.status);
        io::stdout().write_all(&out.stdout).unwrap();
        io::stderr().write_all(&out.stderr).unwrap();
        assert!(out.status.success());

        let out2 = g.run_gen();
        println!("Gen Run Status: {}", out2.status);
        io::stdout().write_all(&out2.stdout).unwrap();
        io::stderr().write_all(&out2.stderr).unwrap();
        assert!(out2.status.success());

        let out3 = g.rename_move();
        println!("move results: {:?}", out3);
        assert!(out3.is_ok());

        let out4 = g.bind();
        println!("bind results: {:?}", out4);
        assert!(out4.is_ok());
    }
//}
