use regex::Regex;
use std::io::Write;
use std::{fs, process::Command}; // For file writing

pub fn save_layout() {
    // 创建并执行命令
    let status = Command::new("sh")
        .arg("-c")
        .arg("hyprctl clients > /tmp/windows.save")
        .status()
        .expect("failed to execute command");

    // 检查命令是否成功执行
    if status.success() {
        println!("Layout saved successfully.");
    } else {
        eprintln!(
            "Failed to save layout with exit code: {}",
            status.code().unwrap_or(-1)
        );
        return;
    }

    Command::new("rm")
        .arg("-rf")
        .arg("/home/akira/.config/hykp/")
        .status()
        .expect("Failed to execute command");

    Command::new("mkdir")
        .arg("-p")
        .arg("/home/akira/.config/hykp")
        .status()
        .expect("Failed to execute command");

    let pid_output = "/home/akira/.config/hykp/pid.save";
    let pid_re = Regex::new(r"\s*pid:\s*(\d+)").unwrap();
    let layout_output = "/home/akira/.config/hykp/layout.save";
    let layout_re = Regex::new(r"\s*workspace:\s*(\d+)\s*\(\d+\)").unwrap();

    //获取命令行参数
    let contents = fs::read_to_string("/tmp/windows.save").unwrap();
    let query = "pid:";
    let pids = search(query, &contents);

    for pid in pids {
        if let Some(pid_number) = extract(pid, &pid_re) {
            println!("Found PID: {}", pid_number);
            if let Err(e) = save_cmdline_args(pid_number, &pid_output) {
                eprintln!(
                    "Failed to save command line args for PID {}: {}",
                    pid_number, e
                );
            }
        }
    }

    //获取layout
    let query_layout = "workspace";
    let workspaces = search(query_layout, &contents);
    for workspace in workspaces {
        if let Some(workspace_number) = extract(workspace, &layout_re) {
            println!("workspace: {}", workspace_number);
            if let Err(e) = save_workspace_info(workspace_number, &layout_output) {
                eprintln!(
                    "Failed to save command line args for PID {}: {}",
                    workspace_number, e
                );
            }
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn extract(line: &str, re: &Regex) -> Option<u32> {
    re.captures(line)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().parse::<u32>().unwrap_or(0)))
}

pub fn save_cmdline_args(pid: u32, output_path: &str) -> Result<(), std::io::Error> {
    let path = format!("/proc/{}/cmdline", pid);

    let contents = fs::read_to_string(path)?;

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)?;

    file.write_all(contents.as_bytes())?;
    file.write_all(b"\n")?;

    println!("Command line arguments appended to {}", output_path);
    Ok(())
}

pub fn save_workspace_info(workspace: u32, output_path: &str) -> Result<(), std::io::Error> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)?;

    let content = format!("{}\n", workspace); // 添加换行符以分隔每个 workspace 信息
    file.write_all(content.as_bytes())?;

    println!("Workspace information saved to {}", output_path);
    Ok(())
}

