use bevy::prelude::Event;


#[derive(Event)]
pub struct BoardMoveStart;

#[derive(Event)]
pub struct BoardMoveEnd;

#[derive(Event)]
pub struct GameOverEvent;
