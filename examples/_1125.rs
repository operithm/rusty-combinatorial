//! LeetCode 1125 - Smallest Sufficient Team
//!
//! Technique: Bitmask Dynamic Programming (Set Cover)
//!
//! Each skill is mapped to a bit. Each person is a bitmask of skills.
//! DP iterates over subsets of skills and builds the smallest team.

use std::collections::HashMap;

pub fn smallest_sufficient_team(
    req_skills: Vec<String>,
    people: Vec<Vec<String>>,
) -> Vec<i32> {
    let n = req_skills.len();

    // Map skill -> bit index
    let mut skill_id = HashMap::new();
    for (i, s) in req_skills.iter().enumerate() {
        skill_id.insert(s.clone(), i);
    }

    // Encode each person as a bitmask
    let people_mask: Vec<u32> = people
        .iter()
        .map(|skills| {
            let mut mask = 0;
            for s in skills {
                if let Some(&i) = skill_id.get(s) {
                    mask |= 1 << i;
                }
            }
            mask
        })
        .collect();

    let full = (1u32 << n) - 1;

    // dp[mask] = smallest team covering exactly `mask`
    let mut dp: Vec<Option<Vec<i32>>> = vec![None; 1 << n];
    dp[0] = Some(vec![]);

    for (i, &pm) in people_mask.iter().enumerate() {
        for mask in (0..=full as usize).rev() {
            if let Some(team) = dp[mask].clone() {
                let new_mask = mask as u32 | pm;
                let mut new_team = team.clone();
                new_team.push(i as i32);

                let entry = &mut dp[new_mask as usize];
                if entry.is_none() || entry.as_ref().unwrap().len() > new_team.len() {
                    *entry = Some(new_team);
                }
            }
        }
    }

    dp[full as usize].clone().unwrap()
}

fn main() {
    let skills = vec![
        "java".to_string(),
        "nodejs".to_string(),
        "reactjs".to_string(),
    ];

    let people = vec![
        vec!["java".to_string()],
        vec!["nodejs".to_string()],
        vec!["nodejs".to_string(), "reactjs".to_string()],
    ];

    let team = smallest_sufficient_team(skills, people);
    println!("Smallest sufficient team: {:?}", team);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let skills = vec![
            "java".to_string(),
            "nodejs".to_string(),
            "reactjs".to_string(),
        ];

        let people = vec![
            vec!["java".to_string()],
            vec!["nodejs".to_string()],
            vec!["nodejs".to_string(), "reactjs".to_string()],
        ];

        let team = smallest_sufficient_team(skills, people);
        assert_eq!(team.len(), 2);
    }
}
