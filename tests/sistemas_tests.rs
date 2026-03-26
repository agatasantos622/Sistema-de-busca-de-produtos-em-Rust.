/// Testes automatizados do sistema de busca de produtos
/// Verifica funcionamento das buscas por categoria, marca e casos inexistentes

// Importa as estruturas necessárias do projeto
use megastore_busca::produto::Produto;
use megastore_busca::sistema_busca::SistemaBusca;

/// Testa a busca de produtos por categoria
#[test]
fn teste_busca_por_categoria() {
    // Cria uma nova instância do sistema
    let mut sistema = SistemaBusca::new();

    // Cria produtos de categorias diferentes
    let p1 = Produto::new(1, "Notebook Dell", "Eletrônicos", "Dell", 3500.0);
    let p2 = Produto::new(2, "Camiseta Nike", "Roupas", "Nike", 120.0);

    // Adiciona os produtos ao sistema
    sistema.adicionar_produto(p1);
    sistema.adicionar_produto(p2);

    // Realiza a busca por categoria (em minúsculo por causa da padronização)
    let resultado = sistema.buscar_por_categoria("eletrônicos");

    // Verifica se encontrou algum resultado
    assert!(resultado.is_some());

    // Verifica se apenas um produto foi retornado
    assert_eq!(resultado.unwrap().len(), 1);
}

/// Testa a busca de produtos por marca
#[test]
fn teste_busca_por_marca() {
    // Cria uma nova instância do sistema
    let mut sistema = SistemaBusca::new();

    // Cria um produto com marca específica
    let p1 = Produto::new(1, "iPhone 13", "Eletrônicos", "Apple", 5000.0);

    // Adiciona o produto ao sistema
    sistema.adicionar_produto(p1);

    // Realiza a busca pela marca (minúsculo)
    let resultado = sistema.buscar_por_marca("apple");

    // Verifica se encontrou resultado
    assert!(resultado.is_some());

    // Verifica se apenas um produto foi retornado
    assert_eq!(resultado.unwrap().len(), 1);
}

/// Testa o comportamento ao buscar um produto inexistente
#[test]
fn teste_produto_inexistente() {
    // Cria um sistema vazio (sem produtos)
    let sistema = SistemaBusca::new();

    // Tenta buscar um produto que não existe
    let resultado = sistema.buscar_por_nome("produto inexistente");

    // Verifica se o resultado é None (nenhum produto encontrado)
    assert!(resultado.is_none());
}