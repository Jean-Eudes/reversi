use crate::{GameConfig, GameState};
use bevy::app::AppExit;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AppExit>();
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(Update, (menu_action).run_if(in_state(GameState::Menu)));
        app.add_systems(OnExit(GameState::Menu), cleanup_menu);
        app.add_systems(OnEnter(GameState::Config), setup_config_menu);
        app.add_systems(Update, (config_action).run_if(in_state(GameState::Config)));
        app.add_systems(OnExit(GameState::Config), cleanup_config_menu);
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play1P,
    Config,
    Quit,
}

#[derive(Component)]
enum ConfigButtonAction {
    ToggleIndicators,
    Back,
}

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct ConfigRoot;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            MenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("REVERSI"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_button(parent, "Jouer", MenuButtonAction::Play1P);

            spawn_button(parent, "Config", MenuButtonAction::Config);

            spawn_button(parent, "Quitter", MenuButtonAction::Quit);
        });
}

fn spawn_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    label: &str,
    action: MenuButtonAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(NORMAL_BUTTON),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn menu_action(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        With<Button>,
    >,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    MenuButtonAction::Play1P => {
                        next_state.set(GameState::InGame);
                    }
                    MenuButtonAction::Config => {
                        next_state.set(GameState::Config);
                    }
                    MenuButtonAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_root_query: Single<Entity, With<MenuRoot>>) {
    let entity = menu_root_query.entity();
    commands.entity(entity).despawn();
}

fn setup_config_menu(mut commands: Commands, config: Res<GameConfig>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ConfigRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CONFIGURATION"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            let indicator_text = if config.show_playable_indicators {
                "Indicateurs: ON"
            } else {
                "Indicateurs: OFF"
            };
            spawn_config_button(parent, indicator_text, ConfigButtonAction::ToggleIndicators);

            spawn_config_button(parent, "Retour", ConfigButtonAction::Back);
        });
}

fn spawn_config_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    label: &str,
    action: ConfigButtonAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(NORMAL_BUTTON),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn config_action(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ConfigButtonAction),
        With<Button>,
    >,
    mut config: ResMut<GameConfig>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    ConfigButtonAction::ToggleIndicators => {
                        config.show_playable_indicators = !config.show_playable_indicators;
                        next_state.set(GameState::Config);
                    }
                    ConfigButtonAction::Back => {
                        next_state.set(GameState::Menu);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_config_menu(mut commands: Commands, config_root_query: Single<Entity, With<ConfigRoot>>) {
    let entity = config_root_query.entity();
    commands.entity(entity).despawn();
}
