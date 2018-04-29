alphabet: 0, 1;
accept: qY;
reject: qN;

q0 {
    * -> q0 * >
    _ -> q1 _ <
}

q1 {
    0 -> q2 _ <
    1 -> q3 _ <
    _ -> qN _ <
}

q2 {
    0 -> qY _ <
    * -> qN _ <
}

q3 * -> qN _ <