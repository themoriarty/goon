// allow non-snake case since we're directly mapping nhl's fields
#![allow(non_snake_case)]

use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
enum GameState {
  Final
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct GameStatus {
  abstractGameState : GameState,
  detailedState : GameState,
}


#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Division {
  id: i64,
  name: String,
  link: String
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Team {
  id: i64,
  name: String,
  link: String,
  division: Division,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Teams {
  away: Team,
  home: Team,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Player {
  id: i64,
  fullName: String,
  link: String,
  currentAge: i16,

}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct GameData {
  status: GameStatus,
  teams: Teams,
  players: HashMap<String, Player>,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct TeamInfo {
  id: i64,
  name: String,
  link: String,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct TeamLineScore {
  team: TeamInfo,
  goals: i16,
  shotsOnGoal: i16,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct TeamLineScores {
  home: TeamLineScore,
  away: TeamLineScore,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct LineScore {
  currentPeriod: i16,
  teams: TeamLineScores,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct LiveData {
  linescore: LineScore,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct LiveFeed {
  link: String,
  gameData: GameData,
  liveData: LiveData,
}
