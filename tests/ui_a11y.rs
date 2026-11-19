//! Critério TASK-E02-01-AC-2, parte executável headless: a janela Classic Stack
//! no tamanho mínimo expõe cada controle de transporte como um nó AccessKit
//! nomeado — a pré-condição para um leitor de tela navegar e acionar.
//!
//! O que este teste NÃO cobre (registrado na TASK): leitores de tela reais
//! (Narrator/NVDA/Orca/VoiceOver, AC-1) e zoom 200% em sessão gráfica —
//! ambos exigem máquina com UI e ficam atrás de `#[ignore]` quando existirem
//! testes de plataforma.

use eframe::egui::{Context, Pos2, RawInput, Rect, Vec2, ViewportId, accesskit};

/// Roda um passe de UI headless e devolve o `TreeUpdate` AccessKit produzido.
///
/// Painel central com `screen_rect` do tamanho mínimo do Classic Stack
/// (`design/02-layouts.md`: 480×660 DIP).
fn accesskit_update_at_min_size(
    build_ui: impl FnMut(&mut eframe::egui::Ui),
) -> accesskit::TreeUpdate {
    let ctx = Context::default();
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 660.0))),
            ..Default::default()
        },
        build_ui,
    );
    // Um passe pode emitir deltas de textura (fontes); sem renderer, aplicá-los
    // num allocator vazio equivale a descartá-los com `clear`.
    let mut textures = output.textures_delta;
    textures.clear();
    output
        .platform_output
        .accesskit_update
        .expect("accesskit habilitado deve produzir TreeUpdate")
}

/// Coleta `(label, role)` de todos os nós com label no update.
fn labeled_nodes(update: &accesskit::TreeUpdate) -> Vec<(String, accesskit::Role)> {
    update
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(|l| (l.to_owned(), node.role())))
        .collect()
}

#[test]
fn classic_stack_exposes_transport_controls_at_minimum_window_size() {
    let update = accesskit_update_at_min_size(|ui| {
        vynamp_player::ui::classic_stack(ui);
    });

    let labeled = labeled_nodes(&update);

    // AC-2: cada controle de transporte alcançável e nomeado na árvore.
    for label in ["Abrir", "Play", "Pause", "Stop", "Próxima"] {
        let hit = labeled
            .iter()
            .find(|(l, _)| l == label)
            .unwrap_or_else(|| panic!("controle {label:?} sem nó AccessKit nomeado"));
        assert_eq!(
            hit.1,
            accesskit::Role::Button,
            "controle {label:?} deveria ter role Button"
        );
    }

    // Seek é um slider: role Slider com label.
    let seek = labeled
        .iter()
        .find(|(l, _)| l == "Seek")
        .unwrap_or_else(|| panic!("controle \"Seek\" sem nó AccessKit nomeado"));
    assert_eq!(
        seek.1,
        accesskit::Role::Slider,
        "Seek deveria ter role Slider"
    );
}

/// Segundo passe com o mesmo `Context`: a árvore deve continuar presente e
/// estável (ids estáveis são o que permite ao leitor de tela acompanhar).
#[test]
fn classic_stack_accesskit_tree_is_stable_across_passes() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    let input = RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 660.0))),
        ..Default::default()
    };

    let mut first = ctx.run_ui(input.clone(), vynamp_player::ui::classic_stack);
    first.textures_delta.clear();
    let mut second = ctx.run_ui(input, vynamp_player::ui::classic_stack);

    second.textures_delta.clear();

    let update = second
        .platform_output
        .accesskit_update
        .expect("segundo passe deve manter TreeUpdate");
    let labeled = labeled_nodes(&update);
    for label in ["Abrir", "Play", "Pause", "Stop", "Próxima", "Seek"] {
        assert!(
            labeled.iter().any(|(l, _)| l == label),
            "controle {label:?} sumiu da árvore no segundo passe"
        );
    }
}

/// Roda dois passes e garante que a raiz é uma janela com filhos —
/// a estrutura mínima para o leitor de tela entrar na árvore.
#[test]
fn accesskit_root_is_window_with_children() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 660.0))),
            ..Default::default()
        },
        vynamp_player::ui::classic_stack,
    );
    let mut textures = output.textures_delta;
    textures.clear();
    let update = output
        .platform_output
        .accesskit_update
        .expect("TreeUpdate presente");

    let root_id = accesskit::NodeId::from(egui::accesskit_root_id().value());
    let tree = update.tree.as_ref().expect("TreeInfo presente");
    assert_eq!(tree.root, root_id, "raiz da árvore é a janela do egui");

    let root_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, node)| node)
        .expect("TreeUpdate deve conter o nó raiz");
    assert_eq!(root_node.role(), accesskit::Role::Window);
    assert!(
        !root_node.children().is_empty(),
        "raiz precisa de filhos para o leitor navegar"
    );

    // ViewportId só para fixar que este é o viewport raiz.
    let _ = ViewportId::ROOT;
}
