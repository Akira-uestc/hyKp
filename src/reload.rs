use std::{fs, process::Command};

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
        let output = Command::new("fish")
            .arg("-c")
            .arg(&cleaned_cmd)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("fish command failed with output: {:?}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
