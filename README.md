# 🏋️ GymTracker Pro

Bem-vindo ao **GymTracker Pro**, seu companheiro de treino definitivo! Este é um aplicativo desktop e mobile multiplataforma, construído com o framework **Dioxus 0.6.3** e a linguagem **Rust**, com uma interface de usuário baseada no código HTML fornecido. O objetivo é ajudar você a planejar, registrar e acompanhar seu progresso na academia de forma simples e eficiente.

## ✨ Recursos

-   **Painel Interativo:** Uma visão geral rápida dos seus treinos, volume total e recordes pessoais.
    
-   **Planos de Treino Personalizados:** Crie e gerencie seus próprios planos de treino com exercícios, séries e repetições.
    
-   **Registro de Progresso:** Salve seus treinos em tempo real, registrando peso e repetições para cada série.
    
-   **Estatísticas Detalhadas:** Veja seu progresso ao longo do tempo, incluindo carga máxima e volume total por exercício.
    
-   **Multiplataforma:** Desenvolvido para funcionar nativamente no **Linux, Windows e Android**.
    

----------

## 🚀 Como Executar o Projeto

Para rodar este aplicativo, você precisa ter o **Rust** e o **Dioxus** instalados na sua máquina.

### Pré-requisitos

-   **Rust:** Certifique-se de ter o `rustup` instalado.
    
-   **Dioxus:** A versão **0.6.3** é a utilizada neste projeto. Você pode instalá-la com o comando:
    
    Bash
    
    ```
    cargo install dioxus-cli --version 0.6.3
    
    ```
    
-   **Ferramentas para Android (opcional):** Se você for compilar para Android, precisará do **Android Studio** e da ferramenta `dioxus-cli` configurada para a plataforma.
    

### Executando para Desktop (Linux/Windows)

Navegue até o diretório raiz do projeto e execute:

Bash

```
cargo run --target x86_64-pc-windows-msvc  # Para Windows

```

ou

Bash

```
cargo run --target x86_64-unknown-linux-gnu # Para Linux

```

_Dica: Se você não precisa especificar o target, pode simplesmente usar `cargo run`._

### Executando para Android

Certifique-se de ter as ferramentas de desenvolvimento do Android (SDK, NDK) instaladas e configuradas. Em seguida, conecte seu dispositivo ou inicie um emulador.

Bash

```
dioxus-cli mobile run

```

_Este comando compilará e instalará o aplicativo no seu dispositivo Android._

----------

## 📥 Downloads

As versões compiladas para **Linux**, **Windows** e **Android** estão disponíveis na página de [Releases](https://github.com/marcelosanto/app_treino/releases) do projeto. Baixe o arquivo correspondente ao seu sistema operacional para começar a usar.

----------

## 🛠️ Estrutura do Projeto

O projeto Dioxus utiliza uma estrutura padrão de um projeto Rust. A maior parte do trabalho da interface está contida em um único arquivo, espelhando a estrutura HTML.

```
/
├── src/
│   └── main.rs          # Lógica principal do aplicativo em Rust e Dioxus.
├── Cargo.toml           # Arquivo de configuração do Rust, listando dependências.
├── Dioxus.toml          # Configurações do Dioxus.
└── README.md            # Este arquivo.

```

O arquivo `main.rs` contém a lógica de renderização da UI, tratando o HTML e CSS como componentes Dioxus e gerindo o estado (treinos, progresso) para persistência em disco.

----------

## ✍️ Contribuições

Contribuições são sempre bem-vindas! Se você tiver uma ideia de melhoria, reporte um bug ou queira adicionar um novo recurso, sinta-se à vontade para abrir uma _issue_ ou enviar um _pull request_.

## 📄 Licença

Este projeto está licenciado sob a Licença MIT.

----------

## 🤔 Quer saber mais?

-   **Dioxus:** [dioxuslabs.com](https://dioxuslabs.com/)
    
-   **Rust:** [rust-lang.org](https://www.rust-lang.org/pt-BR)
