pub mod server;

use std::process::Command;
use uuid::Uuid;

/// Deploy an agent or team to Docker and start HTTP server
pub async fn deploy_agent(
    _config_path: &str,
    port: u16,
    container_name: Option<String>,
    tag: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("🔧 Building Docker image...");
    
    // Generate container name if not provided
    let name = container_name.unwrap_or_else(|| {
        let uuid = Uuid::new_v4();
        format!("helixor-{}", uuid.to_string().split('-').next().unwrap())
    });

    // Build Docker image
    build_docker_image(&name, &tag)?;
    
    // Run container
    let container_id = run_docker_container(&name, &tag, port)?;
    
    // Start HTTP server inside container
    let url = format!("http://localhost:{}", port);
    
    println!("✅ Agent deployed successfully!");
    println!("🌐 URL: {}", url);
    println!("🆔 Container: {}", container_id);
    println!("📊 Health check: {}/health", url);
    
    Ok(url)
}

/// Build Docker image
fn build_docker_image(name: &str, tag: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("docker")
        .args(&["build", "-t", &format!("{}:{}", name, tag), "."])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker build failed: {}", error).into());
    }

    println!("✅ Docker image built successfully");
    Ok(())
}

/// Run Docker container
fn run_docker_container(name: &str, tag: &str, port: u16) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let container_name = format!("{}-{}", name, Uuid::new_v4().to_string().split('-').next().unwrap());
    
    let output = Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name", &container_name,
            "-p", &format!("{}:8080", port),
            "-e", "RUST_LOG=info",
            &format!("{}:{}", name, tag),
        ])
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Docker run failed: {}", error).into());
    }

    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("✅ Container started: {}", container_id);
    
    Ok(container_id)
}

/// Stop and remove a deployed agent
pub fn stop_agent(container_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🛑 Stopping agent: {}", container_name);
    
    // Stop container
    let _ = Command::new("docker")
        .args(&["stop", container_name])
        .output()?;

    // Remove container
    let _ = Command::new("docker")
        .args(&["rm", container_name])
        .output()?;

    println!("✅ Agent stopped and removed");
    Ok(())
}

/// List all deployed agents
pub fn list_deployed_agents() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("docker")
        .args(&["ps", "--filter", "ancestor=helixor", "--format", "{{.Names}}"])
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let containers = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(containers)
} 