use itertools::Itertools;

// TODO: use bitvec instead of &[bool]

struct PacketEval {
    version_sum: usize,
    value: usize,
    length: usize,
}

fn to_binary_array(byte: u8) -> [bool; 4] {
    let value = match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        _ => panic!(),
    };

    [
        value & 8 != 0,
        value & 4 != 0,
        value & 2 != 0,
        value & 1 != 0,
    ]
}

fn to_value<'a>(binary_sequence: impl IntoIterator<Item = &'a bool>) -> usize {
    binary_sequence
        .into_iter()
        .fold(0, |acc, bit| acc * 2 + usize::from(*bit))
}

fn type_4_body(packet: &[bool]) -> (usize, usize) {
    let mut len = 0;
    let value = to_value(
        packet
            .chunks(5)
            .take_while_inclusive(|group| {
                len += 5;
                group[0]
            })
            .flat_map(|group| &group[1..]),
    );
    (value, len)
}

// TODO: gross amounts of indends?
fn evaluate_packet(packet: &[bool]) -> PacketEval {
    let version = to_value(&packet[0..3]);
    let id = to_value(&packet[3..6]);
    match id {
        4 => {
            let (value, body_length) = type_4_body(&packet[6..]);
            PacketEval {
                version_sum: version,
                value,
                length: body_length + 6,
            }
        }
        0..=3 | 5..=7 => {
            let mut sub_packets = Vec::new();
            let mut length;
            // evaluate sub packets
            if packet[6] {
                length = 18;
                let remaining_packets = to_value(&packet[7..length]);
                for _ in 0..remaining_packets {
                    let sub_packet = evaluate_packet(&packet[length..]);
                    length += sub_packet.length;
                    sub_packets.push(sub_packet);
                }
            } else {
                length = 22;
                let final_length = to_value(&packet[7..length]) + length;
                while length < final_length {
                    let sub_packet = evaluate_packet(&packet[length..]);
                    length += sub_packet.length;
                    sub_packets.push(sub_packet);
                }
                debug_assert_eq!(length, final_length);
            }
            // evaluate current packet
            let version_sum = version
                + sub_packets
                    .iter()
                    .map(|packet| packet.version_sum)
                    .sum::<usize>();
            let value = match id {
                0 => sub_packets.iter().map(|packet| packet.value).sum(),
                1 => sub_packets.iter().map(|packet| packet.value).product(),
                2 => sub_packets.iter().map(|packet| packet.value).min().unwrap(),
                3 => sub_packets.iter().map(|packet| packet.value).max().unwrap(),
                5 => {
                    debug_assert_eq!(sub_packets.len(), 2);
                    usize::from(sub_packets[0].value > sub_packets[1].value)
                }
                6 => {
                    debug_assert_eq!(sub_packets.len(), 2);
                    usize::from(sub_packets[0].value < sub_packets[1].value)
                }
                7 => {
                    debug_assert_eq!(sub_packets.len(), 2);
                    usize::from(sub_packets[0].value == sub_packets[1].value)
                }
                _ => unreachable!(),
            };
            PacketEval {
                version_sum,
                value,
                length,
            }
        }
        _ => panic!(),
    }
}

pub fn task1(input: String) -> String {
    let binary_packet: Box<[bool]> = input.bytes().flat_map(to_binary_array).collect();
    evaluate_packet(&binary_packet).version_sum.to_string()
}

pub fn task2(input: String) -> String {
    let binary_packet: Box<[bool]> = input.bytes().flat_map(to_binary_array).collect();
    evaluate_packet(&binary_packet).value.to_string()
}
