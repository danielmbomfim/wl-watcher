# WL-wacher

Script simples para autodeploy de aplicações Weblogic

## Utilização

Para utilizar a ferramenta basta executar o comando abaixo substituindo o valor dos parametros.

```bash
wl-watcher -s <caminho para o diretório contendo o arquivo pom.xml do seu projeto> -d <caminho para o diretório de deploy do weblogic>
```

Para conferir todos os paremetros execute o comando abaico

```bash
wl-watcher --help
```
## Instalação

```bash
git clone https://github.com/danielmbomfim/wl-watcher.git
cargo install --path=wl-watcher
```
