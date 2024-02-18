use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
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
