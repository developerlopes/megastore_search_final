use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub category: String,
    pub price: f32,
    pub stock: u32,
}

pub fn build_catalog() -> Vec<Product> {
    vec![
        Product {
            name: "Notebook Gamer".to_string(),
            category: "Informática".to_string(),
            price: 3500.0,
            stock: 12,
        },
        Product {
            name: "Smartphone Pro".to_string(),
            category: "Celulares".to_string(),
            price: 2800.0,
            stock: 30,
        },
        Product {
            name: "Smart TV 50\"".to_string(),
            category: "Eletrônicos".to_string(),
            price: 2900.0,
            stock: 7,
        },
        Product {
            name: "Cafeteira Elétrica".to_string(),
            category: "Eletrodomésticos".to_string(),
            price: 320.0,
            stock: 20,
        },
    ]
}

pub fn preprocess_text(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn build_index(catalog: &[Product]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, product) in catalog.iter().enumerate() {
        let tokens = preprocess_text(&product.name);
        for token in tokens {
            index.entry(token).or_default().push(i);
        }
    }
    index
}

pub fn search(query: &str, index: &HashMap<String, Vec<usize>>, catalog: &[Product]) -> Vec<Product> {
    let tokens = preprocess_text(query);
    let mut result_indices = vec![];
    for token in tokens {
        if let Some(indices) = index.get(&token) {
            result_indices.extend(indices);
        }
    }
    result_indices.sort_unstable();
    result_indices.dedup();
    result_indices
    .iter()
    .map(|&i: &usize| catalog[i].clone())
    .collect::<Vec<Product>>()
    
}
