# Editor Image Rust

Desenvolvido no curso: **Ultimate Rust Crash Course**
<br>Professor: **Nathan Stocks**


**Editor Image Rust** é uma ferramenta escrita em Rust que permite gerar e editar imagens diretamente no terminal de maneira simples e prática. Com suporte para diversos modos de geração e edição, você pode criar imagens únicas ou ajustar imagens existentes facilmente.

## 📖 Funcionalidades

### 🔨 Geração de Imagens
- **Sólida**: Cria uma imagem de uma cor sólida.
- **Ondas**: Gera uma imagem com padrão de ondas.
- **Gradiente**: Cria um gradiente entre duas cores (RGB). Escolha o tipo do gradiente entre:
  - Horizontal
  - Vertical
  - Diagonal
  - Radial
- **Fractal**: Gera uma imagem fractal baseada no conjunto de Julia.

### ✂️ Edição de Imagens
- **Desfocar**: Aplica um desfoque à imagem.
- **Clarear**: Aumenta o brilho da imagem.
- **Recortar**: Recorta a imagem baseado em tamanho e posição especificados.
- **Rotacionar**: Rotaciona a imagem em incrementos de 90°, 180° ou 270°.
- **Inverter**: Inverte as cores da imagem.
- **Preto e Branco**: Converte a imagem para tons de cinza.

## 🚀 Como Usar

1. **Escolha o que deseja fazer:**
   - Gerar uma nova imagem.
   - Editar uma imagem existente.

2. **Para gerar uma imagem:**
   - Escolha o modo de geração: `Sólida`, `Ondas`, `Gradiente` ou `Fractal`.
   - Especifique as dimensões da imagem (largura e altura).
   - Forneça as cores RGB iniciais e finais (se aplicável).
   - Escolha o nome do arquivo e o diretório para salvar a imagem.

3. **Para editar uma imagem:**
   - Arraste o arquivo para o terminal ou insira o diretório da imagem.
   - Escolha uma das opções de edição disponíveis.
   - Após cada ajuste, decida se deseja fazer mais alterações.
   - Escolha o nome do arquivo final e o diretório para salvá-lo.

## 📦 Formatos Suportados
- Salve suas imagens no formato de sua escolha dentre as opções PNG, JPEG, JPG e ICO.

## 📋 Pré-requisitos
- [Rust](https://www.rust-lang.org/) instalado na sua máquina.

## 🛠️ Instalação
1. Clone este repositório:
   ```bash
   git clonehttps://github.com/deusielbrizante/Editor-imagem-Rust.git
2. Navegue até o diretório do projeto:
   ```bash
   cd Editor-image-Rust
3. Compile o projeto:
    ```bash
    cargo build --release
4. Execute o programa:
    ```bash
    ./target/release/terminal-image-editor
---

### 🖼️ Exemplo de Uso

```markdown
### Gerar uma imagem sólida:
Selecione o modo de geração: Sólida
Informe a largura: 800
Informe a altura: 600
Informe a cor (RGB): 255, 0, 0
Nome do arquivo: minha_imagem_sólida
Formato do arquivo: PNG
Diretório para salvar: ./imagens
Imagem salva com sucesso!
```
```markdown
### Editar uma imagem:
Arraste a imagem para o terminal ou informe o diretório: ./imagens/exemplo.png
Selecione a edição desejada: Rotacionar
Opção dos ângulos escolhida: 90˚
Deseja fazer mais ajustes? (s/n): s
Selecione a edição desejada: Preto e Branco
Nome do arquivo: exemplo_editado
Formato do arquivo: PNG
Diretório para salvar: ./imagens_editadas
Imagem salva com sucesso!
```