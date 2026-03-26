/// Estrutura que representa um produto no sistema
#[derive(Debug, Clone)]
pub struct Produto {
    /// Identificador único do produto
    pub id: u32,

    /// Nome do produto
    pub nome: String,

    /// Categoria do produto (ex: Eletrônicos, Roupas)
    pub categoria: String,

    /// Marca do produto
    pub marca: String,

    /// Preço do produto
    pub preco: f64,
}

impl Produto {
    /// Cria um novo produto
    pub fn new(id: u32, nome: &str, categoria: &str, marca: &str, preco: f64) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            marca: marca.to_string(),
            preco,
        }
    }
}