use std::{fs, process::{Command, Output}};

pub fn parse_saved() -> (Vec<String>, Vec<String>) {
    let pid_input = "/home/akira/.config/hyprStartup/pid.save";
    let layout_input = "/home/akira/.config/hyprStartup/layout.save";

    let mut process = Vec::new();
    let mut layout = Vec::new();

    if let Ok(contents) = fs::read_to_string(pid_input) {
        for line in contents.lines() {
            process.push(line.to_string());
        }
    } else {
        eprintln!("Failed to read the PID input file.");
    }

    if let Ok(contents) = fs::read_to_string(layout_input) {
        for line in contents.lines() {
            layout.push(line.to_string());
        }
    } else {
        eprintln!("Failed to read the layout input file.");
    }

    (process, layout)

}

pub fn restore_window() {
    let windows_info: (Vec<String>, Vec<String>) = parse_saved();
    let (cmdline, workspace) = windows_info;

    for (cmd, ws) in cmdline.iter().zip(workspace.iter()) {
        // 删除 null 字节
        let cleaned_cmd = cmd.replace('\0', "");
        let cleaned_ws = ws.replace('\0', "");

        // 执行 hyprctl 命令
        let output = Command::new("hyprctl")
            .arg("dispatch")
            .arg(format!("workspace {}", cleaned_ws))
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("hyprctl command failed with output: {:?}", String::from_utf8_lossy(&output.stderr));
        }

        // 执行 fish shell 命令
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} > /dev/null 2>&1 &", cleaned_cmd))
            .spawn()  // Use `spawn` to start the process in the background
            .expect("Failed to execute command");
        
        let duration = std::time::Duration::from_millis(2000);
        std::thread::sleep(duration);
    }
}
