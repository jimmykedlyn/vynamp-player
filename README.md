# vynamp-player

**English** · [Português](#português--vynamp-player)

The Vynamp player: the playback shell, the integration of `vynamp-engine` and the
native UI (egui, no browser).

> **Status: specification.** The player is not implemented. The repository holds the
> module specification, its task list and the tooling both are built with. It ships no
> running binary and makes no measured claim. It contains no Winamp code or
> third-party assets.

Licence: `MIT OR Apache-2.0` — see [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
Security: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-player/security/advisories/new)
— see [SECURITY.md](SECURITY.md).
Contributing: [CONTRIBUTING.md](CONTRIBUTING.md).

## Scope

Plays. It owns the shell, the layouts, the library and the plugin surface. It does
not decode, encode or talk to an audio device directly — `vynamp-engine` and the
libraries under it do. Batch conversion is `vynamp-converter` and restoration is
`vynamp-restore`, each in its own process.

## Builds on its own

```bash
cargo test
```

The agent contract is the umbrella's:
[AGENTS.md](https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md).

# Português — vynamp-player

[English](#vynamp-player) · **Português**

O player Vynamp: o shell de playback, a integração do `vynamp-engine` e a UI
nativa (egui, sem navegador).

> **Estado: especificação.** O player não está implementado. O repositório contém a
> especificação do módulo, a lista de tarefas dele e o tooling com que os dois são
> construídos. Não entrega binário em execução e não afirma resultado medido. Não contém
> código do Winamp nem assets de terceiros.

Licença: `MIT OR Apache-2.0` — ver [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT) e
[LICENSE-APACHE](LICENSE-APACHE).
Segurança: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-player/security/advisories/new)
— ver [SECURITY.md](SECURITY.md).
Contribuições: [CONTRIBUTING.md](CONTRIBUTING.md).

## Escopo

Toca. É dono do shell, dos layouts, da biblioteca e da superfície de plugins. Não
decodifica, não codifica e não fala com dispositivo de áudio diretamente — isso é
do `vynamp-engine` e das bibliotecas abaixo dele. Conversão em lote é o
`vynamp-converter` e restauração é o `vynamp-restore`, cada um em processo próprio.

## Compila sozinho

```bash
cargo test
```

O contrato do agente é o do umbrella:
[AGENTS.md](https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md).
