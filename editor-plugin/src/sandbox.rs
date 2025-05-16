//! Sandbox implementation for plugin isolation

use std::path::PathBuf;
use std::collections::HashSet;
use crate::{Permission, Result, PluginError};

#[cfg(unix)]
use rlimit::{Resource, setrlimit};

/// Sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Allowed file system paths
    pub allowed_paths: HashSet<PathBuf>,
    /// Allowed network hosts
    pub allowed_hosts: HashSet<String>,
    /// Allowed network ports
    pub allowed_ports: HashSet<u16>,
    /// Allowed system commands
    pub allowed_commands: HashSet<String>,
    /// Memory limit in bytes
    pub memory_limit: usize,
    /// CPU time limit in milliseconds
    pub cpu_limit: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            allowed_paths: HashSet::new(),
            allowed_hosts: HashSet::new(),
            allowed_ports: HashSet::new(),
            allowed_commands: HashSet::new(),
            memory_limit: 100 * 1024 * 1024, // 100MB
            cpu_limit: 1000, // 1 second
        }
    }
}

impl SandboxConfig {
    /// Creates a new sandbox configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds allowed file system paths
    pub fn allow_paths<I, P>(&mut self, paths: I) -> &mut Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        for path in paths {
            self.allowed_paths.insert(path.into());
        }
        self
    }

    /// Adds allowed network hosts
    pub fn allow_hosts<I, S>(&mut self, hosts: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for host in hosts {
            self.allowed_hosts.insert(host.into());
        }
        self
    }

    /// Adds allowed network ports
    pub fn allow_ports<I>(&mut self, ports: I) -> &mut Self
    where
        I: IntoIterator<Item = u16>,
    {
        self.allowed_ports.extend(ports);
        self
    }

    /// Adds allowed system commands
    pub fn allow_commands<I, S>(&mut self, commands: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for command in commands {
            self.allowed_commands.insert(command.into());
        }
        self
    }

    /// Sets memory limit
    pub fn with_memory_limit(&mut self, limit: usize) -> &mut Self {
        self.memory_limit = limit;
        self
    }

    /// Sets CPU time limit
    pub fn with_cpu_limit(&mut self, limit: u64) -> &mut Self {
        self.cpu_limit = limit;
        self
    }
}

/// Sandbox for plugin isolation
pub struct Sandbox {
    /// Sandbox configuration
    config: SandboxConfig,
}

impl Sandbox {
    /// Creates a new sandbox
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }

    /// Checks if a file system operation is allowed
    pub fn check_fs_access(&self, path: &PathBuf, read_only: bool) -> Result<()> {
        if !self.config.allowed_paths.iter().any(|allowed| path.starts_with(allowed)) {
            return Err(PluginError::SandboxError(
                format!("Access to path {} is not allowed", path.display())
            ));
        }

        // Additional checks for write access
        if !read_only {
            // Implement additional write permission checks
        }

        Ok(())
    }

    /// Checks if a network operation is allowed
    pub fn check_network_access(&self, host: &str, port: u16) -> Result<()> {
        if !self.config.allowed_hosts.contains(host) {
            return Err(PluginError::SandboxError(
                format!("Access to host {} is not allowed", host)
            ));
        }

        if !self.config.allowed_ports.contains(&port) {
            return Err(PluginError::SandboxError(
                format!("Access to port {} is not allowed", port)
            ));
        }

        Ok(())
    }

    /// Checks if a system command is allowed
    pub fn check_command_execution(&self, command: &str) -> Result<()> {
        if !self.config.allowed_commands.contains(command) {
            return Err(PluginError::SandboxError(
                format!("Execution of command {} is not allowed", command)
            ));
        }

        Ok(())
    }

    /// Enforces resource limits
    pub fn enforce_limits(&self) -> Result<()> {
        #[cfg(unix)]
        {
            // Set memory limit (address space)
            setrlimit(Resource::AS, self.config.memory_limit as u64, self.config.memory_limit as u64)
                .map_err(|e| PluginError::SandboxError(format!("Failed to set memory limit: {}", e)))?;

            // Set CPU time limit
            setrlimit(Resource::CPU, self.config.cpu_limit, self.config.cpu_limit)
                .map_err(|e| PluginError::SandboxError(format!("Failed to set CPU limit: {}", e)))?;
                
            // Set file size limit
            setrlimit(Resource::FSIZE, 10 * 1024 * 1024, 10 * 1024 * 1024) // 10MB file size limit
                .map_err(|e| PluginError::SandboxError(format!("Failed to set file size limit: {}", e)))?;
                
            // Set open files limit
            setrlimit(Resource::NOFILE, 50, 50) // Limit number of open files
                .map_err(|e| PluginError::SandboxError(format!("Failed to set open files limit: {}", e)))?;
        }

        #[cfg(windows)]
        {
            // Windows doesn't support rlimit, implement alternative resource limiting if needed
            log::warn!("Resource limits are not fully implemented on Windows");
            // TODO: Implement Windows-specific resource limiting
        }

        Ok(())
    }

    /// Verifies permissions against sandbox configuration
    pub fn verify_permissions(&self, permissions: &[Permission]) -> Result<()> {
        for permission in permissions {
            match permission {
                Permission::FileSystem { paths, read_only } => {
                    for path in paths {
                        self.check_fs_access(path, *read_only)?;
                    }
                }
                Permission::Network { hosts, ports } => {
                    for host in hosts {
                        for port in ports {
                            self.check_network_access(host, *port)?;
                        }
                    }
                }
                Permission::Process { commands } => {
                    for command in commands {
                        self.check_command_execution(command)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_configuration() {
        let mut config = SandboxConfig::new();
        config
            .allow_paths(vec!["/tmp"])
            .allow_hosts(vec!["localhost"])
            .allow_ports(vec![8000, 8001])
            .allow_commands(vec!["ls", "cat"])
            .with_memory_limit(200 * 1024 * 1024)
            .with_cpu_limit(2000);

        assert!(config.allowed_paths.contains(&PathBuf::from("/tmp")));
        assert!(config.allowed_hosts.contains("localhost"));
        assert!(config.allowed_ports.contains(&8000));
        assert!(config.allowed_commands.contains("ls"));
        assert_eq!(config.memory_limit, 200 * 1024 * 1024);
        assert_eq!(config.cpu_limit, 2000);
    }

    #[test]
    fn test_sandbox_permission_verification() {
        let mut config = SandboxConfig::new();
        config
            .allow_paths(vec!["/tmp"])
            .allow_hosts(vec!["localhost"])
            .allow_ports(vec![8000]);

        let sandbox = Sandbox::new(config);

        // Test file system access
        assert!(sandbox.check_fs_access(&PathBuf::from("/tmp/test.txt"), true).is_ok());
        assert!(sandbox.check_fs_access(&PathBuf::from("/etc/test.txt"), true).is_err());

        // Test network access
        assert!(sandbox.check_network_access("localhost", 8000).is_ok());
        assert!(sandbox.check_network_access("example.com", 8000).is_err());
        assert!(sandbox.check_network_access("localhost", 8001).is_err());
    }
}
