# Gaussian elimination
Este repositório contém a implementação da eliminação gaussiana na linguagem Rust. <br>
Existe várias versões:
1. Serial com representação da matriz como um vetor de vetores `Vec<Vec<f64>>`<br>
2. Serial com representação da matriz mapeada em um vetor, utilizando a forma [row major](https://en.wikipedia.org/wiki/Row-_and_column-major_order). Testes preliminares indicaram pouca melhora em relação a utilização de vetor de vetores.<br>
3. Paralela utilizando uma thread de kernel para solucionar cada linha, **NÃO UTILIZE** o overhead é muito grande. <br>
4. Paralela utilizando um [threadpool](https://github.com/reem/rust-scoped-pool) e criando uma task para solucionar cada linha da matriz, boa performance. <br>
5. Paralela utilizando programação assíncrona (utilizando o runtime do [Tokio](https://tokio.rs/)) com green threads, é lançada uma green thread para solucionar cada linha da matriz. Overhead baixo, desempenho levemente inferior ao uso de threadpool. <br>
6. Paralela utilizando [threadpool](https://github.com/reem/rust-scoped-pool) e criando tasks onde cada uma soluciona várias linhas da matriz. <br>

Não existe implementado nenhuma interface para selecionar de forma simplificada qual algoritmo será executado, então é necessário descomentar a opção desejada na função `main` e recompilar e rodar :(

# Build & Run
Para executar, é necessário ter o compilador `Rust`. <br>
1. Instale o Rust utilizando o `rustup` de acordo com o [site oficial](https://www.rust-lang.org/tools/install)
2. Clone o repositório
```
git clone https://github.com/JelsonRodrigues/gaussian_elimination
cd gaussian_elimination
```
3. Rode :)
```
cargo run
```
4. Se quiser compilar e executar de forma otimizada
```
cargo run --release
```