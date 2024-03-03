use bevy::prelude::*;
use log::info;

use crate::components::ui::party::PartyMemberButton;
use crate::components::{
    entity::{PartyList, PartyMember},
    ui::party::PartyListInterface,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const ACTIVE_BUTTON: Color = Color::rgb(0.95, 0.15, 0.15);

fn entities_to_buttons(
    commands: &mut Commands,
    party_list: &PartyList,
    asset_server: Res<AssetServer>,
) -> Vec<Entity> {
    party_list
        .list
        .iter()
        .map(|party_member| {
            let is_active_button = party_list
                .active
                .is_some_and(|active| active == *party_member);
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: if is_active_button {
                            ACTIVE_BUTTON.into()
                        } else {
                            NORMAL_BUTTON.into()
                        },
                        ..default()
                    },
                    PartyMemberButton(*party_member),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "X",
                        TextStyle {
                            font: asset_server.load("fonts/Vollkorn-VariableFont_wght.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .id()
        })
        .collect()
}

pub fn setup_party_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    party: Query<Entity, With<PartyMember>>,
) {
    let party_list = PartyList {
        list: party.iter().collect(),
        active: party.iter().last(),
    };
    let buttons = entities_to_buttons(&mut commands, &party_list, asset_server);
    let parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            PartyListInterface,
            party_list,
        ))
        .id();
    commands.entity(parent).push_children(&buttons);
}

pub fn update_party_ui_members(
    mut party_list_interface: Query<
        (&PartyList, &Children, Entity),
        (Changed<PartyList>, With<PartyListInterface>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((party_list, children, entity)) = party_list_interface.get_single_mut() {
        info!("single partylist updated");
        for y in &party_list.list {
            info!("Member: {:?}", y);
        }
        commands.entity(entity).clear_children();
        for child in children.iter() {
            commands.entity(*child).despawn();
        }
        let buttons = entities_to_buttons(&mut commands, party_list, asset_server);
        commands.entity(entity).push_children(&buttons);
    }
}

pub fn party_ui(
    mut interaction: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &PartyMemberButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut party_lists: Query<&mut PartyList, Without<PartyListInterface>>,
    mut party_list_interface: Query<&mut PartyList, With<PartyListInterface>>,
) {
    for (interaction, mut color, mut border_color, children, party_member) in &mut interaction {
        let is_active_button = party_list_interface
            .single()
            .active
            .is_some_and(|active| active == party_member.0);
        match *interaction {
            Interaction::Pressed => {
                for mut list in &mut party_lists {
                    list.active = Some(party_member.0);
                }
                party_list_interface.single_mut().active = Some(party_member.0);
                *color = ACTIVE_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = ACTIVE_BUTTON.into();
                border_color.0 = Color::WHITE
            }
            Interaction::None => {
                *color = if is_active_button {
                    ACTIVE_BUTTON.into()
                } else {
                    NORMAL_BUTTON.into()
                };
                border_color.0 = Color::BLACK
            }
        }
    }
}
