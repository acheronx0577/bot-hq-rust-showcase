//! Offline demonstration of fair admission to a shared bot computer.
use std::collections::VecDeque;

struct Owner<'a> {
    name: &'a str,
    waiting: VecDeque<&'a str>,
}

// One job finishes per turn. Owners with waiting jobs rotate in FIFO order.
fn schedule<'a>(owners: impl IntoIterator<Item = Owner<'a>>) -> Vec<(&'a str, &'a str)> {
    let mut ready: VecDeque<_> = owners.into_iter().collect();
    let mut turns = Vec::new();
    while let Some(mut owner) = ready.pop_front() {
        if let Some(job) = owner.waiting.pop_front() {
            turns.push((owner.name, job));
            if !owner.waiting.is_empty() {
                ready.push_back(owner);
            }
        }
    }
    turns
}

fn main() {
    let turns = schedule([
        Owner {
            name: "Alex",
            waiting: VecDeque::from(["read notes", "summarize notes", "save draft"]),
        },
        Owner {
            name: "Sam",
            waiting: VecDeque::from(["open report", "review report"]),
        },
    ]);
    println!("One computer slot shared by two fictional owners");
    for (index, (owner, job)) in turns.iter().enumerate() {
        println!("{}. {owner} — {job}", index + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_empty_owners_and_preserves_each_owners_job_order() {
        let turns = schedule([
            Owner {
                name: "A",
                waiting: VecDeque::from(["A1", "A2", "A3"]),
            },
            Owner {
                name: "B",
                waiting: VecDeque::new(),
            },
            Owner {
                name: "C",
                waiting: VecDeque::from(["C1", "C2"]),
            },
        ]);
        assert_eq!(
            turns,
            [
                ("A", "A1"),
                ("C", "C1"),
                ("A", "A2"),
                ("C", "C2"),
                ("A", "A3")
            ]
        );
    }
}
