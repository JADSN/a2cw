// * [FINALIDADE] - Implementação do Arc<RwLock>

use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryStorage {
    content: Arc<RwLock<Vec<u8>>>,
}

// impl ToString for InMemoryStorage {
//     fn to_string(&self) -> String {
//         let data_vec = self.clone().read_all();
//         let mut result = String::new();

//         for (idx, data) in data_vec.iter().enumerate() {
//             let format = format!("{}. -> {}", idx, data);
//             result.push_str(format.as_ref());
//         }

//         result
//     }
// }

//? Explicação sobre Sized https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait
// TODO: Tornar trait PartialEq e Default como opcional
impl InMemoryStorage {
    // * Builder
    pub fn new() -> Self {
        Self {
            content: Arc::new(RwLock::new(Vec::<u8>::new())),
        }
    }

    // * READ(COUNT)
    pub fn read_count(self) -> usize {
        match self.content.read() {
            Ok(data) => {
                let data_vec = data.clone();
                data_vec.len()
            }
            Err(_) => 0,
        }
    }

    // * CRUD - REAL(ALL)
    pub fn read_all(self) -> Vec<u8> {
        match self.content.read() {
            Ok(data) => data.clone().to_vec(),
            Err(_) => vec![],
        }
    }

    // * CRUD - CREATE
    pub fn create_one(self, new_item: u8) {
        let mut data_vec = self.clone().read_all();
        data_vec.push(new_item);
        // * Tamanho do conteudo após a inserção do dado
        // TODO: Estudar a diferença entre `len()` e `capacity()`
        // ? Elementos Vs Bytes
        // let new_length: usize = data_vec.len();

        // * Sobrescreve o conteúdo em memória
        let mut data_write = self.content.write().unwrap();
        *data_write = data_vec;
    }

    // * CRUD - UPDATE
    pub fn update_one(&self, index: usize, new_value: u8) {
        let mut data_vec = self.clone().read_all();

        if index < data_vec.len() {
            data_vec[index] = new_value;

            // * Sobrescreve o conteúdo em memória
            let mut data_write = self.content.write().unwrap();
            *data_write = data_vec.clone();

            //? data_vec 2 read_all
        }
    }

    // * CRUD - DELETE
    pub fn delete_one(self, index: usize) {
        let mut data_vec = self.clone().read_all();
        if index < data_vec.len() {
            data_vec.remove(index);

            // * Tamanho do conteudo após a remoção do dado
            // TODO: Estudar a diferença entre `len()` e `capacity()`
            // let new_length: usize = data_vec.len();

            // * Sobrescreve o conteúdo em memória
            let mut data_write = self.content.write().unwrap();
            *data_write = data_vec;
        }
    }
}
