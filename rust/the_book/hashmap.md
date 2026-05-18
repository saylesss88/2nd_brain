## Rustlings HashMap problem 3

```rust
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
        //
        let team_1 = scores.entry(team_1_name).or_insert(TeamScores::default());
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name).or_insert(TeamScores::default());
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
```

**What the HashMap holds**

```rust
HashMap<&str, TeamScores>
```

- If we just did `scores.insert(team_1_name, ...)` every line, England would
get overwritten each match. You'd lose it's previous goals. You need to _find_
the existing entry and _add_ to it.

**What `entry().or_insert()` does**

This is the "get it or create it" pattern:

- If the team **already exists** in the map → give me a mutable reference to it
- If the team **doesn't exist yet** → insert a fresh `TeamScores::default()` (zeros), then give me a mutable reference to that

Either way you get back a `&mut TeamScores` you can modify.

**Why `+=` and not `=`**

Because you're **accumulating** across multiple matches. England plays 3 times -
you need to add each match's goals on top of what's already there.

