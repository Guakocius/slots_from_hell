use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.0, 0.28, 0.73);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    Playing,
}

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy, Component)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Component)]
struct Setting<T>(T);

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy, Component)]
struct Volume(u32);

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .init_state::<GameState>()
            .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

mod splash {
    use bevy::prelude::*;

    use super::GameState;

    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("img/icon.png");
        cmds.spawn((
            DespawnOnExit(GameState::Splash),
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            OnSplashScreen,
            children![(
                ImageNode::new(icon),
                Node {
                    width: px(200),
                    ..default()
                },
            )],
        ));
        cmds.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameState::Menu);
        }
    }
}

mod game {
    use bevy::{
        color::palettes::basic::{NAVY, SILVER},
        prelude::*,
    };

    use super::{DisplayQuality, GameState, TEXT_COLOR, Volume};

    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)));
    }

    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn game_setup(mut cmds: Commands, display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
        cmds.spawn((
            DespawnOnExit(GameState::Game),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnGameScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("Back to menu"),
                        TextFont {
                            font_size: 68.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Text::default(),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                        children![
                            (
                                TextSpan(format!("Quality: {:?}", *display_quality)),
                                TextFont {
                                    font_size: 50.0,
                                    ..default()
                                },
                                TextColor(NAVY.into()),
                            ),
                            (
                                TextSpan::new(" - "),
                                TextFont {
                                    font_size: 50.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ),
                            (
                                TextSpan(format!("Volume: {:?}", *volume)),
                                TextFont {
                                    font_size: 50.0,
                                    ..default()
                                },
                                TextColor(SILVER.into()),
                            ),
                        ]
                    ),
                ]
            )],
        ));

        cmds.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    }

    fn game(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameState::Playing);
        }
    }
}

mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::NAVAJO_WHITE,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };

    use super::{DisplayQuality, GameState, Setting, TEXT_COLOR, Volume};

    use crate::components::player::{move_player, setup_instructions, update_camera};

    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(GameState::Playing), (setup_instructions))
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
            )
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                Update,
                (move_player, update_camera)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct OnDisplaySettingsMenuScreen;

    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
    const HOVERED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.5, 0.65, 0.5);
    const PRESSED_BUTTON: Color = Color::srgb(0.65, 0.75, 0.65);

    #[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<
            (&Interaction, &Setting<T>, Entity),
            (Changed<Interaction>, With<Button>),
        >,
        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut cmds: Commands,
        mut setting: ResMut<T>,
    ) {
        let (prev_btn, mut prev_btn_color) = selected_query.into_inner();
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != button_setting.0 {
                *prev_btn_color = NORMAL_BUTTON.into();

                cmds.entity(prev_btn).remove::<SelectedOption>();
                cmds.entity(entity).insert(SelectedOption);
                *setting = button_setting.0;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let button_node = Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_node = Node {
            width: px(30),
            position_type: PositionType::Absolute,
            left: px(10),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        let right_icon = asset_server.load("img/right.png");
        let wrench_icon = asset_server.load("img/wrench.png");
        let exit_icon = asset_server.load("img/exit.png");

        cmds.spawn((
            DespawnOnExit(MenuState::Main),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NAVAJO_WHITE.into()),
                children![
                    (
                        Text::new("2D Horror Game Main Menu"),
                        TextFont {
                            font_size: 68.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Play,
                        children![
                            (ImageNode::new(right_icon), button_icon_node.clone()),
                            (
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Settings,
                        children![
                            (ImageNode::new(wrench_icon), button_icon_node.clone()),
                            (
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (ImageNode::new(exit_icon), button_icon_node),
                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                        ]
                    )
                ]
            )],
        ));
    }

    fn settings_menu_setup(mut cmds: Commands) {
        let button_node = Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        cmds.spawn((
            DespawnOnExit(MenuState::Settings),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NAVAJO_WHITE.into()),
                Children::spawn(SpawnIter(
                    [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ]
                    .into_iter()
                    .map(move |(action, text)| {
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                            children![(Text::new(text), button_text_style.clone())],
                        )
                    })
                ))
            )],
        ));
    }

    fn display_settings_menu_setup(mut cmds: Commands, display_quality: Res<DisplayQuality>) {
        fn button_node() -> Node {
            Node {
                width: px(200),
                height: px(65),
                margin: UiRect::all(px(20)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }
        }
        fn button_text_style() -> impl Bundle {
            (
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            )
        }

        let display_quality = *display_quality;
        cmds.spawn((
            DespawnOnExit(MenuState::SettingsDisplay),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnDisplaySettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NAVAJO_WHITE.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(NAVAJO_WHITE.into()),
                        Children::spawn((
                            Spawn((Text::new("Display Quality"), button_text_style())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for quality_setting in [
                                    DisplayQuality::Low,
                                    DisplayQuality::Medium,
                                    DisplayQuality::High,
                                ] {
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: px(150),
                                            height: px(65),
                                            ..button_node()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        Setting(quality_setting),
                                        children![(
                                            Text::new(format!("{quality_setting:?}")),
                                            button_text_style(),
                                        )],
                                    ));
                                    if display_quality == quality_setting {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), button_text_style())]
                    )
                ]
            )],
        ));
    }

    fn sound_settings_menu_setup(mut cmds: Commands, volume: Res<Volume>) {
        let button_node = Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        let volume = *volume;
        let button_node_clone = button_node.clone();
        cmds.spawn((
            DespawnOnExit(MenuState::SettingsSound),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSoundSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NAVAJO_WHITE.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(NAVAJO_WHITE.into()),
                        Children::spawn((
                            Spawn((Text::new("Volume"), button_text_style.clone())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                (0..=9)
                                    .collect::<Vec<u32>>()
                                    .iter()
                                    .for_each(|volume_setting| {
                                        let mut entity = parent.spawn((
                                            Button,
                                            Node {
                                                width: px(30),
                                                height: px(65),
                                                ..button_node_clone.clone()
                                            },
                                            BackgroundColor(NORMAL_BUTTON),
                                            Setting(Volume(*volume_setting)),
                                        ));
                                        if volume == Volume(*volume_setting) {
                                            entity.insert(SelectedOption);
                                        }
                                    });
                            })
                        ))
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), button_text_style)]
                    )
                ]
            )],
        ));
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => menu_state.set(MenuState::SettingsDisplay),
                    MenuButtonAction::SettingsSound => menu_state.set(MenuState::SettingsSound),
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => menu_state.set(MenuState::Settings),
                }
            }
        }
    }
}
