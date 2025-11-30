# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado
- **Vectorizer Integration**: Integração completa do Vectorizer para busca semântica de regras D&D 5e
  - Configuração de volumes nomeados do Docker para persistência de dados
  - Scripts Python para processamento e indexação de PDFs de livros D&D 5e
  - Pipeline completo: Transmutation → Classify → Vectorizer
  - Suporte para vetores híbridos (512d + BM25)
  - Documentação completa de setup e persistência
  - **Collections por Livro**: Sistema de collections separadas para cada livro D&D 5e
    - `create-collections-per-book.py`: Script para criar e indexar collections individuais
    - `check-collections-status.py`: Script para verificar status de todas as collections
    - 6 collections criadas: Manual dos Monstros, Livro do Jogador, Guia do Mestre, Ficha de Personagem, Guia de Xanathar, Guia do Volo
    - Corrigido endpoint de inserção: `/collections/{collection}/vectors` (não `/insert`)
    - Adicionado geração de IDs únicos (UUID) para cada chunk

### Mudado
- **Vectorizer Docker Configuration**: Migrado de bind mounts para volumes nomeados do Docker
  - Melhor gerenciamento de volumes pelo Docker
  - Persistência garantida entre reinicializações
  - Volumes aparecem na interface de gerenciamento do Docker Desktop
- **Docker Compose**: Removido `version: '3.8'` (obsoleto no Docker Compose v2)
- **Healthcheck**: Atualizado para usar `wget` como alternativa ao `curl`

### Corrigido
- **Vectorizer Persistence**: Corrigido problema de perda de dados após reinicialização do container
  - Volumes nomeados garantem persistência correta
  - Configuração de `DATA_DIR` para diretório correto no container

### Documentação
- Adicionado `vectorizer-setup.md` com guia completo de instalação e configuração
- Adicionado `vectorizer-persistence.md` com detalhes sobre persistência de dados
- Atualizado `INDEX.md` com referências às novas documentações

## [2025-01-XX] - Sistema de Magias D&D 5e

### Adicionado
- **Sistema de Magias D&D 5e**: Implementação completa do sistema de magias
  - Estrutura de dados para magias (níveis, escolas, componentes, etc.)
  - Gerenciamento de spell slots por nível
  - Sistema de casting com validação
  - Suporte a upcasting (casting em nível superior)
  - Cálculo de dano e cura com upcasting
  - Spell database com busca e filtros
  - Endpoints HTTP REST para interação com o sistema

### Mudado
- **SpellCaster**: Adicionado suporte para upcasting
  - `calculate_upcast_damage`: Calcula dano aumentado por nível extra
  - `calculate_upcast_healing`: Calcula cura aumentada por nível extra
  - `cast`: Atualizado para usar cálculos de upcasting baseados no `slot_level`

### Corrigido
- **Type Mismatch**: Corrigido erro de tipo ao calcular upcasting (u32 vs i32)
  - Cast explícito de `level_diff` para `u32` em funções de upcasting

---

## Formato

### Tipos de Mudanças
- **Adicionado**: Para novas funcionalidades
- **Mudado**: Para mudanças em funcionalidades existentes
- **Depreciado**: Para funcionalidades que serão removidas
- **Removido**: Para funcionalidades removidas
- **Corrigido**: Para correções de bugs
- **Segurança**: Para vulnerabilidades
