use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};

type Input = (i32, i32, bool);
type Output = (i32, i32, u32, u32, u32);

const PACKAGE: &str = "guest";

fn main() {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    // Simulate a game turn: move right 2 steps, up 1 step, and shoot
    let input: Input = (2, 1, true);

    print!("Proving execution of the nexfus-fps game...");
    let proof = prover
        .prove_with_input::<Input>(&pp, &input)
        .expect("failed to prove program");

    let output = proof
        .output::<Output>()
        .expect("failed to deserialize output");

    println!(" Game state output:");
    println!("Player position: ({}, {})", output.0, output.1);
    println!("Player health: {}", output.2);
    println!("Player ammo: {}", output.3);
    println!("Enemies killed: {}", output.4);

    println!(">>>>> Logging\n{}<<<<<", proof.logs().join("\n"));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");
}