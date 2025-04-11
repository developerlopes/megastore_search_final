use std::io::{self, Write};
use megastore_search::{build_catalog, build_index, search};

fn print_product(product: &megastore_search::Product) {
    println!(
        "Produto: {}
Categoria: {}
Preço: R${:.2}
Estoque: {}
---",
        product.name, product.category, product.price, product.stock
    );
}

fn main() {
    let catalog = build_catalog();
    let index = build_index(&catalog);

    loop {
        print!("Digite sua busca (ou 'sair' para encerrar): ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Erro ao ler entrada");
        let query = query.trim().to_lowercase();

        if query == "sair" {
            println!("Encerrando o sistema de busca. Até mais!");
            break;
        }

        let results = search(&query, &index, &catalog);

        if results.is_empty() {
            println!("
Nenhum produto encontrado para '{}'.
", query);
        } else {
            println!("
Resultados para '{}':
", query);
            for product in &results {
                print_product(product);
            }
        }
    }
}
