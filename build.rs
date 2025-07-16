fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if protoc is available
    match std::process::Command::new("protoc").arg("--version").output() {
        Ok(_) => {
            // Compile protobuf files
            tonic_build::configure()
                .build_server(true)
                .build_client(true)
                .compile(&["proto/agent.proto"], &["proto"])?;
            
            println!("cargo:rerun-if-changed=proto/agent.proto");
            println!("cargo:rerun-if-changed=proto");
        }
        Err(_) => {
            // Fallback: just watch for changes but don't compile
            println!("cargo:warning=protoc not found. gRPC will use manual types.");
            println!("cargo:warning=Install protoc to enable full gRPC functionality:");
            println!("cargo:warning=  Windows: https://grpc.io/docs/protoc-installation/");
            println!("cargo:warning=  Or use: winget install Google.Protobuf");
            println!("cargo:rerun-if-changed=proto/agent.proto");
            println!("cargo:rerun-if-changed=proto");
        }
    }
    
    Ok(())
} 