use crate::datamodel::quests::quest_new::*;

#[test]
#[should_panic(expected = "Winner's rule cannot be bigger than quest member number")]
fn test_quest_panics_on_winner_rule_bigger_than_quest_member_num() {
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let winner_rule = (Vote::Success, quest_members.len()+1);
    let _quest = QuestNew::new(&quest_members, &winner_rule);
 }

#[test]
fn test_quest_successes_on_full_crew_voting_success_with_full_success_rule() {
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Success, Vote::Success];
    let winner_rule = (Vote::Success, quest_members.len());
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_fails_with_full_success_rule() { //failed
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Success, Vote::Failed];
    let winner_rule = (Vote::Success, quest_members.len());
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_fails_with_success_rule() { //failed
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Failed, Vote::Failed];
    let winner_rule = (Vote::Success, quest_members.len()-1);
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_successes_with_success_rule() { //failed
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Success, Vote::Failed];
    let winner_rule = (Vote::Success, quest_members.len()-1);
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_successes_on_full_crew_voting_success_with_failed_rule() {
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Success, Vote::Success];
    let winner_rule = (Vote::Failed, 0);
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_fails_on_full_crew_voting_success_with_failed_rule() {
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Failed, Vote::Success];
    let winner_rule = (Vote::Failed, 0);
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Failed);
}
