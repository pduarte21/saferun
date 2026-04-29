pub fn contains_blocked_network_tools(script_contents: &str) -> Option<&'static str> {
    let blocked = ["curl", "wget", "nc", "netcat", "scp"];

    for tool in blocked {
        if script_contents.contains(tool) {
            return Some(tool);
        }
    }

    None
}