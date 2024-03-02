use bevy::prelude::*;
use log::info;

use crate::components::{
    entity::{PartyList, PartyMember},
    ui::party::PartyListInterface,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn setup_party_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    party: Query<&PartyMember>,
) {
    commands.spawn((
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
        PartyList {
            list: Vec::new(),
            active: None,
        },
    ));
}

pub fn update_party_ui_members(
    mut party_list_interface: Query<
        (&PartyList, Entity),
        (Changed<PartyList>, With<PartyListInterface>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((party_list, entity)) = party_list_interface.get_single_mut() {
        info!("single partylist updated");
        for y in &party_list.list {
            info!("Member: {:?}", y);
        }

        let buttons: Vec<Entity> = party_list
            .list
            .iter()
            .map(|party_member| {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
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
            .collect();

        commands.entity(entity).clear_children();
        commands.entity(entity).push_children(&buttons);
    }
}

pub fn party_ui(mut query: Query<&mut PartyList>) {}
