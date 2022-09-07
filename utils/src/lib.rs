#[derive(Debug, Copy, Clone)]
enum State {
    Ongoing(usize),
    Finished(usize),
    Empty,
}

pub const fn assert_no_intersection<const N: usize>(msgs: [&[&str]; N]) {
    let mut states = init_states(&msgs);

    while !should_end(&states) {
        // Pivot index
        let index = get_index_of_alphabetically_smallest(&msgs, &states);

        // compare all elements at current index
        verify_no_collissions(&msgs, &states, &index);

        // increment index of alaphabeticaly first element
        states[index] = match states[index] {
            State::Ongoing(wi) => {
                if msgs[index].len() == wi + 1 {
                    State::Finished(wi)
                } else {
                    State::Ongoing(wi + 1)
                }
            }
            _ => panic!("This should never be reached!"),
        };
    }
}

const fn init_states<const N: usize>(msgs: &[&[&str]; N]) -> [State; N] {
    let mut states = [State::Ongoing(0); N];
    let mut i = 0;
    while i < N {
        if msgs[i].is_empty() {
            states[i] = State::Empty;
        }
        i += 1;
    }
    states
}

const fn get_index_of_alphabetically_smallest<const N: usize>(
    msgs: &[&[&str]; N],
    states: &[State; N],
) -> usize {
    let mut i = 1;
    let mut output_index = 0;
    while i < N {
        match states[i] {
            State::Ongoing(outer_i) => match states[output_index] {
                State::Ongoing(inner_i) => {
                    if let std::cmp::Ordering::Greater =
                        konst::cmp_str(msgs[output_index][inner_i], msgs[i][outer_i])
                    {
                        output_index = i
                    }
                }
                _ => output_index = i,
            },
            _ => continue,
        }

        i += 1;
    }
    output_index
}

const fn verify_no_collissions<const N: usize>(
    msgs: &[&[&str]; N],
    states: &[State; N],
    index: &usize,
) {
    let mut i = 0;
    while i < N {
        if i == *index {
            i += 1;
            continue;
        }
        match states[i] {
            State::Ongoing(outer_i) | State::Finished(outer_i) => match states[*index] {
                State::Ongoing(inner_i) | State::Finished(inner_i) => {
                    if konst::eq_str(msgs[i][outer_i], msgs[*index][inner_i]) {
                        panic!("Message overlaps between interface and contract impl!");
                    }
                }
                _ => (),
            },
            _ => (),
        }
        i += 1;
    }
}

const fn should_end<const N: usize>(states: &[State; N]) -> bool {
    let mut i = 0;
    while i < N {
        if let State::Ongoing(..) = states[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
