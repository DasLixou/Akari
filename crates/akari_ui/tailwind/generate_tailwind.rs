use duct::cmd;

fn main() {
    cmd!(
        "npx",
        "tailwindcss",
        "-i",
        "./style.css",
        "-o",
        "./tailwind/generated.css",
    )
    .read()
    .unwrap();
}
