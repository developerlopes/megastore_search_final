use megastore_search::{build_catalog, build_index, preprocess_text, search};

#[test]
fn test_preprocess_text() {
    let entrada = "Notebook Lenovo!";
    let saida = preprocess_text(entrada);
    assert_eq!(saida, vec!["notebook", "lenovo"]);
}

#[test]
fn test_busca_encontra_produto() {
    let catalog = build_catalog();
    let index = build_index(&catalog);
    let resultados = search("smartphone", &index, &catalog);

    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].name, "Smartphone Pro");
}

#[test]
fn test_busca_nao_encontra_nada() {
    let catalog = build_catalog();
    let index = build_index(&catalog);
    let resultados = search("geladeira", &index, &catalog);

    assert!(resultados.is_empty());
}
