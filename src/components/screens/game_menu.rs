//! This module generates the graphical user interfaces of both
//! the main menu as well as the pause menu.

use crate::FpsPlugin;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.0, 0.28, 0.73);

/// This enum symbolizes the current game state which is then
/// handled accordingly.
///
/// # Examples
///
/// ```
/// use slots_from_hell::components::screens::game_menu::GameState;
/// let game_state = &GameState::Splash;
///
/// match game_state {
///     GameState::Splash => println!("Splash"),
///     GameState::Menu => println!("Menu"),
///     GameState::Game => println!("Game"),
///     GameState::Playing => println!("Playing"),
///     GameState::Pause => println!("Pause"),
/// }
/// ```
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    /// Default `start screen` game state.
    Splash,
    /// Currently in the `main menu`.
    Menu,
    /// `Game` screen.
    Game,
    /// Currently `playing`.
    Playing,
    /// Actively `paused the game`.
    Pause,
}

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource)]
struct Setting<T>(pub T);

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
struct Volume(pub u32);

/// Structural representation of the current state of the player,
/// whether they are in game or not.
///
/// # Examples
///
/// ```
/// use slots_from_hell::components::screens::game_menu::InGame;
///
/// let in_game = InGame(true);
///
/// if in_game.0 {
///     println!("Player is in game.");
/// }
/// ```
#[derive(Resource)]
pub struct InGame(
    /// Boolean to check whether the `Player` is in game or not.
    pub bool,
);

