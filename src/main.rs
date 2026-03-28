/// Arquivo principal do sistema de busca de produtos
/// Responsável por executar o programa e demonstrar as funcionalidades

mod produto;
mod sistema_busca;

// Importa as estruturas do projeto
use produto::Produto;
use sistema_busca::SistemaBusca;

// Importa ferramenta para medir tempo de execução
use std::time::Instant;

/// Função auxiliar para exibir produtos de forma organizada
fn exibir_produtos(produtos: Vec<&Produto>) {
    for p in produtos {
        println!(
            "ID: {} | Nome: {} | Categoria: {} | Marca: {} | Preço: R$ {:.2}",
            p.id, p.nome, p.categoria, p.marca, p.preco
        );
    }
}

fn main() {
    // Cria o sistema de busca
    let mut sistema = SistemaBusca::new();

    // 🔁 Inserindo produtos fictícios (simulação de volume)
    for i in 0..10000 {
        let produto = Produto::new(
            i,
            &format!("Produto {}", i),
            "Eletrônicos",
            "MarcaX",
            100.0 + i as f64,
        );

        sistema.adicionar_produto(produto);
    }

    // 🧪 Produtos reais para teste
    sistema.adicionar_produto(Produto::new(
        10001,
        "Notebook Dell",
        "Eletrônicos",
        "Dell",
        3500.0,
    ));

    sistema.adicionar_produto(Produto::new(
        10002,
        "Notebook Samsung",
        "Eletrônicos",
        "Samsung",
        3200.0,
    ));

    sistema.adicionar_produto(Produto::new(
        10003,
        "iPhone 13",
        "Eletrônicos",
        "Apple",
        5000.0,
    ));

    // ===============================
    // 🔎 BUSCA EXATA (HashMap)
    // ===============================
    println!("\n🔎 Busca EXATA por 'Notebook Dell':");

    let inicio_exato = Instant::now();

    match sistema.buscar_por_nome("Notebook Dell") {
        Some(produtos) => {
            // Converte Vec<Produto> para Vec<&Produto>
            let lista: Vec<&Produto> = produtos.iter().collect();
            exibir_produtos(lista);
        }
        None => println!("Nenhum produto encontrado."),
    }

    let tempo_exato = inicio_exato.elapsed();
    println!("⏱ Tempo busca exata: {:?}\n", tempo_exato);

    // ===============================
    // 🔎 BUSCA PARCIAL (Linear)
    // ===============================
    println!("🔎 Busca PARCIAL por 'note':");

    // Marca o início da medição de tempo
    let inicio_parcial = Instant::now();

    // Executa a busca parcial no sistema
    let resultados = sistema.buscar_parcial_nome("note");

    // Verifica se encontrou resultados
    if resultados.is_empty() {
        println!("Nenhum produto encontrado.");
    } else {
        // Exibe todos os produtos encontrados
        exibir_produtos(resultados);
        }

    // Calcula o tempo total da busca
    let tempo_parcial = inicio_parcial.elapsed();
    // Exibe o tempo de execução
    println!("⏱ Tempo busca parcial: {:?}", tempo_parcial);
}
