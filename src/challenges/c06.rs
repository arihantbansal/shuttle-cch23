use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(alias = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(alias = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

pub async fn count_elf(elf_string: String) -> impl IntoResponse {
    let elf = elf_string.matches("elf").count();
    let elf_on_a_shelf = elf_string.matches("elf on a shelf").count();
    let shelf = elf_string.matches("shelf").count();

    Json(ElfCount {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it: shelf - elf_on_a_shelf,
    })
}
