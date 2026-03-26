use std::collections::HashMap;
use crate::produto::Produto;

/// Estrutura principal do sistema de busca
pub struct SistemaBusca {
    /// Lista geral de produtos
    pub produtos: Vec<Produto>,

    /// Índice para busca rápida por nome
    pub indice_nome: HashMap<String, Vec<Produto>>,

    /// Índice para busca por categoria
    pub indice_categoria: HashMap<String, Vec<Produto>>,

    /// Índice para busca por marca
    pub indice_marca: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    /// Cria um novo sistema de busca vazio
    pub fn new() -> Self {
        SistemaBusca {
            produtos: Vec::new(),
            indice_nome: HashMap::new(),
            indice_categoria: HashMap::new(),
            indice_marca: HashMap::new(),
        }
    }

    /// Adiciona um produto ao sistema e atualiza todos os índices
    pub fn adicionar_produto(&mut self, produto: Produto) {
        // Adiciona na lista principal
        self.produtos.push(produto.clone());

        // Indexação por nome (em minúsculo para padronização)
        self.indice_nome
            .entry(produto.nome.to_lowercase())
            .or_insert(Vec::new())
            .push(produto.clone());

        // Indexação por categoria
        self.indice_categoria
            .entry(produto.categoria.to_lowercase())
            .or_insert(Vec::new())
            .push(produto.clone());

        // Indexação por marca
        self.indice_marca
            .entry(produto.marca.to_lowercase())
            .or_insert(Vec::new())
            .push(produto);
    }

    /// Busca produtos pelo nome exato
    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.indice_nome.get(&nome.to_lowercase())
    }

    /// Busca produtos pela categoria
    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> {
        self.indice_categoria.get(&categoria.to_lowercase())
    }

    /// Busca produtos pela marca
    pub fn buscar_por_marca(&self, marca: &str) -> Option<&Vec<Produto>> {
        self.indice_marca.get(&marca.to_lowercase())
    }

    /// 🔎 Busca parcial pelo nome (ex: "note" encontra "notebook")
    pub fn buscar_parcial_nome(&self, termo: &str) -> Vec<&Produto> {
        let termo = termo.to_lowercase();
        let mut resultados = Vec::new();

        // Percorre todos os produtos
        for produto in &self.produtos {
            // Verifica se o nome contém o termo buscado
            if produto.nome.to_lowercase().contains(&termo) {
                resultados.push(produto);
            }
        }

        resultados
    }
}