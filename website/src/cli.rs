use crate::models::{
    CliAutocompleteRequest, CliAutocompleteResponse, CliExecuteRequest, CliExecuteResponse,
};
use axum::{http::StatusCode, Json};
use rand::Rng;

// command list
const AVAILABLE_COMMANDS: &[&str] = &[
    "help", "about", "projects", "contact", "clear", "whoami", "sudo", "echo", "theme", "github",
    "resume", "matrix", "ls", "cat", "rm", "roll",
];

pub async fn handle_execute(
    Json(payload): Json<CliExecuteRequest>,
) -> (StatusCode, Json<CliExecuteResponse>) {
    let input = payload.command.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return (
            StatusCode::OK,
            Json(CliExecuteResponse {
                output: String::new(),
            }),
        );
    }

    let output = match parts[0] {
        "help" => format!("Available commands: {}", AVAILABLE_COMMANDS.join(", ")),
        "about" => "Hi, I'm a wannabe Rust developer.\nWelcome to my server :)".to_string(),
        "projects" => "1. My Site :D\n2. Live Terminal Resume\n3. REDACTED".to_string(),
        "contact" => "Please use the GUI form, or ping me on GitHub!".to_string(),
        "whoami" => "guest".to_string(),
        "clear" => "CLEAR_TERMINAL".to_string(),
        "github" => "OPEN_TAB_GITHUB".to_string(),
        "resume" => "DOWNLOAD_RESUME".to_string(),
        "matrix" => "TRIGGER_MATRIX_EFFECT".to_string(),
        "theme" => {
            if parts.len() > 1 {
                match parts[1] {
                            "dark" => "SET_THEME_DARK".to_string(),
                            "light" => "SET_THEME_LIGHT".to_string(),
                            "matrix" => "SET_THEME_HACKER".to_string(),
                            "coffee" => "SET_THEME_COFFEE".to_string(),
                            "red" => "SET_THEME_RED".to_string(),
                            "synthwave" => "SET_THEME_SYNTHWAVE".to_string(),
                            "ocean" => "SET_THEME_OCEAN".to_string(),
                            _ => format!("Unknown theme '{}'. Available: dark, light, hacker, coffee, red, synthwave, ocean.", parts[1]),
                        }
            } else {
                "Usage: theme dark|light|coffee|red|synthwave|ocean|matrix".to_string()
            }
        }
        "echo" => {
            if parts.len() > 1 {
                parts[1..].join(" ")
            } else {
                "Usage: echo <text>".to_string()
            }
        }
        "roll" => {
            if parts.len() > 1 && parts[1].starts_with('d') {
                if let Ok(sides) = parts[1][1..].parse::<u32>() {
                    if sides > 0 {
                        let roll = rand::thread_rng().gen_range(1..=sides);
                        format!("You rolled a d{}: {}", sides, roll)
                    } else {
                        "A dice must have at least 1 side!".to_string()
                    }
                } else {
                    "Invalid dice format. Try 'roll d20', 'roll d6', 'roll d100'.".to_string()
                }
            } else {
                "Usage: roll d<number> (e.g., roll d20)".to_string()
            }
        }
        "sudo" => {
            "User 'guest' is not in the sudoers file. This incident will be reported to Santa D:"
                .to_string()
        }
        "ls" => {
            let has_flags = parts.len() > 1 && parts[1].starts_with('-');
            if has_flags {
                "drwxr-xr-x 2 guest guest 4096 Apr 26 14:47 .\ndrwxr-xr-x 3 root  root  4096 Apr 26 14:47 ..\n-rw-r--r-- 1 root  root   104 Apr 26 14:47 projects.txt\n-r-------- 1 root  root    42 Apr 26 14:47 secret_keys.env\n-rwxr-xr-x 1 root  root  8192 Apr 26 14:47 deploy.sh".to_string()
            } else {
                "projects.txt   secret_keys.env   deploy.sh".to_string()
            }
        }
        "cat" => {
            if parts.len() > 1 {
                match parts[1] {
                    "projects.txt" => {
                        "1. My Website\n2. Live Terminal Resume\n3. Leaning the Kernel".to_string()
                    }
                    "secret_keys.env" => "cat: secret_keys.env: Permission denied".to_string(),
                    "deploy.sh" => {
                        "echo 'Deploying to prod on a Friday...' \necho 'ERROR: Cluster offline'"
                            .to_string()
                    }
                    file => format!("cat: {}: No such file or directory", file),
                }
            } else {
                "Usage: cat <filename>".to_string()
            }
        }
        "rm" => {
            if parts.join(" ") == "rm -rf /" {
                "Nice try.".to_string()
            } else {
                "rm: missing operand. (Also, please don't delete my stuff)".to_string()
            }
        }
        cmd => format!("Command not found: {}. Type 'help' for options.", cmd),
    };

    (StatusCode::OK, Json(CliExecuteResponse { output }))
}

pub async fn handle_autocomplete(
    Json(payload): Json<CliAutocompleteRequest>,
) -> (StatusCode, Json<CliAutocompleteResponse>) {
    let partial = payload.partial.trim();
    let matches: Vec<String> = AVAILABLE_COMMANDS
        .iter()
        .filter(|cmd| cmd.starts_with(partial))
        .map(|s| s.to_string())
        .collect();

    (StatusCode::OK, Json(CliAutocompleteResponse { matches }))
}
