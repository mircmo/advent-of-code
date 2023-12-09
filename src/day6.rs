pub(crate) fn first() -> usize {
    // let games = vec![(7, 9), (15, 40), (30, 200)];
    let games = vec![(46, 208), (85, 1412), (75, 1257), (82, 1410)];

    games
        .into_iter()
        .map(|(duration, record)| {
            (1..duration)
                .into_iter()
                .filter(|button_press_duration| {
                    let rest = duration - button_press_duration;
                    let distance = rest * button_press_duration;
                    return distance > record;
                })
                .count()
        })
        .product()
}