/// The plugin for the game menu, having all relevant resources
/// and systems (`Volume`, `DisplayQuality`, [`InGame`],
/// [`GameState`], `splash_plugin`,
/// [`menu::menu_plugin`], `setup`).
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::screens::game_menu::GameMenuPlugin;
///
/// App::new().add_plugins((DefaultPlugins, GameMenuPlugin));
/// ```
#[derive(Debug)]
pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium)
            .insert_resource(Volume(7))
            .insert_resource(InGame(false))
            .init_state::<GameState>()
            .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn((Camera2d, Msaa::Off));
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
                            font_size: FontSize::Px(68.0),
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
                                    font_size: FontSize::Px(50.0),
                                    ..default()
                                },
                                TextColor(NAVY.into()),
                            ),
                            (
                                TextSpan::new(" - "),
                                TextFont {
                                    font_size: FontSize::Px(50.0),
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ),
                            (
                                TextSpan(format!("Volume: {:?}", *volume)),
                                TextFont {
                                    font_size: FontSize::Px(50.0),
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

/// This module is for the menu (main menu as well as pause menu).
pub mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::NAVAJO_WHITE,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };

    use super::{DisplayQuality, FpsPlugin, GameState, InGame, Setting, TEXT_COLOR, Volume};

    /// Function for generating the game's menu.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use slots_from_hell::components::screens::game_menu::menu::menu_plugin;
    ///
    /// App::new().add_plugins((DefaultPlugins, menu_plugin));
    ///
    /// ```
    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(FpsPlugin)
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Pause), pause_menu_setup)
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
                (menu_action, button_system).run_if(in_state(GameState::Pause)),
            );
    }

    /// This enum represents the current menu state.
    ///
    /// # Examples
    ///
    /// ```
    /// use slots_from_hell::components::screens::game_menu::menu::MenuState;
    ///
    /// let menu_state = MenuState::Main;
    ///
    /// match menu_state {
    ///     MenuState::Main => println!("Main menu."),
    ///     MenuState::Pause => println!("Pause menu"),
    ///     MenuState::Settings => println!("Settings menu."),
    ///     MenuState::SettingsDisplay => println!("Settings display menu."),
    ///     MenuState::SettingsSound => println!("Settings sound menu."),
    ///     MenuState::Disabled => println!("Disabled all menus."),
    /// }
    /// ```
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    pub enum MenuState {
        /// Main menu.
        Main,
        /// Pause menu.
        Pause,
        /// Settings menu.
        Settings,
        /// Settings display menu.
        SettingsDisplay,
        /// Settings sound menu.
        SettingsSound,
        #[default]
        /// No menu: default state.
        Disabled,
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnPauseScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct OnDisplaySettingsMenuScreen;

    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    #[derive(Component)]
    struct SelectedOption;

    const NORMAL_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
    const HOVERED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.5, 0.65, 0.5);
    const PRESSED_BUTTON: Color = Color::srgb(0.65, 0.75, 0.65);

    /// The actions the menu buttons do.
    ///
    /// # Examples
    ///
    /// ```
    /// use slots_from_hell::components::screens::game_menu::menu::MenuButtonAction;
    ///
    /// let menu_button_action = MenuButtonAction::Play;
    ///
    /// match menu_button_action {
    ///     MenuButtonAction::Play => println!("Play."),
    ///     MenuButtonAction::Continue => println!("Continue."),
    ///     MenuButtonAction::Settings => println!("Settings."),
    ///     MenuButtonAction::SettingsDisplay => println!("Settings display."),
    ///     MenuButtonAction::SettingsSound => println!("Settings sound."),
    ///     MenuButtonAction::BackToMainMenu => println!("Back to main menu."),
    ///     MenuButtonAction::BackToSettings => println!("Back to settings."),
    ///     MenuButtonAction::Quit => println!("Quit."),
    /// }
    /// ```
    #[derive(Component)]
    pub enum MenuButtonAction {
        /// `Play` the game.
        Play,
        /// `Continue` the game.
        Continue,
        /// Go into the `settings` menu.
        Settings,
        /// Go into the `settings display options` menu.
        SettingsDisplay,
        /// Go into the `settings sound options` menu.
        SettingsSound,
        /// Go from the `settings menu` back to the `main menu`.
        BackToMainMenu,
        /// Go from the `settings display` or `settings sound`
        /// options back to the `settings menu`.
        BackToSettings,
        /// `Quit` the game.
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

    use bevy::ecs::component::Mutable;

    fn setting_button<T: Resource + Component<Mutability = Mutable> + PartialEq + Copy>(
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

    fn generate_buttons_and_font() -> (Node, Node, TextFont) {
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
            font_size: FontSize::Px(33.0),
            ..default()
        };

        (button_node, button_icon_node, button_text_font)
    }

    fn generate_icons(
        asset_server: Res<AssetServer>,
    ) -> (Handle<Image>, Handle<Image>, Handle<Image>) {
        (
            asset_server.load("img/right.png"),
            asset_server.load("img/wrench.png"),
            asset_server.load("img/exit.png"),
        )
    }

    fn main_menu_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let generator = generate_buttons_and_font();
        let icons = generate_icons(asset_server);

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
                            font_size: FontSize::Px(68.0),
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
                        generator.0.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Play,
                        children![
                            (ImageNode::new(icons.0), generator.1.clone()),
                            (
                                Text::new("New Game"),
                                generator.2.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        generator.0.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Settings,
                        children![
                            (ImageNode::new(icons.1), generator.1.clone()),
                            (
                                Text::new("Settings"),
                                generator.2.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        generator.0,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (ImageNode::new(icons.2), generator.1),
                            (Text::new("Quit"), generator.2, TextColor(TEXT_COLOR),),
                        ]
                    )
                ]
            )],
        ));
    }

    fn pause_menu_setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let generator = generate_buttons_and_font();
        let icons = generate_icons(asset_server);

        cmds.spawn((
            DespawnOnExit(MenuState::Pause),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnPauseScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NAVAJO_WHITE.into()),
                children![
                    (
                        Text::new("Paused"),
                        TextFont {
                            font_size: FontSize::Px(68.0),
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
                        generator.0.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Continue,
                        children![
                            (ImageNode::new(icons.0), generator.1.clone()),
                            (
                                Text::new("Continue"),
                                generator.2.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        generator.0.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Settings,
                        children![
                            (ImageNode::new(icons.1), generator.1.clone()),
                            (
                                Text::new("Settings"),
                                generator.2.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        generator.0,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (ImageNode::new(icons.2), generator.1),
                            (Text::new("Quit"), generator.2, TextColor(TEXT_COLOR),),
                        ]
                    )
                ]
            )],
        ));
    }

    fn settings_menu_setup(mut cmds: Commands) {
        let generator = generate_buttons_and_font();

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
                            generator.0.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                            children![(Text::new(text), generator.2.clone())],
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
                    font_size: FontSize::Px(33.0),
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
                font_size: FontSize::Px(33.0),
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
                    MenuButtonAction::Continue => {
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
