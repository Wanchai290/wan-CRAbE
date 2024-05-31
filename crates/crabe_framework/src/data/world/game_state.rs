use crate::data::world::TeamColor;
use serde::Serialize;

/// Defines the possible game states of the match
#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameState {
    Halted(HaltedState),
    Stopped(StoppedState),
    Running(RunningState),
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HaltedState {
    /// The game hasn't started yet
    GameNotStarted,
    /// A halt command has been issued
    Halt,
    /// A team is having a timeout
    Timeout,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StoppedState {
    /// The team `TeamColor` is preparing to do their kickoff
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    /// The team `TeamColor` is trying to place the ball automatically
    /// without the help of a human to pursue the game
    BallPlacement(TeamColor),
    /// Generic stop command, issued when robots must slow down after
    /// a foul, for example. Can be issued manually
    Stop,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RunningState {
    /// The team `TeamColor` is doing their kickoff
    /// Everyone can move, but only `TeamColor` is allowed
    /// to perform the first ball touch
    KickOff(TeamColor),
    /// The team `TeamColor` has a robot ready to score a penalty
    /// towards the goalkeeper of the enemy team
    Penalty(TeamColor),
    /// The team `TeamColor` can freely kick the ball once
    FreeKick(TeamColor),
    /// Generic running command, when no special event has occurred
    /// Can be issued manually
    Run,
}